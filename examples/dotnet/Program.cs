// This implementation is meant to be a reference implementation and an example and not a production
// ready implementation as the C# implementation has not been tested very thoroughly. You can use 
// this as a starting point for your integration.
//
// There are two main things missing from this C# implementation:
// 1- All requests to the transaction library either retrun a valid response or an error. Since C#
//    doesn't have anything equivalent to Rust's `Result<T, E>`, what is coded here is just the 
//    "happy" path without any error handling or exceptions.
// 2- Bucket and proof identifiers should either be strings of u32. However, the OpenAPI generator
//    I'm using failed to generate this. So, all that the C# code supports (for the time being) is
//    string identifiers for buckets and proofs. We should look into this.

using System.Security.Cryptography;
using Nethereum.Hex.HexConvertors.Extensions;
using Nethereum.Signer;
using Nethereum.Signer.Crypto;
using Models;

byte[] computeDoubleHash(byte[] array)
{
    using (SHA256 sha256 = SHA256.Create())
    {
        return sha256.ComputeHash(sha256.ComputeHash(array));
    }
}

// Defining the paths where the WASM module and the sample complex.rtm file are at
const System.String wasmModulePath = "../../target/wasm32-unknown-unknown/release/transaction_library.wasm";
const System.String manifestFilePath = "../complex.rtm";

var manifestStr = File.ReadAllText(manifestFilePath);
var manifestString = new ManifestString(manifestStr);

// Creating a new transaction service from the above defined WASM module path
var service = new TransactionService(wasmModulePath);

// Example 1: Printing the information of the transaction service. This is essentially the "Hello 
// World" of this project. If the information of the package is printed correctly, then this means 
// that the calls to the WASM modules are happening without any issues.
Console.WriteLine("======= Example 1 =======");
Console.WriteLine(service.information().ToJson());
Console.WriteLine("=========================\n");

// Example 2: One of the functions that are exposed by this library is one which allows clients to 
// convert manifests from one format to another. In this example, we will read the manifest file in 
// the `examples` directory and convert it to a JSON manifest through the transaction library.
var convertManifestRequest = new ConvertManifestRequest(
    0x01,
    0xF2,
    ManifestKind.JSON,
    manifestString
);
var convertManifestResponse = service.convert_manifest(convertManifestRequest);
Console.WriteLine("======= Example 2 =======");
Console.WriteLine(convertManifestResponse.ToJson());
Console.WriteLine("=========================\n");

// Example 3: When signing a transaction, the compiled intent of a transaction is what gets signed. 
// Obtaining this compiled intent requires SBOR encoding the intent and therefore requires an SBOR 
// implementation. However, this library provides the ability to compile transactions without 
// needing to implement the SBOR codec at the client.
var transactionHeader = new TransactionHeader(
    version: 0x01,
    networkId: 0xF2,
    startEpochInclusive: 0x00,
    endEpochExclusive: 0x20,
    nonce: 0x00,
    notaryPublicKey: "031c3796382de8e6e7a1aacb069221e43943af8be417d4c8c92dca7c4b07f93969",
    notaryAsSignatory: false,
    costUnitLimit: 0x0,
    tipPercentage: 0x0
);
var transactionIntent = new TransactionIntent(transactionHeader, manifestString);
var compileTransactionIntentResponse =
    service.compile_transaction_intent(transactionIntent);
Console.WriteLine("======= Example 3 =======");
Console.WriteLine(compileTransactionIntentResponse.ToJson());
Console.WriteLine("=========================\n");

// Example 4: There are certain cases where you might the compiled transaction intent and you wish 
// to understand what exactly you might be signing. In this case, you would need to decompile the 
// byte-representation of the transaction intent into something that you can understand (in code or 
// as a human).
var decompileTransactionIntentRequest = new DecompileTransactionIntentRequest(
    compiledIntent: compileTransactionIntentResponse.CompiledIntent,
    manifestOutputFormat: ManifestKind.JSON
);
var decompileTransactionIntentResponse = service.decompile_transaction_intent(
    decompileTransactionIntentRequest
);
Console.WriteLine("======= Example 4 =======");
Console.WriteLine(decompileTransactionIntentResponse.ToJson());
Console.WriteLine("=========================\n");

// Example 5: In example 3, we compiled a manifest down to its SBOR bytes representation, which we 
// need when signing transactions. In this example, we will sign a transaction with multiple private
// keys and then request a compiled signed transaction intent from the transactions API.
var privateKeys = new List<System.String> {
    "d54b4de65b9bb6b076c248e4d3d14ef29875a241e1245f54e6601b0827123fd4",
    "08724d6795c40488df15c653c5ac4831c466482ec65846723add17ee2b67c610",
    "c98b96a1263b8b8506c71590357214e2e064ed36b7bf780c40a6a81d51b80916",
    "85657258fbf0a5751c3fc89e0cff88d7ac0801d6b5216a028c37085a179e2451",
}.Select(privateKey => new EthECKey(privateKey)).ToArray();

var compiledIntent = Convert.FromHexString(compileTransactionIntentResponse.CompiledIntent);
var compiledIntentDoubleHash = computeDoubleHash(compiledIntent);

var signatures = privateKeys.Select(privateKey =>
{
    var publicKey = new ECKey(privateKey.GetPrivateKeyAsBytes(), true).GetPubKey(true);
    EthECDSASignature signature = privateKey.Sign(compiledIntentDoubleHash);

    var zeroPaddedSignature = Enumerable
        .Repeat<Byte>(0, 64 - (signature.R.Length + signature.S.Length))
        .Concat(signature.R)
        .Concat(signature.S)
        .ToArray();

    return new Signature(publicKey.ToHex(), zeroPaddedSignature.ToHex());
}).ToArray();

var signedTransactionIntent = new SignedTransactionIntent(
    transactionIntent,
    signatures.ToList()
);
var compileSignedTransactionIntentResponse = service.compile_signed_transaction_intent(signedTransactionIntent);
Console.WriteLine("======= Example 5 =======");
Console.WriteLine(compileSignedTransactionIntentResponse.ToJson());
Console.WriteLine("=========================\n");

// Example 6: Just like we have done with the previous examples, anything that is compiled down can 
// be decompiled again. In this case, the compiled signed transaction intent can be decompiled.
var decompileSignedTransactionIntentRequest = new DecompileSignedTransactionIntentRequest(
    compileSignedTransactionIntentResponse.CompiledSignedIntent,
    ManifestKind.JSON
);
var decompileSignedTransactionIntentResponse = service.decompile_signed_transaction_intent(
    decompileSignedTransactionIntentRequest
);
Console.WriteLine("======= Example 6 =======");
Console.WriteLine(decompileSignedTransactionIntentResponse.ToJson());
Console.WriteLine("=========================\n");

// Example 7: Compiling and decompiling of notarized transactions.
var notaryPrivateKey = new EthECKey("0d5666def4fb894f18a5075b261845c044b7e3dd2ba8514b2614dbbb6606c622");

var compiledSignedIntent = Convert.FromHexString(compileSignedTransactionIntentResponse.CompiledSignedIntent);
var compiledSignedIntentDoubleHash = computeDoubleHash(compiledSignedIntent);

var notarySignature = notaryPrivateKey.Sign(compiledSignedIntentDoubleHash);
string notarySignatureString = Enumerable
    .Repeat<Byte>(0, 64 - (notarySignature.R.Length + notarySignature.S.Length))
    .Concat(notarySignature.R)
    .Concat(notarySignature.S)
    .ToArray()
    .ToHex();

var compileNotarizedTransactionIntentRequest = new NotarizedTransactionIntent(
    signedTransactionIntent,
    notarySignatureString
);
var compileNotarizedTransactionIntentResponse = service.compile_notarized_transaction_intent(
    compileNotarizedTransactionIntentRequest
);
Console.WriteLine("======= Example 7 =======");
Console.WriteLine(compileNotarizedTransactionIntentResponse.ToJson());
Console.WriteLine("=========================\n");

var decompileNotarizedTransactionIntentRequest = new DecompileNotarizedTransactionIntentRequest(
    compileNotarizedTransactionIntentResponse.CompiledNotarizedIntent,
    ManifestKind.JSON
);
var decompileNotarizedTransactionIntentResponse = service.decompile_notarized_transaction_intent(
    decompileNotarizedTransactionIntentRequest
);
Console.WriteLine("======= Example 7 =======");
Console.WriteLine(decompileNotarizedTransactionIntentResponse.ToJson());
Console.WriteLine("=========================\n");

// Example 8: There are cases where we might have some blob which we suspect to be a transaction 
// intent of some sort. However, there is no easy way to tell whether this is an unsigned, signed, 
// or notarized transaction compiled transaction intent blob. For this specific use case, this 
// library provides a function for the decompilation of a compiled transaction intent which we are 
// not sure what type it is.
var decompileUnknownTransactionIntentRequest = new DecompileUnknownTransactionIntentRequest(
    compiledUnknownIntent: compileNotarizedTransactionIntentResponse.CompiledNotarizedIntent,
    manifestOutputFormat: ManifestKind.JSON
);
var decompileUnknownTransactionIntentResponse = service.decompile_unknown_transaction_intent(
    decompileUnknownTransactionIntentRequest
);
Console.WriteLine("======= Example 8 =======");
Console.WriteLine(decompileUnknownTransactionIntentResponse.ToJson());
Console.WriteLine("=========================\n");
