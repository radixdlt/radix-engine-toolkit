from radix_engine_toolkit_uniffi import *

# Act
build_info = build_information()

# Assert
assert build_info.version == "0.12.0-fig.1"