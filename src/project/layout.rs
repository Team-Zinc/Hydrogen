/// The layout struct simply describes which files
/// that can be used by Hydrogen are present.
#[derive(Debug)]
pub struct Layout {
    pub has_meta: bool,
    pub has_fetch: bool,
    pub has_dynamic: bool,
}

impl Layout {
    pub fn new() -> Self {
        Self {
            has_meta: false,
            has_fetch: false,
            has_dynamic: false,
        }
    }
}
