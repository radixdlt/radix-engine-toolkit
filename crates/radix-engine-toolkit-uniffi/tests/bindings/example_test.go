package main

/*
 To build this file manually export following environemnt variable:
 `export CGO_LDFLAGS="-L<path to directory with libradix_engine_toolkit_uniffi library> -lradix_engine_toolkit_uniffi"`
 To execute tests run command:
 `GO111MODULE=auto go test -v`
*/

import "../../output/radix_engine_toolkit_uniffi"
import "testing"

func TestRetVersion(t *testing.T) {
    var build_info = radix_engine_toolkit_uniffi.GetBuildInformation()

    if build_info.Version != "1.0.10" {
        t.Fatalf("Wrong RET version: %s", build_info.Version)
    }
}
