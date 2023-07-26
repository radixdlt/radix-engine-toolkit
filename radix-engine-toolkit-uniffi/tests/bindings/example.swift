import Foundation
import radix_engine_toolkit_uniffi

do {
    // Act
    let buildInformation = radix_engine_toolkit_uniffi.buildInformation()

    // Assert
    assert(buildInformation.version == "0.11.0")
}
