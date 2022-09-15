import wasmInit from "../wasm/transaction_library.wasm?init";
import TransactionService from "./transaction-service";

export { ManifestKind, Curve } from "./interfaces";
export { default as ManifestBuilder } from "./manifest-builder";
export * from "./value";

export const createTransactionService = async (
  options: WebAssembly.Imports = {
    env: {
      memory: new WebAssembly.Memory({
        initial: 1,
      }),
    },
  }
) => new TransactionService(await wasmInit(options));
