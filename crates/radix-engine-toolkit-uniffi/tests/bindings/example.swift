import Foundation
import radix_engine_toolkit_uniffi

do {
    // Act
    let buildInformation = radix_engine_toolkit_uniffi.getBuildInformation()

    // Assert
    assert(buildInformation.version == "1.0.10")
}
