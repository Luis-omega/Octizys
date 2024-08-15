use crate::documentation::configuration::DocumentationConfiguration;
use crate::error::Error;

pub enum DocumentationError {}

impl Into<Error> for DocumentationError {
    fn into(self) -> Error {
        todo!()
    }
}

pub fn generate_documentation(
    config: DocumentationConfiguration,
) -> Result<(), DocumentationError> {
    println!("{:?}", config);
    todo!("generating documentation");
}
