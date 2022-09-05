"""
This script generates a OpenAPI 3.0.0 YAML specification of the models used in this library from the
TypeScript example provided above. The reason models are generated from TypeScript and not the 
original rust code is that there do not exist any useful generators for Rust that are compatible
with the serde and serde_with crates
"""

from typing import Dict, Any
import yaml
import os
import io

def main() -> None:
    script_path: str = os.path.dirname(os.path.realpath(__file__))

    # Generating the OpenAPI specification
    typescript_dir: str = os.path.join(script_path, '..', 'examples', 'typescript')
    os.chdir(typescript_dir)
    spec_string: str = os.popen(f'npx --yes ts-to-openapi --no-as-comment').read()
    os.chdir(script_path)
    spec: Dict[Any, Any] = yaml.safe_load(io.StringIO(spec_string)) # type: ignore

    # Adding additional properties to the spec to make it a proper OpenAPI file
    # TODO: In the current implementation, the components are not sorted in any local way. Example,
    # we have `Bool` coming right after `AssertWorktopContains` which is not at the end of the 
    # instruction list. We need to order these in a more friendly way.
    # TODO: The tool I'm using generates a 3.1 OpenAPI spec which has no support with generators and
    # editors. Consider using another tool.
    spec: Dict[Any, Any] = {
        'openapi': '3.1.0',
        'info': {
            "title": "Transaction Lib",
            "version": "0.1.0"
        },
        'version': '0.1.0',
        'components': spec['components']
    }

    # Writing the spec to a file
    with open(os.path.join(script_path, 'transaction-api-spec.yaml'), 'w') as file:
        yaml.dump(spec, file, sort_keys=False)

if __name__ == "__main__":
    main()