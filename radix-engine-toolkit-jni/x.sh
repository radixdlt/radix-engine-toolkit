# export CPPFLAGS="-I/Users/omarabdulla/Downloads/android-ndk-r22b/toolchains/llvm/prebuilt/darwin-x86_64/include/c++/4.9.x"
# export LDFLAGS="-L/Users/omarabdulla/Downloads/android-ndk-r22b/toolchains/llvm/prebuilt/darwin-x86_64/lib/lib64 -Wl,-rpath,/Users/omarabdulla/Downloads/android-ndk-r22b/toolchains/llvm/prebuilt/darwin-x86_64/lib/lib64"
export CC="/Users/omarabdulla/Downloads/android-ndk-r22b/toolchains/llvm/prebuilt/darwin-x86_64/bin/aarch64-linux-android21-clang"

cargo build --release --target aarch64-linux-android