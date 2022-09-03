import TransactionService from "./transaction-service";

const main = async (): Promise<void> => {
	// Creating a new transaction service object from the transaction service WASM file path
	const path: string = "../../target/wasm32-unknown-unknown/release/transaction_service.wasm";
	const transactionService: TransactionService = await TransactionService.fromPath(path);

	// Example 1: Printing the information of the transaction service. This is essentially the
	// "Hello World" of this project. If the information of the package is printed correctly, then
	// this means that the calls to the WASM modules are happening without any issues.
	console.log(transactionService.information());
};

main();
