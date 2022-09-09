from abc import ABC, abstractmethod
import shutil
from typing import Any, Dict, List
from enum import Enum, auto
import requests
import subprocess
import yaml
import os
import re


class OpenApiProgrammingLanguage(Enum):
    TypeScript = auto()
    CSharp = auto()

    def __str__(self) -> str:
        if self == OpenApiProgrammingLanguage.TypeScript:
            return "ts"
        elif self == OpenApiProgrammingLanguage.CSharp:
            return "csharp"
        else:
            raise ValueError("Non exhaustive if else statement")


class LanguageModelGenerator(ABC):
    spec_path: str
    output_path: str
    spec: Dict[Any, Any]
    language: OpenApiProgrammingLanguage

    @abstractmethod
    def internal_pre_process(self):
        """
        An internal method used to perform some preprocessing before the OpenAPI models are
        generated. There are a number of places where the preprocessing method may be useful, as an
        example, you might wish to make changes to the OpenAPI spec before the models are generated
        (perhaps because the OpenAPI generator for that language does not support something in the
        spec). So, this method allows you to perform any kind of preprocessing you wish to perform
        before the models are generated.
        """
        if os.path.exists(self.output_path):
            shutil.rmtree(self.output_path)

    @abstractmethod
    def internal_post_process(self):
        """
        An internal method called after the models have been generated. Most of the time, this is a
        cleanup method used to cleanup the directory where the models were originally generated.
        """
        pass

    @abstractmethod
    def internal_generate_models(self, open_api_generator_jar_path: str):
        """
        An internal method used for the generation of the OpenAPI models.
        """
        # Building the command used for the generation of the OpenAPI models
        command: List[str] = [
            "java",
            "-jar",
            open_api_generator_jar_path,
            "generate",
            "-g",
            str(self.language),
            "-i",
            self.spec_path,
            "-o",
            self.output_path,
        ]

        # Running the model generation command
        response: subprocess.Popen[Any] = subprocess.Popen(
            " ".join(command),
            shell=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
        )
        stderr: str = response.communicate()[1]

        if stderr:
            raise ValueError(f"Generation of models failed. Error {stderr}")

    @abstractmethod
    def generate_models(self, open_api_generator_jar_path: str):
        """
        A public method which combined all of the internal methods to preprocess, and generate the
        OpenAPI models for the desired language, and then process the generated models.
        """
        self.internal_pre_process()
        self.internal_generate_models(open_api_generator_jar_path)
        self.internal_post_process()


class CSharpModelGenerator(LanguageModelGenerator):
    def __init__(self, spec_path: str, output_path: str):
        self.spec_path = spec_path
        self.output_path = output_path
        with open(spec_path, "r") as file:
            self.spec = yaml.safe_load(file)
        self.language = OpenApiProgrammingLanguage.CSharp

    def internal_pre_process(self):
        super().internal_pre_process()

    def internal_post_process(self):
        # The OpenAPI generator for C# has some issues where it doesn't work well with the
        # discriminator. It requires manual fixes to be made.
        self.__fix_model_discriminator()
        self.__fix_one_of_string_integer()
        self.__fix_unavailable_imports()
        self.__fix_namespace_name()
        self.__fix_incorrect_types()
        self.__fix_model_path()

    def internal_generate_models(self, open_api_generator_jar_path: str):
        super().internal_generate_models(open_api_generator_jar_path)

    def generate_models(self, open_api_generator_jar_path: str):
        super().generate_models(open_api_generator_jar_path)

    def __fix_model_path(self):
        current_models_directory: str = os.path.join(
            self.output_path, "src", "Org.OpenAPITools", "Model"
        )
        new_models_directory: str = self.output_path
        models: List[str] = os.listdir(current_models_directory)
        files: List[str] = os.listdir(new_models_directory)

        for model in models:
            current_model_path: str = os.path.join(current_models_directory, model)
            new_model_path: str = os.path.join(new_models_directory)
            shutil.move(current_model_path, new_model_path)
        for file in files:
            file_path: str = os.path.join(new_models_directory, file)
            if os.path.isfile(file_path):
                os.remove(file_path)
            elif os.path.isdir(file_path):
                shutil.rmtree(file_path)
            else:
                raise FileExistsError("File does not exist")

    def __fix_model_discriminator(self):
        instruction_replace_dict: Dict[str, str] = {
            "CallFunction": "CALL_FUNCTION",
            "CallMethod": "CALL_METHOD",
            "CallMethodWithAllResources": "CALL_METHOD_WITH_ALL_RESOURCES",
            "TakeFromWorktop": "TAKE_FROM_WORKTOP",
            "TakeFromWorktopByAmount": "TAKE_FROM_WORKTOP_BY_AMOUNT",
            "TakeFromWorktopByIds": "TAKE_FROM_WORKTOP_BY_IDS",
            "AssertWorktopContains": "ASSERT_WORKTOP_CONTAINS",
            "AssertWorktopContainsByAmount": "ASSERT_WORKTOP_CONTAINS_BY_AMOUNT",
            "AssertWorktopContainsByIds": "ASSERT_WORKTOP_CONTAINS_BY_IDS",
            "PopFromAuthZone": "POP_FROM_AUTH_ZONE",
            "PushToAuthZone": "PUSH_TO_AUTH_ZONE",
            "ClearAuthZone": "CLEAR_AUTH_ZONE",
            "CreateProofFromAuthZone": "CREATE_PROOF_FROM_AUTH_ZONE",
            "CreateProofFromAuthZoneByAmount": "CREATE_PROOF_FROM_AUTH_ZONE_BY_AMOUNT",
            "CreateProofFromAuthZoneByIds": "CREATE_PROOF_FROM_AUTH_ZONE_BY_IDS",
            "CreateProofFromBucket": "CREATE_PROOF_FROM_BUCKET",
            "CloneProof": "CLONE_PROOF",
            "DropProof": "DROP_PROOF",
            "DropAllProofs": "DROP_ALL_PROOF",
            "ReturnToWorktop": "RETURN_TO_WORKTOP",
            "PublishPackage": "PUBLISH_PACKAGE",
        }

        models_directory: str = os.path.join(
            self.output_path, "src", "Org.OpenAPITools", "Model"
        )
        for model_name in sorted(os.listdir(models_directory)):
            model_file_path: str = os.path.join(models_directory, model_name)

            with open(model_file_path, "r") as file:
                content: str = file.read()

            matches: List[str] = re.findall(r"public (\w+).*: base\(\w+\)", content)
            if matches:
                match: str = matches[0]

                content: str = re.sub(
                    r"public (\w+)(.*: base)\(\w+\)",
                    rf'public \1 \2 ("{instruction_replace_dict.get(match) if instruction_replace_dict.get(match) is not None else match}")',
                    content,
                )

            if "[JsonSubtypes.KnownSubType(" in content:
                value_name: str = re.findall(
                    r"public string (\w+) { get; private set; }", content
                )[0]
                content: str = re.sub(
                    r"public (\w+)\(\)[.\s]*{[.\s]*}",
                    rf"public \1(string value = default(string)){{\n\t\t\tthis.{value_name} = value;\n\t\t}}",
                    content,
                )

            with open(model_file_path, "w") as file:
                file.write(content)

    def __fix_one_of_string_integer(self):
        models_directory: str = os.path.join(
            self.output_path, "src", "Org.OpenAPITools", "Model"
        )
        for model_name in sorted(os.listdir(models_directory)):
            model_file_path: str = os.path.join(models_directory, model_name)

            with open(model_file_path, "r") as file:
                content: str = file.read()

            with open(model_file_path, "w") as file:
                file.write(content.replace("OneOfstringinteger", "string"))

    def __fix_unavailable_imports(self):
        models_directory: str = os.path.join(
            self.output_path, "src", "Org.OpenAPITools", "Model"
        )
        for model_name in sorted(os.listdir(models_directory)):
            model_file_path: str = os.path.join(models_directory, model_name)

            with open(model_file_path, "r") as file:
                content: str = file.read()

            with open(model_file_path, "w") as file:
                file.write(
                    content.replace(
                        "using OpenAPIDateConverter = Org.OpenAPITools.Client.OpenAPIDateConverter;\n",
                        "",
                    )
                )

    def __fix_namespace_name(self):
        models_directory: str = os.path.join(
            self.output_path, "src", "Org.OpenAPITools", "Model"
        )
        for model_name in sorted(os.listdir(models_directory)):
            model_file_path: str = os.path.join(models_directory, model_name)

            with open(model_file_path, "r") as file:
                content: str = file.read()

            with open(model_file_path, "w") as file:
                file.write(
                    content.replace(
                        "namespace Org.OpenAPITools.Model", "namespace Models"
                    )
                )

    def __fix_incorrect_types(self):
        models_directory: str = os.path.join(
            self.output_path, "src", "Org.OpenAPITools", "Model"
        )
        for model_name in sorted(os.listdir(models_directory)):
            model_file_path: str = os.path.join(models_directory, model_name)

            with open(model_file_path, "r") as file:
                content: str = file.read()

            with open(model_file_path, "w") as file:
                file.write(
                    content.replace(
                        "public int CostUnitLimit { get; set; }",
                        "public uint CostUnitLimit { get; set; }",
                    )
                    .replace(
                        "public int TipPercentage { get; set; }",
                        "public uint TipPercentage { get; set; }",
                    )
                    .replace("(int)4294967295", "(uint)4294967295")
                    .replace(
                        "int costUnitLimit = default(int)",
                        "uint costUnitLimit = default(uint)",
                    )
                    .replace(
                        "int tipPercentage = default(int)",
                        "uint tipPercentage = default(uint)",
                    )
                )


def main() -> None:
    # Getting the path of the current running script
    SCRIPT_PATH: str = os.path.dirname(os.path.realpath(__file__))

    # Constants definition
    GENERATOR_INSTALLATION_URL: str = "https://search.maven.org/remotecontent?filepath=org/openapitools/openapi-generator-cli/6.0.1/openapi-generator-cli-6.0.1.jar"
    GENERATOR_FILE_PATH: str = os.path.join(
        SCRIPT_PATH, "openapi-generator-cli-6.0.1.jar"
    )
    SPEC_FILE_PATH: str = os.path.join(
        SCRIPT_PATH, "..", "spec", "transaction-api-spec.yaml"
    )

    # Check if the JAR of the OpenAPI generator is already present or not. If it is not, then we
    # install it from the provided link above
    if not os.path.exists(GENERATOR_FILE_PATH):
        with requests.get(GENERATOR_INSTALLATION_URL) as response:
            with open(GENERATOR_FILE_PATH, "wb") as file:
                file.write(response.content)

    # Creating the OpenAPI model generators for the languages that we wish to create models for
    generators: List[LanguageModelGenerator] = [
        CSharpModelGenerator(
            SPEC_FILE_PATH,
            os.path.join(
                SCRIPT_PATH,
                os.path.join(SCRIPT_PATH, "..", "examples", "dotnet", "Models"),
            ),
        ),
    ]
    list(map(lambda x: x.generate_models(GENERATOR_FILE_PATH), generators))


if __name__ == "__main__":
    main()
