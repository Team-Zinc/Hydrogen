pub struct PolicyExample<'a> {
    pub good_examples: &'a [&'a str],
    pub bad_examples: &'a [&'a str],
    pub convert_examples: &'a [&'a [&'a str]],
}

pub struct Policy<'a> {
    pub important: bool,
    pub name: &'a str,
    pub description: &'a str,
    pub message: &'a str,
    pub example: PolicyExample<'a>,

    pub valid: fn(&crate::Project) -> bool
}

pub mod policies {
    pub mod style;
    pub mod basic;

    // TODO: Replace with macro for maintainability.
    pub const POLICIES: [super::Policy; 4] =
        [style::INVALID_NAME,
            basic::EMPTY_NAME,
            basic::EMPTY_DESCRIPTION,
            basic::NO_AUTHORS];
}
