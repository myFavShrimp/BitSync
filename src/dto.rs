#[derive(async_graphql::SimpleObject)]
pub struct File {
    path: String,
    size: u64,
}

impl From<(String, opendal::Metadata)> for File {
    fn from(value: (String, opendal::Metadata)) -> Self {
        Self {
            path: value.0.clone(),
            size: value.1.content_length(),
        }
    }
}
