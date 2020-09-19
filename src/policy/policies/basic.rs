use crate::project::project::Project;

pub const EMPTY_NAME: super::super::Policy = super::super::Policy {
    important: true,

    name: "Empty Name",
    description: "Is project name empty?",
    message: "Project name is empty!",
    example: super::super::PolicyExample {
        good_examples: &["CATnet", "Tasty Things", "sally"],
        bad_examples: &[""],
        convert_examples: &[&["name: ", "name: Untitled"], &["name: ", "name: ImToLazyToComeUpWithAName"]],
    },

    valid: |p: &Project| -> bool {
        p.name != "~"
    },
};

pub const EMPTY_DESCRIPTION: super::super::Policy = super::super::Policy {
    important: true,

    name: "Empty Description",
    description: "Is project description empty?",
    message: "Project description is empty!",
    example: super::super::PolicyExample {
        good_examples: &["CATnet", "Tasty Things", "sally"],
        bad_examples: &[""],
        convert_examples: &[&["description: ", "description: not gonna tell ya"], &["description: ", "description: ImToLazyToComeUpWithADescription"]],
    },

    valid: |p: &Project| -> bool {
        p.description != "~"
    },
};

pub const NO_AUTHORS: super::super::Policy = super::super::Policy {
    important: false,

    name: "No authors",
    description: "Are there any authors?",
    message: "No authors!",
    example: super::super::PolicyExample {
        good_examples: &["Milo Banks", "Eli Hatton", "Sally"],
        bad_examples: &[""],
        convert_examples: &[&["authors: []", "authors: [Milo Banks]"], &["authors: []", "authors: [Anonymous]"]],
    },

    valid: |p: &Project| -> bool {
        p.authors.len() != 0
    }
};
