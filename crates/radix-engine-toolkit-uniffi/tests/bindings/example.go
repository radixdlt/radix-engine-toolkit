package main

import "../../output/radix_engine_toolkit_uniffi"
import "fmt"

func main() {
    var bi = radix_engine_toolkit_uniffi.GetBuildInformation()

    fmt.Println("RET version:")
    fmt.Println(bi.Version)
}
