use std::error::Error;

pub mod add;
pub mod clone;
pub mod info;
pub mod init;
pub mod ls;
pub mod unlink;

pub trait Command {
    fn run(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
