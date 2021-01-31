<<<<<<< HEAD
<<<<<<< HEAD


use super::{ConfigureError, ConfigurePool};



=======
use super::{ConfigureError, ConfigurePool};

>>>>>>> 01f6c2494ac5bd90b98f5797f796ad31592f4cc5
=======
use super::{ConfigureError, ConfigurePool};

>>>>>>> 01f6c2494ac5bd90b98f5797f796ad31592f4cc5
impl ConfigurePool {
    pub fn find_binary(&mut self, bin: String) -> Result<(), Box<dyn std::error::Error>> {
        let l = which::which(&bin);

        if l.is_err() {
            return Err(Box::new(ConfigureError::BinaryFindError { bin }));
        }

        self.executables.insert(bin, l.unwrap());

        Ok(())
    }

    #[allow(dead_code)]
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
