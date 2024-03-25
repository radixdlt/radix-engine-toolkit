use std::{path::PathBuf, process::*};

#[test]
fn test_go_binding() {
    // Check if Golang and uniffi-bindgen-go are installed
    assert!(
        Command::new("go")
            .arg("version")
            .stdout(Stdio::null())
            .status()
            .is_ok(),
        "Golang not installed"
    );
    assert!(
        Command::new("uniffi-bindgen-go")
            .arg("--version")
            .stdout(Stdio::null())
            .status()
            .is_ok(),
        "Uniffi-bindgen-go not installed"
    );

    // Build radix-engine-toolkit-uniffi to generate required libraries
    assert!(Command::new("cargo").arg("build").status().is_ok());

    let mut udl_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    udl_path.push("src");
    udl_path.push("radix_engine_toolkit_uniffi.udl");
    assert!(
        udl_path.exists(),
        "UDL file not found: {}",
        udl_path.display()
    );

    let mut target_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    target_path.pop(); // project dir
    target_path.pop(); // crates dir
    target_path.push("target");
    target_path.push("debug");
    target_path.push("libradix_engine_toolkit_uniffi");
    #[cfg(target_os = "macos")]
    target_path.set_extension("dylib");
    #[cfg(target_os = "linux")]
    target_path.set_extension("so");
    #[cfg(target_os = "windows")]
    target_path.set_extension("dll");
    assert!(
        target_path.exists(),
        "Lib file not found: {}",
        target_path.display()
    );

    let mut out_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    out_dir.push("output");

    // Generate go binding module
    assert!(
        Command::new("uniffi-bindgen-go")
            .args([
                udl_path.display().to_string().as_str(),
                "--lib-file",
                target_path.display().to_string().as_str(),
                "--out-dir",
                out_dir.display().to_string().as_str()
            ])
            .status()
            .is_ok(),
        "uniffi-bindgen-go command failed"
    );

    let go_package_name = "radix_engine_toolkit_uniffi.go";
    let mut go_package_dir = out_dir.clone();
    go_package_dir.push("radix_engine_toolkit_uniffi");

    let mut go_package = go_package_dir.clone();
    go_package.push(go_package_name);
    assert!(
        go_package.exists(),
        "File not found: {}",
        go_package.display()
    );
    assert!(
        go_package.with_extension("c").exists(),
        "File not found: {}",
        go_package.with_extension("c").display()
    );
    assert!(
        go_package.with_extension("h").exists(),
        "File not found: {}",
        go_package.with_extension("h").display()
    );

    // Verify if generated go package can be installed
    let output = Command::new("go")
        .current_dir(go_package_dir)
        .args(["install", go_package_name])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "go install package failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let mut go_src = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    go_src.push("tests");
    go_src.push("bindings");

    let mut target_dir = target_path.clone();
    target_dir.pop(); // lib name

    let mut target_go_path = target_dir.clone();
    target_go_path.push("go");
    target_go_path.push("test_go");

    // Compile go test program
    let output = Command::new("go")
        .args([
            "build",
            "-o",
            target_go_path.display().to_string().as_str(),
            "example.go",
        ])
        .current_dir(go_src)
        .envs([
            ("GO111MODULE", "auto"),
            (
                "CGO_LDFLAGS", // specify lib search path and lib name
                &format!(
                    "-L{} -lradix_engine_toolkit_uniffi",
                    target_dir.display().to_string()
                ),
            ),
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "go program compilation failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    assert!(
        target_go_path.exists(),
        "test_go program does not exists: {}",
        target_go_path.display().to_string()
    );

    // Run test program
    let output = Command::new(target_go_path).output().unwrap();
    assert!(
        output.status.success(),
        "test_go program failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}
