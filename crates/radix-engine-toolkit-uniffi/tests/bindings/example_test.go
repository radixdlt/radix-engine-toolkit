package main

/*
 To build this file manually export following environment variable:
 `export CGO_LDFLAGS="-L<path to directory with libradix_engine_toolkit_uniffi library> -lradix_engine_toolkit_uniffi"`
 Ensure generated Go bindings are in ../../output/ folder.
 Execute tests with command:
 `GO111MODULE=auto go test -v`
*/

import (
	"crypto/rand"
	"testing"

	"../../output/radix_engine_toolkit_uniffi"
)

func TestRetVersion(t *testing.T) {
	var buildInfo = radix_engine_toolkit_uniffi.GetBuildInformation()

	if buildInfo.Version != "1.0.10" {
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
	rand.Read(token)

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

	_, err = intent.Compile()
	assert(t, err == nil, "Transaction Intent compile failed")
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

	_, err = notarizedTransaction.Compile()
	assert(t, err == nil, "Notarized Transaction compile failed")
}
