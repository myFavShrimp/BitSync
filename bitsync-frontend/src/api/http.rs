use std::rc::Rc;

use super::{GraphQlResult, GraphQlVariablesHelper};

mod request;
mod response;

use cynic::Operation;
use leptos::SignalGetUntracked;
pub use request::{FileMapper, MultipartGraphqlOperation, RequestError};
pub use response::ResponseError;

pub async fn post_multipart_graphql_operation<T, V>(operation: &Operation<T, V>) -> GraphQlResult<T>
where
    T: for<'de> serde::Deserialize<'de>,
    V: serde::Serialize + Clone + GraphQlVariablesHelper,
{
    let login_token = match V::ADD_LOGIN {
        true => crate::global_storage::use_login_token().0.get_untracked(), // TODO: log
        false => None,
    };

    let multipart_operation = operation.try_into()?;
    let api_response = request::send_form_data(login_token, multipart_operation).await?;
    Ok(response::handle_graphql_response(api_response).await?)
}

impl<T, V> TryFrom<&cynic::Operation<T, V>> for MultipartGraphqlOperation
where
    V: GraphQlVariablesHelper + serde::Serialize,
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
