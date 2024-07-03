package main

/*
 To build this file manually export following environment variable:
 `export CGO_LDFLAGS="-L. -lradix_engine_toolkit_uniffi"`
 Ensure generated Go bindings are in ../../output/ folder, and generated dynamic library is in this folder.
 Execute tests with command:
 `GO111MODULE=auto go test -v`
*/

import (
	"encoding/hex"
	"reflect"
	"testing"

	"../../output/radix_engine_toolkit_uniffi"
)

func TestRetVersion(t *testing.T) {
	var buildInfo = radix_engine_toolkit_uniffi.GetBuildInformation()

	if buildInfo.Version != "2.1.0-dev1" {
		t.Fatalf("Wrong RET version: %s", buildInfo.Version)
	}
}

func assert(t *testing.T, condition bool, message string) {
	if !condition {
		t.Fatalf(message)
	}
}

func newVirtualAccount(t *testing.T, networkId uint8) (*radix_engine_toolkit_uniffi.Address, *radix_engine_toolkit_uniffi.PrivateKey, *radix_engine_toolkit_uniffi.PublicKey) {
	var token = make([]byte, 32)
	var i uint8
	for i = 0; i < 32; i++ {
		token[i] = i + 1
	}

	privateKey, err := radix_engine_toolkit_uniffi.PrivateKeyNewSecp256k1(token)
	assert(t, err == nil, "Unable to generate new private key secp256k1")
	var publicKey = privateKey.PublicKey()

	address, err := radix_engine_toolkit_uniffi.AddressVirtualAccountAddressFromPublicKey(publicKey, networkId)
	assert(t, err == nil, "Unable to generate virtual accound address")

	return address, privateKey, &publicKey
}

func TestRetManifestBuilder(t *testing.T) {
	var networkId uint8 = 2 // Stokenet
	var account, _, publicKey = newVirtualAccount(t, networkId)
	var addresses = radix_engine_toolkit_uniffi.GetKnownAddresses(networkId)

	var mb = radix_engine_toolkit_uniffi.NewManifestBuilder()

	mb, err := mb.FaucetLockFee()
	assert(t, err == nil, "FaucetLockFee failed")

	mb, err = mb.FaucetFreeXrd()
	assert(t, err == nil, "FaucetFreeXrd failed")

	bucket := radix_engine_toolkit_uniffi.ManifestBuilderBucket{
		Name: "free_xrd",
	}
	mb, err = mb.TakeAllFromWorktop(addresses.ResourceAddresses.Xrd, bucket)
	assert(t, err == nil, "TakeAllFromWorktop failed")

	mb, err = mb.AccountTryDepositOrAbort(account, bucket, nil)
	assert(t, err == nil, "AccountTryDepositOrAbort failed")

	var transactionManifest = mb.Build(networkId)

	var manifestSummary = transactionManifest.Summary(networkId)
	assert(t, len(manifestSummary.Classification) == 1, "Wrong length of manifest classification")
	assert(t, manifestSummary.Classification[0] == radix_engine_toolkit_uniffi.ManifestClassGeneral, "Wrong manifest classification")
	assert(t, len(manifestSummary.AccountsDepositedInto) == 1, "Wrong length of accounts deposited into")
	assert(t, manifestSummary.AccountsDepositedInto[0].AddressString() == account.AddressString(), "Wrong accound deposited into value")
	assert(t, len(manifestSummary.EncounteredEntities) == 3, "Wrong length of encountered entities")
	assert(t, manifestSummary.EncounteredEntities[0].AddressString() == addresses.ComponentAddresses.Faucet.AddressString(), "Wrong faucet address at index 0")
	assert(t, manifestSummary.EncounteredEntities[1].AddressString() == addresses.ResourceAddresses.Xrd.AddressString(), "Wrong resource address at index 1")
	assert(t, manifestSummary.EncounteredEntities[2].AddressString() == account.AddressString(), "Wrong account address at index 2")

	_, err = transactionManifest.Compile()
	assert(t, err == nil, "Transaction failed to compile")

	var theader = radix_engine_toolkit_uniffi.TransactionHeader{
		NetworkId:           networkId,
		StartEpochInclusive: 1,
		EndEpochExclusive:   10,
		Nonce:               1,
		NotaryPublicKey:     *publicKey,
		NotaryIsSignatory:   true,
		TipPercentage:       0,
	}

	var message = radix_engine_toolkit_uniffi.MessagePlainText{
		Value: radix_engine_toolkit_uniffi.PlainTextMessage{
			MimeType: "text/plain",
			Message: radix_engine_toolkit_uniffi.MessageContentStr{
				Value: "Somr XRD Transfer",
			},
		},
	}

	var intent = radix_engine_toolkit_uniffi.NewIntent(theader, transactionManifest, message)
	_, err = intent.Hash()
	assert(t, err == nil, "Transaction Intent hash failed")

	var intentBytes, error = intent.Compile()
	assert(t, error == nil, "Transaction Intent compile failed")

	intentHex := hex.EncodeToString(intentBytes)
	assert(t, intentHex == "4d220104210707020a01000000000000000a0a0000000000000009010000002200012007210284bf" +
		"7562262bbd6940085748f3be6afa52ae317155181ece31b66351ccffa4b0010108000020220441038000c0566318c6318c6" +
		"4f798cacc6318c6318cf7be8af78a78f8a6318c6318c60c086c6f636b5f666565210185000010632d5ec76b050000000000" +
		"0000000000000000000041038000c0566318c6318c64f798cacc6318c6318cf7be8af78a78f8a6318c6318c60c046672656" +
		"52100020180005da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c641038000d1e120200bde95a525" +
		"81c9284dab790cd0a1fd242eee0096186d6e5ef3670c147472795f6465706f7369745f6f725f61626f72742102810000000" +
		"022000020200022010121020c0a746578742f706c61696e2200010c11536f6d7220585244205472616e73666572",
		"Intent bytes not equal")

}

func TestRetNotarizedTransaction(t *testing.T) {
	var networkId uint8 = 2 // Stokenet
	var account1, privateKey1, publicKey1 = newVirtualAccount(t, networkId)
	var account2, privateKey2, _ = newVirtualAccount(t, networkId)
	var addresses = radix_engine_toolkit_uniffi.GetKnownAddresses(networkId)

	var mb = radix_engine_toolkit_uniffi.NewManifestBuilder()

	mb, err := mb.FaucetLockFee()
	assert(t, err == nil, "FaucetLockFee failed")

	mb, err = mb.AccountWithdraw(account1, addresses.ResourceAddresses.Xrd, radix_engine_toolkit_uniffi.DecimalOne())
	assert(t, err == nil, "AccountWithdraw failed")

	mb, err = mb.AccountTryDepositEntireWorktopOrAbort(account2, nil)
	assert(t, err == nil, "AccountTryDepositEntireWorktopOrAbort failed")

	var transactionManifest = mb.Build(networkId)

	var theader = radix_engine_toolkit_uniffi.TransactionHeader{
		NetworkId:           networkId,
		StartEpochInclusive: 1,
		EndEpochExclusive:   10,
		Nonce:               1,
		NotaryPublicKey:     *publicKey1,
		NotaryIsSignatory:   true,
		TipPercentage:       0,
	}

	var message = radix_engine_toolkit_uniffi.MessagePlainText{
		Value: radix_engine_toolkit_uniffi.PlainTextMessage{
			MimeType: "text/plain",
			Message: radix_engine_toolkit_uniffi.MessageContentStr{
				Value: "Somr XRD Transfer",
			},
		},
	}

	var notarizedTransaction, error = radix_engine_toolkit_uniffi.NewTransactionBuilder().
		Header(theader).
		Manifest(transactionManifest).
		Message(message).
		SignWithPrivateKey(privateKey1).
		SignWithPrivateKey(privateKey2).
		NotarizeWithPrivateKey(privateKey1)
	assert(t, error == nil, "Notarized Transaction failed")

	_, err = notarizedTransaction.Hash()
	assert(t, err == nil, "Notarized Transaction hash failed")

	var transactionBytes, _ = notarizedTransaction.Compile()

	transactionHex := hex.EncodeToString(transactionBytes)
	assert(t, transactionHex == "4d22030221022104210707020a01000000000000000a0a00000000000000090100000022000" +
		"12007210284bf7562262bbd6940085748f3be6afa52ae317155181ece31b66351ccffa4b0010108000020220341038000c0" +
		"566318c6318c64f798cacc6318c6318cf7be8af78a78f8a6318c6318c60c086c6f636b5f666565210185000010632d5ec76" +
		"b0500000000000000000000000000000041038000d1e120200bde95a52581c9284dab790cd0a1fd242eee0096186d6e5ef3" +
		"670c087769746864726177210280005da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c685000064a" +
		"7b3b6e00d0000000000000000000000000000000041038000d1e120200bde95a52581c9284dab790cd0a1fd242eee009618" +
		"6d6e5ef3670c1a7472795f6465706f7369745f62617463685f6f725f61626f72742102830022000020200022010121020c0" +
		"a746578742f706c61696e2200010c11536f6d7220585244205472616e7366657220220200012101200741009fc7e77ff3f1" +
		"4ad73d83333f636f591933539126e4d9f226780a4fa273ed9fc905849b750318fc3ea50ef85bda0ba3ecb6778967e371555" +
		"b48352d0d8d80dbf300012101200741009fc7e77ff3f14ad73d83333f636f591933539126e4d9f226780a4fa273ed9fc905" +
		"849b750318fc3ea50ef85bda0ba3ecb6778967e371555b48352d0d8d80dbf32200012101200741010e0b2475a6b9ccc784b" +
		"6e23c6766b0757b0d37d1beb825e7acd85cd8ec63badf660f5b34285e9cfb511325d5dc67e8494a973998298dc6076d548e" +
		"36f80ef54d",
		"Notarized Transaction bytes not equal")
}

func TestRetPublicKeyFingerprint(t *testing.T) {
	var bytes = []byte{65, 66, 67, 68, 69, 70, 71, 72, 73, 74}
	var fingerprint = radix_engine_toolkit_uniffi.PublicKeyFingerprintFromVec(bytes)
	var converted = radix_engine_toolkit_uniffi.PublicKeyFingerprintToVec(fingerprint)

	assert(t, len(fingerprint.Bytes) == 10, "Length of string is invalid")
	assert(t, fingerprint.Bytes == "ABCDEFGHIJ", "Conversion of string failed")
	assert(t, len(converted) == 10, "Length of byte array is invalid")
	assert(t, reflect.DeepEqual(converted, bytes), "Conversion of byte array failed")
}

func TestRetDecimalToFromBytes(t *testing.T) {
	var bytes = []byte{78, 243, 148, 77, 255, 81, 151, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0}
	var dec = "1.123456789012345678"

	var number, err = radix_engine_toolkit_uniffi.NewDecimal(dec)
	assert(t, err == nil, "New Decimal failed")
	var bytes_converted = number.ToLeBytes();
	var converted_back = radix_engine_toolkit_uniffi.DecimalFromLeBytes(bytes_converted)

	assert(t, len(bytes_converted) == 24, "Length of byte array is invalid")
	assert(t, reflect.DeepEqual(bytes_converted, bytes), "Conversion of byte array failed")
	assert(t, converted_back.Equal(number), "Conversion of byte array back to Decimal failed")
}
