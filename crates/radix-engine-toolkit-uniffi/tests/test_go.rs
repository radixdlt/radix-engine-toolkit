use std::{path::PathBuf, process::*};

#[test]
fn uniffi_foreign_language_testcase_example_go() {
    let min_supported_go_version = (1, 21); // version 1.21.x

    // Check if Golang and uniffi-bindgen-go are installed
    let output = Command::new("go")
        .arg("version")
        .output()
        .expect("go version command failed, Golang not installed?");
    let version = String::from_utf8_lossy(&output.stdout)
        .strip_prefix("go version go")
        .expect("Malformed go version command output")
        .split(' ')
        .collect::<Vec<_>>()
        .first()
        .expect("Malformed go version command output")
        .split('.')
        .map(|a| a.parse::<usize>().expect("Not supported go version"))
        .collect::<Vec<usize>>();
    assert!(
        version.len() == 3
            && version[0] >= min_supported_go_version.0
            && version[1] >= min_supported_go_version.1,
        "Not supported go version: {}.{}, minimum is {}.{}",
        version[0],
        version[1],
        min_supported_go_version.0,
        min_supported_go_version.1
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

    // Run go tests
    let output = Command::new("go")
        .args(["test", "-v"])
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
        "go test command failed:\n{}",
        String::from_utf8_lossy(&output.stdout)
    );
}
