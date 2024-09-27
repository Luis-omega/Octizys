use octizys_pretty::types::{Document, Pretty};

pub struct Error {
    pub error: Document,
}

fn report_error<'error, E: Into<Error>>(_error: E) -> () {
    todo!();
}

pub fn report_if_error<'error, A, E: Into<Error>>(value: Result<A, E>) -> () {
    match value {
        Ok(_) => (),
        Err(e) => report_error(e),
    }
}

pub fn error_from_pretty<P: Pretty>(e: &P) -> Error {
    Error {
        error: Pretty::to_document(e),
    }
}
