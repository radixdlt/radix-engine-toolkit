import caseconverter
import json


def main() -> None:
    with open('file.json', 'r') as file:
        content: dict[str, list[str]] = json.load(file)

    string: str = ""
    string += "define_instruction_visitor! {\n"
    string += "    pub trait InstructionVisitor {\n"

    for instruction_name, fields in content.items():
        snake_case_instruction_name: str = caseconverter.snakecase(instruction_name)
        string += f"        visit_{snake_case_instruction_name}(\n"
        for field in fields:
            snake_case_field_name: str = caseconverter.snakecase(field)
            string += f"            _{snake_case_field_name}: &mut crate::model::value::ast::ManifestAstValue,\n"
        string += "        ),\n"

    string += "    }\n"
    string += "}\n"

    string += """/// A function which traverses [`Instruction`]s calling the value visitors first and then calling
/// the instruction visitors
pub fn traverse_instruction(
    instruction: &mut Instruction,
    value_visitors: &mut [&mut dyn ManifestAstValueVisitor],
    instructions_visitors: &mut [&mut dyn InstructionVisitor],
) -> Result<(), VisitorError> {
    match instruction {"""

    for instruction_name, fields in content.items():
        pascal_case_instruction_name: str = caseconverter.pascalcase(instruction_name)
        snake_case_instruction_name: str = caseconverter.snakecase(instruction_name)
        string += f"        Instruction::{pascal_case_instruction_name} {{\n"
        for field in fields:
            snake_case_field_name: str = caseconverter.snakecase(field)
            string += f"            {snake_case_field_name},\n"
        string += "        } => {\n"

        for field in fields:
            snake_case_field_name: str = caseconverter.snakecase(field)
            string += f"            traverse_value({snake_case_field_name}, value_visitors)?;\n"
        
        string += "            visit!(\n"
        string += "                instructions_visitors,\n"
        string += f"                visit_{snake_case_instruction_name},\n"
        for field in fields:
            snake_case_field_name: str = caseconverter.snakecase(field)
            string += f"                {snake_case_field_name},\n"
        string += "            )?;\n"
    
        string += "        }\n"
    
    string += """    };
    visit!(instructions_visitors, post_visit,)?;
    Ok(())
}"""

    print(string)

if __name__ == "__main__":
    main()