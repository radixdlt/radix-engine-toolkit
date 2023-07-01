import uniffi.radix_engine_toolkit_uniffi.*;

// Act
val information = buildInformation();

// Assert
assert(information.version == "0.10.0-elm.1");