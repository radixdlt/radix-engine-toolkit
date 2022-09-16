import TransactionService from "../src/transaction-service";
import {
  ManifestInstructionsKind,
  TransactionHeader,
  CompileSignedTransactionIntentResponse,
  CompileNotarizedTransactionIntentResponse,
  TransactionManifest,
  Curve,
  TransactionIntent,
  SignedTransactionIntent,
} from "../src/interfaces";
import { createTransactionService, ManifestBuilder, ValueKind } from "../src/transaction-library";
import { byteArrayFromHex, hexStringFromByteArray } from "../src/utils";
import { Secp256k1KeyPair } from "../src/key_pair";
import { Instruction } from "../src/instruction";

const main = async (): Promise<void> => {
  // Creating a new transaction service object from the transaction service WASM file path
  const transactionService: TransactionService = await createTransactionService();

  // Creating a new key pair for the account that we are about to create.
  let keyPair: Secp256k1KeyPair = Secp256k1KeyPair.newRandom();
  console.log(`Account Public Key ${keyPair.publicKeyHex()}`);
  console.log(`Account Private Key ${keyPair.privateKeyHex()}`);

  // Deriving the `NonFungibleAddress` based on the public key of the account.
  // TODO: The Public Key => NonFungibleAddress conversion should be something that the
  // transaction service does.
  let nonFungibleAddress: Uint8Array = Uint8Array.from(
    Buffer.concat([
      byteArrayFromHex("000000000000000000000000000000000000000000000000000002"), // ECDSA resource address
      Uint8Array.from([
        0x30, // A list
        0x07, // of 8bit unsigned numbers
        0x21, // of a length of 0x21 (33)
        0,
        0,
        0,
      ]),
      keyPair.publicKey(),
    ])
  );
  let nonFungibleAddressString: string = hexStringFromByteArray(nonFungibleAddress);
  console.log("Their NonFungible Address is", nonFungibleAddressString);

  // Creating the transaction for the account creation.
  // TODO: The whole faucet situation might change, this needs to account for that.
  // TODO: Use Alphanet addresses instead of local simulator addresses
  let manifestInstructions: Instruction[] = new ManifestBuilder()
    .callMethod("system_sim1qsqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqs9fh54n", "lock_fee", [
      {
        type: ValueKind.Decimal,
        value: "100",
      },
    ])
    .callMethod("system_sim1qsqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqs9fh54n", "free_xrd")
    .takeFromWorktop(
      "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag",
      "xrd_bucket"
    )
    .callFunction(
      "package_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqpsuluv44",
      "Account",
      "new_with_resource",
      [
        {
          type: ValueKind.Enum,
          variant_name: "Protected",
          fields: [
            {
              type: ValueKind.Enum,
              variant_name: "ProofRule",
              fields: [
                {
                  type: ValueKind.Enum,
                  variant_name: "Require",
                  fields: [
                    {
                      type: ValueKind.Enum,
                      variant_name: "StaticNonFungible",
                      fields: [
                        {
                          type: ValueKind.NonFungibleAddress,
                          address: nonFungibleAddressString,
                        },
                      ],
                    },
                  ],
                },
              ],
            },
          ],
        },
      ]
    )
    .build();

  // Building a transaction manifest from the instructions and header
  let currentEpoch = 0x00; // Should obtain from the Core-API
  let randomNonce = 0x10; // Should be a random number
  let transactionHeader: TransactionHeader = {
    version: 1,
    network_id: 0x0a,
    start_epoch_inclusive: currentEpoch,
    end_epoch_exclusive: currentEpoch + 0x10,
    nonce: randomNonce,
    notary_public_key: {
      type: Curve.Ecdsa,
      public_key: keyPair.publicKeyHex(),
    },
    notary_as_signatory: false,
    cost_unit_limit: 10_000_000,
    tip_percentage: 0,
  };
  let transactionManifest: TransactionManifest = {
    instructions: {
      type: ManifestInstructionsKind.JSON,
      value: manifestInstructions,
    },
  };

  // Creating an account does not require us to sign the transaction or anything of that sort. The
  // only thing that we need to do is notarize the transaction and that should be sufficient.
  let transactionIntent: TransactionIntent = {
    header: transactionHeader,
    manifest: transactionManifest,
  };
  let signedTransactionIntent: SignedTransactionIntent = {
    transaction_intent: transactionIntent,
    signatures: [],
  };
  let compiledSignedTransactionIntent: string = (
    transactionService.compileSignedTransactionIntent(
      signedTransactionIntent
    ) as CompileSignedTransactionIntentResponse
  ).compiled_signed_intent;
  let notarySignature: Uint8Array = keyPair.sign(byteArrayFromHex(compiledSignedTransactionIntent));

  let compiledTransaction: string = (
    transactionService.compileNotarizedTransactionIntent({
      signed_intent: signedTransactionIntent,
      notary_signature: {
        type: Curve.Ecdsa,
        signature: hexStringFromByteArray(notarySignature),
      },
    }) as CompileNotarizedTransactionIntentResponse
  ).compiled_notarized_intent;

  console.log("The compiled transaction for account creation is", compiledTransaction);

  // Submit to the Core-API.
};

main();
