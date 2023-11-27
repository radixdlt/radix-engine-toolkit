import os
import shutil
import re

SCRIPT_DIRECTORY: str = os.path.dirname(os.path.realpath(__file__))
REPOSITORY_URL: str = "https://github.com/radixdlt/radixdlt-scrypto"
OUTPUT_DIRECTORIES: list[str] = [
    os.path.join(SCRIPT_DIRECTORY, "..", "crates", "radix-engine-toolkit-core", "tests", "manifests"),
    os.path.join(SCRIPT_DIRECTORY, "..", "crates", "radix-engine-toolkit", "tests", "manifests"),
]

def main() -> None:
    # Get the tag or branch of the Scrypto dependency being used.
    path_of_cargo_toml_manifest: str = os.path.join(SCRIPT_DIRECTORY, '..', 'Cargo.toml')
    branch_or_tag: str = read_tag_or_version(path_of_cargo_toml_manifest)

    # Remove the radixdlt-scrypto if it exists
    radixdlt_scrypto_directory: str = os.path.join(SCRIPT_DIRECTORY, 'radixdlt-scrypto')
    if os.path.exists(radixdlt_scrypto_directory):
        shutil.rmtree(radixdlt_scrypto_directory)

    # Clone the repository
    os.system(f'git clone {REPOSITORY_URL} -b {branch_or_tag} "{radixdlt_scrypto_directory}" --depth 1')

    # Copy the manifests from the path that they're at to the output path
    manifests_path: str = os.path.join(radixdlt_scrypto_directory, 'transaction', 'examples')
    for output_directory in OUTPUT_DIRECTORIES:
        if os.path.exists(output_directory):
            shutil.rmtree(output_directory)
        shutil.copytree(manifests_path, output_directory)

        # Get and apply the needed replacements to the transaction manifests.
        replacements: dict[str, str] = get_replacements(radixdlt_scrypto_directory)
        for (root_directory, _, file_names) in os.walk(output_directory):
            for file_name in file_names:
                if not file_name.endswith('rtm'):
                    continue

                path: str = os.path.join(root_directory, file_name)
                with open(path, 'r') as file:
                    content: str = file.read()

                for (old, new) in replacements.items():
                    content = content.replace(old, new)

                with open(path, 'w') as file:
                    file.write(content)

    # Remove the radixdlt-scrypto repo after we're finished with it.
    shutil.rmtree(radixdlt_scrypto_directory)

def read_tag_or_version(manifest_path: str) -> str:
    if not manifest_path.endswith('.toml'):
        raise Exception("Invalid file extension")
    
    with open(manifest_path, 'r') as file:
        contents: str = file.read()

    pattern: str = r'scrypto\s?=\s?\{.*[branch|tag|rev]\s?=\s?"([\w\d\/.-]*)".*\}'
    return re.findall(pattern, contents)[0]

def get_replacements(radixdlt_scrypto_directory: str) -> dict[str, str]:
    replacements_file_path: str = os.path.join(radixdlt_scrypto_directory, 'transaction', 'src', 'manifest', 'e2e.rs')
    
    with open(replacements_file_path, 'r') as file:
        content: str = file.read()

    pattern: str = r'\([\n\s\t]*"(.*?)",[\n\s\t]*"(.*?)",?[\n\s\t]*\)'
    return {
        key: value
        for key, value
        in re.findall(pattern, content)
    }

if __name__ == "__main__":
    main()