use crate::error::Error;
use crate::formatter::configuration::FormatterConfiguration;
use crate::pretty::configuration::PrettifierConfiguration;
use crate::pretty::types::SimpleDocument;

pub enum FormatterError {}

impl Into<Error> for FormatterError {
    fn into(self) -> Error {
        todo!()
    }
}

pub fn format_them(
    config: FormatterConfiguration,
) -> Result<(), FormatterError> {
    println!("{:?}", config);
    todo!("formatting");
}
