from radix_engine_toolkit_uniffi import *

# Act
build_info = build_information()

# Assert
assert build_info.version == "1.0.6"