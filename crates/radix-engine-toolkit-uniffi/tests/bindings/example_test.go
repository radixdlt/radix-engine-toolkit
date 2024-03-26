package main

/*
 To build this file manually export following environemnt variable:
 `export CGO_LDFLAGS="-L<path to directory with libradix_engine_toolkit_uniffi library> -lradix_engine_toolkit_uniffi"`
 To execute tests run command:
 `GO111MODULE=auto go test -v`
*/

import (
	"crypto/rand"
	"testing"

	"../../output/radix_engine_toolkit_uniffi"
)

func TestRetVersion(t *testing.T) {
    var build_info = radix_engine_toolkit_uniffi.GetBuildInformation()

    if build_info.Version != "1.0.10" {
        t.Fatalf("Wrong RET version: %s", build_info.Version)
    }
}

func assert(t *testing.T, condition bool, message string) {
    if !condition {
        t.Fatalf(message)
    }
}

func newVirtualAccount(t *testing.T, networkId uint8) *radix_engine_toolkit_uniffi.Address {
    var token = make([]byte, 32)
    rand.Read(token)

    private_key, err := radix_engine_toolkit_uniffi.PrivateKeyNewSecp256k1(token);
    assert(t, err == nil, "Unable to generate new private key secp256k1")
    var public_key = private_key.PublicKey()

    address, err := radix_engine_toolkit_uniffi.AddressVirtualAccountAddressFromPublicKey(public_key, networkId)
    assert(t, err == nil, "Unable to generate virtual accound address")

    return address
}

func TestRet(t *testing.T) {
    var networkId uint8 = 1
    var account = newVirtualAccount(t, networkId)
    var addresses = radix_engine_toolkit_uniffi.GetKnownAddresses(networkId);

    var mb = radix_engine_toolkit_uniffi.NewManifestBuilder()

    mb, err := mb.FaucetLockFee()
    assert(t, err == nil, "FaucetLockFee failed")

    mb, err = mb.FaucetFreeXrd()
    assert(t, err == nil, "FaucetFreeXrd failed")

    bucket :=  radix_engine_toolkit_uniffi.ManifestBuilderBucket {
        Name: "free_xrd",
    }
    mb, err = mb.TakeAllFromWorktop(addresses.ResourceAddresses.Xrd, bucket)
    assert(t, err == nil, "TakeAllFromWorktop failed")

    mb, err = mb.AccountTryDepositOrAbort(account, bucket, nil)
    assert(t, err == nil, "AccountTryDepositOrAbort failed")

    var transactionManifest = mb.Build(networkId)

    var manifestSummary = transactionManifest.Summary(networkId)
    assert(t, manifestSummary.Classification[0] == radix_engine_toolkit_uniffi.ManifestClassGeneral, "Wrong manifest classification")
    assert(t, len(manifestSummary.AccountsDepositedInto) == 1, "Wrong length of accounts deposited into")
}
