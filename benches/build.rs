fn main() {
    println!("cargo:rustc-link-lib=dylib=radix_engine_toolkit");
    println!("cargo:rustc-link-search=native=./lib");
}
