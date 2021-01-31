<<<<<<< HEAD
<<<<<<< HEAD


use super::{ConfigurePool};
=======
use super::ConfigurePool;
>>>>>>> 01f6c2494ac5bd90b98f5797f796ad31592f4cc5
=======
use super::ConfigurePool;
>>>>>>> 01f6c2494ac5bd90b98f5797f796ad31592f4cc5
use crate::project::build::language::Language;

impl ConfigurePool {
    pub fn find_compiler_as(&mut self, com: String, lang: Language) -> Result<(), which::Error> {
        let l = which::which(&com)?;

        self.compilers.insert(lang, l);

        Ok(())
    }
}
