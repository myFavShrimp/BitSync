use time::PrimitiveDateTime;

use crate::time::datetime_chrono_to_time;

#[derive(async_graphql::SimpleObject)]
pub struct File {
    path: String,
    size: u64,
    updated_at: Option<PrimitiveDateTime>,
}

impl From<(String, opendal::Metadata)> for File {
    fn from(value: (String, opendal::Metadata)) -> Self {
        Self {
            path: value.0.clone(),
            size: value.1.content_length(),
            updated_at: value.1.last_modified().map(datetime_chrono_to_time),
        }
    }
}
