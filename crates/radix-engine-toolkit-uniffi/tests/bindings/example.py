from radix_engine_toolkit_uniffi import *

# Act
build_info = get_build_information()

# Assert
assert build_info.version == "2.2.2-dev1"