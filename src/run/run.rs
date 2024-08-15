use crate::error::Error;
use crate::run::configuration::RunConfiguration;

pub struct RunError {}

impl Into<Error> for RunError {
    fn into(self) -> Error {
        todo!()
    }
}

pub fn run(config: RunConfiguration) -> Result<(), RunError> {
    println!("{:?}", config);
    todo!();
}
