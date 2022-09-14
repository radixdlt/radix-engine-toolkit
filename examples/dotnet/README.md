# C# Example Client

TODO: Complete this document

## Running this Example

The steps required to run this example are fairly simple: the first thing you do is build the transaction library, build the client code (this example), and then run the client code. The following commands can be used to do what is described above. Alternatively, you may run the `run.sh` script in this directory which will do that for you.

1. The first thing that you will need to do is to build the WASM module of the transaction library. This can be done by running the `build.sh` script at the root of the repository. This will build the library for release and will copy the WASM module into the C# directory. This can be run through the following command:

   ```shell
   $ ./build.sh
   ```

2. We will then build the dotnet example in debug mode (you can also build it for release if you would like, however, this example will use paths from the debug build). This can be done by navigating to the directory containing the dotnet example (this directory) and then running the build command:

   ```shell
   $ cd ./examples/dotnet
   $ dotnet build
   ```

3. To run the example, navigate to the directory containing the built binary and execute it:

   ```shell
   $ cd ./bin/Debug/net6.0
   $ ./dotnet
   ```
