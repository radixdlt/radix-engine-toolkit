import os
import shutil
import re

SCRIPT_DIRECTORY: str = os.path.dirname(os.path.realpath(__file__))
REPOSITORY_URL: str = "https://github.com/radixdlt/radixdlt-scrypto"
OUTPUT_PATH: str = os.path.join(SCRIPT_DIRECTORY, "radix-engine-toolkit", "src", "models", "manifest", "inputs.rs")

def main() -> None:
    # Get the tag or branch of the Scrypto dependency being used.
    path_of_cargo_toml_manifest: str = os.path.join(SCRIPT_DIRECTORY, 'radix-engine-toolkit-core', 'Cargo.toml')
    branch_or_tag: str = read_tag_or_version(path_of_cargo_toml_manifest)

    # Remove the radixdlt-scrypto if it exists
    radixdlt_scrypto_directory: str = os.path.join(SCRIPT_DIRECTORY, 'radixdlt-scrypto')
    if os.path.exists(radixdlt_scrypto_directory):
        shutil.rmtree(radixdlt_scrypto_directory)

    # Clone the repository
    os.system(f'git clone {REPOSITORY_URL} -b {branch_or_tag} "{radixdlt_scrypto_directory}"')

    # Obtain the struct definitions that match the Regex pattern
    structs: list[str] = []
    for (root_directory, _, file_names) in os.walk(radixdlt_scrypto_directory):
        for file_name in file_names:
            if not file_name.endswith('rs'):
                continue

            path: str = os.path.join(root_directory, file_name)
            with open(path, 'r') as file:
                content: str = file.read()

            new_structs: list[str] = re.findall(r'struct\s*.*Input\s*\{[\n\s\w\t_:(),<>#\[\]\(\)="]*BTreeMap[\n\s\w\t_:(),<>#\[\]\(\)="]*\}', content)
            structs.extend(new_structs)
    
    replacements: dict[str, str] = {}
    imports: list[str] = [
        "use radix_engine::blueprints::package::*;",
        "use radix_engine_common::prelude::*;",
        "use scrypto::api::node_modules::metadata::*;",
        "use scrypto::prelude::*;",
    ]    
    with open(OUTPUT_PATH, 'w') as file:
        file.write("\n".join(imports) + '\n\n')
        
        for struct in structs:
            derives: str = "#[derive(ManifestSbor, Clone, Debug)]" if "ManifestInput" in struct else "#[derive(ScryptoSbor, Clone, Debug)]"
            struct: str = struct.replace("BTreeMap", "IndexMap")
            struct_ident: str = re.findall(r'struct\s*(.*)Input', struct)[0]
            struct = "pub " + struct.replace(struct_ident, f'{struct_ident}IndexMap')

            replacements[struct_ident + "Input"] = f'{struct_ident}IndexMapInput'

            file.write(derives + '\n' + struct + '\n\n')

    # Remove the radixdlt-scrypto repo after we're finished with it.
    shutil.rmtree(radixdlt_scrypto_directory)

    # Use the new types in code.
    for (root_dir, _, file_names) in os.walk(SCRIPT_DIRECTORY):
        for file_name in file_names:
            if not file_name.endswith(".rs"):
                continue;

            path: str = os.path.join(root_dir, file_name)
            with open(path, 'r') as file:
                content: str = file.read()

            for (old, new) in replacements.items():
                content = content.replace(old, new)

            with open(path, 'w') as file:
                file.write(content)

def read_tag_or_version(manifest_path: str) -> str:
    if not manifest_path.endswith('.toml'):
        raise Exception("Invalid file extension")
    
    with open(manifest_path, 'r') as file:
        contents: str = file.read()

    pattern: str = r'scrypto\s?=\s?\{.*[branch|tag]\s?=\s?"([\w\d\/-]*)".*\}'
    return re.findall(pattern, contents)[0]

if __name__ == "__main__":
    main()