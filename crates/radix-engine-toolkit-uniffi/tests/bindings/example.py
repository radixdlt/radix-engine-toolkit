from radix_engine_toolkit_uniffi import *

# Act
build_info = get_build_information()

# Assert
assert build_info.version == "1.0.10"