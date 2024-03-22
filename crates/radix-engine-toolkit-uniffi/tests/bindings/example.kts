import com.radixdlt.ret.*;

// Act
val information = getBuildInformation();

// Assert
assert(information.version == "1.0.10");