use std::error::Error;

pub mod add;
pub mod cd;
pub mod clone;
pub mod init;
pub mod ls;
pub mod unlink;

pub trait Command {
    fn run(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
