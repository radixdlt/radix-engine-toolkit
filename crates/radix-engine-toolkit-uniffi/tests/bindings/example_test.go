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

	if buildInfo.Version != "2.0.0" {
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
	var networkId uint8 = 1
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
	assert(t, intentHex == "4d220104210707010a01000000000000000a0a0000000000000009010000002200012007210284bf" + 
		"7562262bbd6940085748f3be6afa52ae317155181ece31b66351ccffa4b0010108000020220441038000c0566318c6318c6" +
		"4f798cacc6318c6318cf7be8af78a78f8a6318c6318c60c086c6f636b5f666565210185000010632d5ec76b050000000000" +
		"0000000000000000000041038000c0566318c6318c64f798cacc6318c6318cf7be8af78a78f8a6318c6318c60c046672656" +
		"52100020180005da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c641038000d1e120200bde95a525" +
		"81c9284dab790cd0a1fd242eee0096186d6e5ef3670c147472795f6465706f7369745f6f725f61626f72742102810000000" +
		"022000020200022010121020c0a746578742f706c61696e2200010c11536f6d7220585244205472616e73666572",
		"Intent bytes not equal")

}

func TestRetNotarizedTransaction(t *testing.T) {
	var networkId uint8 = 1
	var account1, privateKey1, publicKey1 = newVirtualAccount(t, networkId)
	var account2, privateKey2, _ = newVirtualAccount(t, networkId)
	var addresses = radix_engine_toolkit_uniffi.GetKnownAddresses(networkId)

	var mb = radix_engine_toolkit_uniffi.NewManifestBuilder()

	mb, err := mb.FaucetLockFee()
	assert(t, err == nil, "FaucetLockFee failed")

	mb, err = mb.AccountWithdraw(account1, addresses.ResourceAddresses.Xrd, radix_engine_toolkit_uniffi.DecimalOne())
	assert(t, err == nil, "AccountWithdraw failed")

	mb, err = mb.AccountDepositEntireWorktop(account2)
	assert(t, err == nil, "AccountDepositEntireWorktop failed")

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
	assert(t, transactionHex == "4d22030221022104210707010a01000000000000000a0a00000000000000090100000022000" +
		"12007210284bf7562262bbd6940085748f3be6afa52ae317155181ece31b66351ccffa4b0010108000020220341038000c0" +
		"566318c6318c64f798cacc6318c6318cf7be8af78a78f8a6318c6318c60c086c6f636b5f666565210185000010632d5ec76" +
		"b0500000000000000000000000000000041038000d1e120200bde95a52581c9284dab790cd0a1fd242eee0096186d6e5ef3" +
		"670c087769746864726177210280005da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c685000064a" +
		"7b3b6e00d0000000000000000000000000000000041038000d1e120200bde95a52581c9284dab790cd0a1fd242eee009618" +
		"6d6e5ef3670c0d6465706f7369745f62617463682101830020200022010121020c0a746578742f706c61696e2200010c115" +
		"36f6d7220585244205472616e736665722022020001210120074100df99c5ae00f9b69903074625de0d5175a89a0b2ae19d" +
		"5cf599dbcb0be93512d838b58d9829a2b9afd3f511b97d3ce01f24a990186eb75f7092e496c54aef8406000121012007410" +
		"0df99c5ae00f9b69903074625de0d5175a89a0b2ae19d5cf599dbcb0be93512d838b58d9829a2b9afd3f511b97d3ce01f24" +
		"a990186eb75f7092e496c54aef84062200012101200741018588242bce5f10c823b7a7cc59cafaa958e2380a95867ee1f7b" +
		"e78f8772092300373c892fb35f22f1392e2b9edd0833c3d655961e400ae954d2a6e18d1ad55e8",
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
