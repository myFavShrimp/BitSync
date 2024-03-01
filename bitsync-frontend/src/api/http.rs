use std::rc::Rc;

use super::GraphQlResult;

mod request;
mod response;

use cynic::Operation;
use leptos::SignalGetUntracked;
pub use request::{FileMapper, MultipartGraphqlOperation, RequestError, WithOptionalFileMapper};
pub use response::ResponseError;

pub async fn post_multipart_graphql_operation<T, V>(operation: &Operation<T, V>) -> GraphQlResult<T>
where
    T: for<'de> serde::Deserialize<'de>,
    V: serde::Serialize + Clone + WithOptionalFileMapper,
{
    let (login_token, _) = crate::global_storage::use_login_token();

    let multipart_operation = operation.try_into()?;
    let api_response =
        request::send_form_data(Some(login_token.get_untracked()), multipart_operation).await?;
    Ok(response::handle_graphql_response(api_response).await?)
}

impl<T, V> TryFrom<&cynic::Operation<T, V>> for MultipartGraphqlOperation
where
    V: WithOptionalFileMapper + serde::Serialize,
{
    type Error = RequestError;

    fn try_from(value: &cynic::Operation<T, V>) -> Result<Self, Self::Error> {
        let operation = serde_json::to_string(value)
            .map_err(Rc::new)
            .map_err(request::FormCreationError::Serialization)?;

        let file_mapper = value.variables.file_mapper();

        Ok(Self {
            operation,
            file_mapper,
        })
    }
}
