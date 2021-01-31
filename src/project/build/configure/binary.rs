

use super::{ConfigureError, ConfigurePool};



impl ConfigurePool {
    pub fn find_binary(&mut self, bin: String) -> Result<(), Box<dyn std::error::Error>> {
        let l = which::which(&bin);

        if l.is_err() {
            return Err(Box::new(ConfigureError::BinaryFindError { bin }));
        }

        self.executables.insert(bin, l.unwrap());

        Ok(())
    }

    pub fn find_binary_as(
        &mut self,
        bin: String,
        as_name: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let l = which::which(&bin);

        if l.is_err() {
            return Err(Box::new(ConfigureError::BinaryFindError { bin }));
        }

        self.executables.insert(as_name, l.unwrap());

        Ok(())
    }
}
