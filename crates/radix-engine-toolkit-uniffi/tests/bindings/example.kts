import com.radixdlt.ret.*;

// Act
val information = getBuildInformation();

// Assert
assert(information.version == "2.2.0-dev2");