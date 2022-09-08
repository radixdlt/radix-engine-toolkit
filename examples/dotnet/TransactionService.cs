using System.Text;
using System.Runtime.InteropServices;

using WebAssembly;
using WebAssembly.Runtime;
using Newtonsoft.Json;
using Models;

public class TransactionService {
    private WebAssembly.Instance<dynamic> serviceInstance;

    public TransactionService(Stream wasmModule) {
        Instance<dynamic> instance = Compile.FromBinary<dynamic>(wasmModule)(new ImportDictionary());
        this.serviceInstance = instance;
    }

    public TransactionService(System.String wasmModulePath) : this(File.OpenRead(wasmModulePath)) {}

    public InformationResponse information() {
        Func<int, int> function = x => this.serviceInstance.Exports.information(x);
        return this.callWasmFunction<Object, InformationResponse>(
            new object(),
            function
        )!;
    }
    
    public Manifest convert_manifest(
        ConvertManifestRequest request
    ) {
        Func<int, int> function = x => this.serviceInstance.Exports.convert_manifest(x);
        return this.callWasmFunction<Object, Manifest>(
            request,
            function
        )!;
    }
    
    public CompileTransactionIntentResponse compile_transaction_intent(
        TransactionIntent request
    ) {
        Func<int, int> function = x => this.serviceInstance.Exports.compile_transaction_intent(x);
        return this.callWasmFunction<Object, CompileTransactionIntentResponse>(
            request,
            function
        )!;
    }
    
    public TransactionIntent decompile_transaction_intent(
        DecompileTransactionIntentRequest request
    ) {
        Func<int, int> function = x => this.serviceInstance.Exports.decompile_transaction_intent(x);
        return this.callWasmFunction<Object, TransactionIntent>(
            request,
            function
        )!;
    }

    public CompileSignedTransactionIntentResponse compile_signed_transaction_intent(
        SignedTransactionIntent request
    ) {
        Func<int, int> function = x => this.serviceInstance.Exports.compile_signed_transaction_intent(x);
        return this.callWasmFunction<Object, CompileSignedTransactionIntentResponse>(
            request,
            function
        )!;
    }
    
    public SignedTransactionIntent decompile_signed_transaction_intent(
        DecompileSignedTransactionIntentRequest request
    ) {
        Func<int, int> function = x => this.serviceInstance.Exports.decompile_signed_transaction_intent(x);
        return this.callWasmFunction<Object, SignedTransactionIntent>(
            request,
            function
        )!;
    }

    public CompileNotarizedTransactionIntentResponse compile_notarized_transaction_intent(
        NotarizedTransactionIntent request
    ) {
        Func<int, int> function = x => this.serviceInstance.Exports.compile_notarized_transaction_intent(x);
        return this.callWasmFunction<Object, CompileNotarizedTransactionIntentResponse>(
            request,
            function
        )!;
    }

    public NotarizedTransactionIntent decompile_notarized_transaction_intent(
        DecompileNotarizedTransactionIntentRequest request
    ) {
        Func<int, int> function = x => this.serviceInstance.Exports.decompile_notarized_transaction_intent(x);
        return this.callWasmFunction<Object, NotarizedTransactionIntent>(
            request,
            function
        )!;
    }
    
    public DecompileUnknownTransactionIntentResponse decompile_unknown_transaction_intent(
        DecompileUnknownTransactionIntentRequest request
    ) {
        Func<int, int> function = x => this.serviceInstance.Exports.decompile_unknown_transaction_intent(x);
        return this.callWasmFunction<Object, DecompileUnknownTransactionIntentResponse>(
            request,
            function
        )!;
    }

    private Resp callWasmFunction<Req, Resp>(
        Req request,
        Func<int, int> func
    ) {
        // Allocate memory and perform the function call
        IntPtr requestMemoryLocation = this.writeRequestToMemory(request!);
        IntPtr responseMemoryLocation = (IntPtr)func((int)requestMemoryLocation);
        Resp response = this.readResponseFromMemory<Resp>(responseMemoryLocation)!;

        // Free up the allocated memory
        this.freeMemory(requestMemoryLocation);
        this.freeMemory(responseMemoryLocation);

        return response;
    }

    private IntPtr writeRequestToMemory(Object request) {
        // Serialize the passed object and add a null terminator to the end of it.
        System.String serializedRequest = JsonConvert.SerializeObject(request);
        serializedRequest += '\0';

        // Encoding it and allocating enough memory for it
        byte[] encodedRequest = Encoding.UTF8.GetBytes(serializedRequest);
        IntPtr allocatedMemoryLocation = this.allocateMemory((UInt32)encodedRequest.Length);

        // Writing the string to memory
        Marshal.Copy(
            encodedRequest, 
            0, 
            this.serviceInstance.Exports.memory.Start + allocatedMemoryLocation.ToInt32(), 
            encodedRequest.Length
        );

        // Return a pointer to the object in memory
        return allocatedMemoryLocation;
    }

    private T? readResponseFromMemory<T>(IntPtr pointer) {
        // Read the string from memory until the null terminator
        IntPtr memoryLocation = this.serviceInstance.Exports.memory.Start + pointer.ToInt32();
        System.String? responseString = Marshal.PtrToStringAnsi(memoryLocation);
        if (responseString == null) {
            throw new Exception("Response string is null");
        } else {
            return JsonConvert.DeserializeObject<T>(responseString);
        }
    }

    private IntPtr allocateMemory(UInt32 capacity) {
        return (IntPtr)this.serviceInstance.Exports.__transaction_lib_alloc((int)capacity);
    }
    
    private void freeMemory(IntPtr pointer) {
        this.serviceInstance.Exports.__transaction_lib_free((int)pointer);
    }
}