use octizys_pretty::document::Document;

pub trait ToDocument<Configuration> {
    fn to_document(&self, configuration: &Configuration) -> Document;
}

impl<T, Configuration> ToDocument<Configuration> for Box<T>
where
    T: ToDocument<Configuration>,
{
    fn to_document(&self, configuration: &Configuration) -> Document {
        (**self).to_document(configuration)
    }
}
