use octizys_pretty::types::{Document, Pretty};

pub struct Error<'a> {
    pub error: Document<'a>,
}

fn report_error<'error, E: Into<Error<'error>>>(_error: E) -> () {
    todo!();
}

pub fn report_if_error<'error, A, E: Into<Error<'error>>>(
    value: Result<A, E>,
) -> () {
    match value {
        Ok(_) => (),
        Err(e) => report_error(e),
    }
}

pub fn error_from_pretty<'a, 'b, P: Pretty<'a, 'b>>(e: &'a P) -> Error<'b> {
    Error {
        error: Pretty::to_document(e),
    }
}
