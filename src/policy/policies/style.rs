use crate::project::project::Project;
use inflections::case;

pub const INVALID_NAME: super::super::Policy = super::super::Policy {
    important: false,

    name: "Invalid name",
    description: "Name in camelCase or PascalCase?",
    message: "Invalid name style! Please use PascalCase or camelCase instead.",
    example: super::super::PolicyExample {
        good_examples: &["MyProject", "My Project", "my project"],
        bad_examples: &["Kebab-Generator", "snake_mace"],
        convert_examples: &[&["Kebabs-Yum", "KebabsYum"], &["gCc", "gcc"], &["CATnet", "CatNet"]],
    },

    valid: |p: &Project| -> bool {
        case::is_camel_case(&p.name) || case::is_pascal_case(&p.name)
    },
};
