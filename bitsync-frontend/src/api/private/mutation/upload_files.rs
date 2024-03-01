use cynic::{coercions::CoercesTo, schema::NamedType, variables::VariableType};
use serde::ser::SerializeSeq;

use crate::api::schema::private as schema;

#[derive(Clone)]
pub struct UploadFiles(pub Vec<web_sys::File>);

impl CoercesTo<Vec<schema::Upload>> for UploadFiles {}

impl serde::Serialize for UploadFiles {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq_serializer = serializer.serialize_seq(Some(self.0.len()))?;

        for _ in 0..(self.0.len()) {
            seq_serializer.serialize_element(&Option::<u8>::None)?;
        }

        seq_serializer.end()
    }
}

impl schema::variable::Variable for UploadFiles {
    const TYPE: VariableType = VariableType::List(&VariableType::Named(schema::Upload::NAME));
}
