# Radix Engine Toolkit

The Radix Engine Toolkit is a multiplatform library written in Rust that exposes a set of functions to help clients written in languages other than Rust compile and decompile transactions, perform SBOR encoding and decoding, derive virtual account and virtual badge addresses, and other functionalities.

## Background

Babylon transactions are composed of a number of different parts, but their building blocks essentially boil down to: a transaction header, a manifest, and signatures which might either be intent signatures or a notary signature depending on what exactly was signed. The diagram below illustrates the relationship and names of the different parts of a transaction.

![image](./images/v0.5.0-transaction-model.png)

When we have a transaction intent, getting to a signed transaction intent requires that the transaction intent is "compiled" and signed. In this context, compiling something simply refers to encoding it in SBOR. Once all of the signers have signed the compiled transaction intent, then we have all we need to for a signed transaction intent.

Similarly, when the notary wishes to notarize a signed transaction intent, they compile the signed transaction intent (as explained above, they encode the signed transaction intent in SBOR), sign that, and with that they have the notary signature required to form a complete transaction.

## Motivation

As can be seen in the [background section](#background), the process of creating a transaction requires that a client is able to encode different parts of a transaction in SBOR to later allow for these parts to be signed or notarized. This means that a client would need to, at a minimum, have an SBOR encoder for compiling transactions, and perhaps a decoder if the decompilation of transactions is desirable.

The main implementation of SBOR is written in Rust. That being said, clients wishing to integrate their services with Radix (or just build and send programmatic transactions) could have clients written in a any programming languages. It is unrealistic to expect that clients would write, test, and maintain their own SBOR implementation in their client language as this is a high implementation burden on the client and is a non-trivial and potentially security critical task.

In addition to the building of transactions requiring SBOR for compilation, certain clients might wish to decompile transactions to figure out what the transaction does, who has signed it, contents of the header, or for other application-specific reasons. Much like compilation, without an SBOR implementation available to the client, the decompilation of transactions would be impossible.

Therefore, there is a strong need to **not** rewrite SBOR in different languages, instead, to reuse and re-utilize the original SBOR implementation for clients written in any language. This library achieves that by being fully written in rust; thus leveraging the ability to use the original SBOR implementation, and being compilable to different targets to allow it to run on different operating systems with different processor architectures. In addition to that, this library may be compiled to WASM which means that languages with easy access to a WASM runtime can utilize this library. As an example, the Radix Engine Toolkit may run in the web by utilizing web assembly.

The choice of making the radix engine toolkit a library and not an REST API comes with a number of advantages, the first of which is that certain clients may wish to build their transactions, SBOR encode and decode their data, or Bech32 encode or decode their addresses in a trustless fashion without having to rely on a REST API where a non-negligible chance of tampering exists. Having this library as a library allows such clients to do all they need in a trustless manner without worrying about trusting an external server.

In addition to that, using WASM instead of a docker-based solution to this problem means that the solution is very lightweight and also compatible with platforms where Docker is no an option (mostly smart phones.), thus making this library available to the largest amount of clients without any compromises.

With the above in mind, this library allows for the following functionality to be performed in a turstless fashion:

1. The compilation and decompilation of unsigned, signed, and notarized transaction intents.
2. The ability to convert manifests between their string and JSON representations.
3. The ability to Bech32 encode and decode addresses as needed.
4. The ability to encode and decode SBOR values as needed.
5. The ability to statically validate transactions to check for common errors and problems (currently a work in progress, although this functionality is implemented as part of certain requests in the library)

## High Level Architecture Overview

This section gives a high-level overview of the architecture of the Radix Engine Toolkit and is useful for developers hoping to extend the Radix Engine Toolkit to add additional interfaces and additional ways to talk to use the core functionality available in the library.

![image](./images/library-overview.png)

The core functionality of the library is implemented in the `radix-engine-toolkit-core` crate. This crate has the definition of the requests, their responses, how they are validated, and how they are handled and fulfilled. This crate utilizes the Scrypto standard library as well as other dependencies from the Scrypto repository to handle requests. If there is a need to extend the library to handle a new request type (e.g. a request to statically validate transactions) then it is implemented in this crate. 

Utilizing the core crate are a number of "interface" crates which expose the functionality of the Radix Engine Toolkit for different platforms and through different serialization techniques. Currently, all communication into and out of toolkit happens in JSON, although, if other serialization techniques are desirable (e.g. CBOR or Protocol Buffers), then an additional "interface" crates may be implemented which make use of these serialization techniques. 

In addition to that, if it is desirable to expose the functionality in different ways (e.g. as a REST API) then a simple server may be implemented as an "interface" for the toolkit which would expose the `radix-engine-toolkit-core` functionality through a REST API interface.

With the above in mind, this means that if you were to build the toolkit from source, you would not build the `radix-engine-toolkit-core` crate. Instead, you would build one of the interface creates available in the repository. There are currently three main interfaces implemented for the Radix Engine Toolkit:

1. WASM JSON Interface: This interface is implemented in the `radix-engine-toolkit-wasm` crate and exposes a WASM interface with JSON as the underlying serialization protocol for the interface. This is useful when you are developing web applications which require functionality available in the Radix Engine Toolkit.
2. JNI JSON Interface: This interface is implemented in the `radix-engine-toolkit-jni` crate and exposes a JNI interface with JSON as the underlying serialization protocol for the interface. This is useful when you want to use the toolkit with a JVM language such as Kotlin, Java, or when you want to use it in building Android applications. Since JNI interfaces require a specific package and namespace, this interface assumes the wrapper to be unpackaged and assumes that the functions will be exposed through a class called `RadixEngineToolkitFFI`.
3. Native JSON Interface: This interface is implemented in the `radix-engine-toolkit-native` crate and exposes a c-style function interface with JSON as the underlying serialization protocol for the interface. This is useful for most applications where you want to build a library that would run on different platforms.

## Exposed Functionality

A full document of the functionality exposed by the Radix Engine Toolkit can be found in the [request examples](./docs-examples-generator/function-examples.md) document. This has a full list of the different functions exposed, what their functionality is, and what a sample request and response looks like for a given request type.

## Schema and Specification

Although the Radix Engine Toolkit is not a traditional REST API, its uses OpenAPI to define the specification of the models used for the requests, responses, instructions, and the value model used by the toolkit. The OpenAPI specification can be found in the [specification](./spec/transaction-api-spec.yaml) document

## Building From Source

This section describes how you can build your own Radix Engine Toolkit directly from source. It describes what the process looks like and also describes what the needed dependencies are.

### Dependencies

1. To build the Radix Engine Toolkit, you need to have both the stable and nightly versions of rust installed. You can do that through the following commands:

    ```shell
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   rustup default nightly
   rustup default stable
   rustup update
   ```
   
2. To allow the library to be cross compiled to different platforms, you must install the different compilation targets through `rustup`. The following command installs a set of targets which are commonly used for the toolkit, however, you may want to add additional targets or remove some of those targets if they are not of interest to you.

    ```shell
   rustup target install aarch64-linux-android aarch64-apple-darwin x86_64-unknown-linux-gnu x86_64-apple-darwin i686-linux-android aarch64-apple-ios aarch64-apple-ios-sim wasm32-unknown-unknown x86_64-pc-windows-gnu armv7-linux-androideabi
   rustup +nightly target install aarch64-linux-android aarch64-apple-darwin x86_64-unknown-linux-gnu x86_64-apple-darwin i686-linux-android aarch64-apple-ios aarch64-apple-ios-sim wasm32-unknown-unknown x86_64-pc-windows-gnu armv7-linux-androideabi
   ```
   
3. (Optional) Certain build targets require that a custom compiler and linker are used. An example of one of those targets is Android which requires these dependencies from the NDK. This library has been built successfully through NDK version r22b and therefore this is the recommended version to install. The following command shows how this may be installed, however, you must keep in mind that you will need to know the path of its installation as it is required for the build script.

    ```shell
    wget https://dl.google.com/android/repository/android-ndk-r22b-darwin-x86_64.zip
    unzip -q android-ndk-r22b-darwin-x86_64.zip -d $HOME/android-ndk
   ```
   
    In addition to the above, depending on the architecture of the machine that the build will run on, you may need additional compilers and linkers (e.g. building for a `x86_64-pc-windows-gnu` target on an `aarch64-apple-darwin` machine requires a custom compiler). 

### Building the Library

The Radix Engine Toolkit comes with a [bash script](./build-specific.sh) to make the building of the library easier. This build script requires certain environment variables to be set before running it:

1. `CRATE_NAME`: This is the name of the crate to build (e.g. `radix-engine-toolkit-native`)
2. `TARGET_TRIPLE`: The target to build the radix engine toolkit for (e.g. `aarch64-apple-darwin`)
3. `CUSTOM_COMPILER`: The custom compiler to use to use for this build. When unsure, set this to the path of your current clang binary and try running this script (e.g. `/usr/bin/clang`).
4. `CUSTOM_ARCHIVER`: The custom archiver to use to use for this build. When unsure, set this to the path of your current llvm-ar binary and try running this script (e.g. `/usr/bin/llvm-ar`).
5. `CUSTOM_LINKER`: The custom linker to use to use for this build. When unsure, do not set this variable to anything and try running this script. This variable should not be needed for all targets.

Once you set these environment variables, building the toolkit is as easy as:
```shell
./build-specific.sh
```
