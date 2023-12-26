use time::PrimitiveDateTime;

use crate::time::datetime_chrono_to_time;

#[derive(async_graphql::SimpleObject)]
pub struct File {
    path: String,
    size: u64,
    updated_at: PrimitiveDateTime,
}

impl From<(String, opendal::Metadata)> for File {
    fn from(value: (String, opendal::Metadata)) -> Self {
        Self {
            path: value.0.clone(),
            size: value.1.content_length(),
            updated_at: datetime_chrono_to_time(value.1.last_modified().unwrap()),
        }
    }
}
