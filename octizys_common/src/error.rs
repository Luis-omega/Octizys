use octizys_pretty::document::Document;

pub struct Error {
    pub error: Document,
}

fn report_error<E: Into<Error>>(_error: E) -> () {
    todo!();
}

pub fn report_if_error<A, E: Into<Error>>(value: Result<A, E>) -> () {
    match value {
        Ok(_) => (),
        Err(e) => report_error(e),
    }
}

pub fn error_from_document<P>(e: &P) -> Error
where
    for<'a> &'a P: Into<Document>,
{
    Error { error: e.into() }
}
