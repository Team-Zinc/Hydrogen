use super::ConfigurePool;
use crate::project::build::language::Language;

impl ConfigurePool {
    pub fn find_compiler_as(&mut self, com: String, lang: Language) -> Result<(), which::Error> {
        let l = which::which(&com)?;

        self.compilers.insert(lang, l);

        Ok(())
    }
}
