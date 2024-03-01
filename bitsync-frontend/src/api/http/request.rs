use std::{collections::HashMap, rc::Rc};

use web_sys::wasm_bindgen::JsValue;

static API_PATH: &str = "http://localhost:8080/api/graphql";

pub struct MultipartGraphqlOperation {
    pub operation: String,
    pub file_mapper: Option<FileMapper>,
}

pub struct FileMapper {
    pub path_prefix: &'static str,
    pub files: Vec<web_sys::File>,
}

impl FileMapper {
    pub fn to_form_data_map(&self) -> HashMap<String, Vec<String>> {
        self.files
            .iter()
            .enumerate()
            .map(|(index, _file)| {
                (
                    index.to_string(),
                    vec![format!("variables.{}.{}", self.path_prefix, index)],
                )
            })
            .collect()
    }
}

pub trait WithOptionalFileMapper {
    fn file_mapper(&self) -> Option<FileMapper> {
        None
    }
}

impl WithOptionalFileMapper for () {}

#[derive(thiserror::Error, Debug, Clone)]
pub enum FormCreationError {
    #[error("Form creation failed")]
    FormCreation(JsValue),
    #[error("Data serialization failed")]
    Serialization(#[from] Rc<serde_json::Error>),
}

static MULTIPART_GRAPHQL_OPERATIONS_KEY: &str = "operations";
static MULTIPART_GRAPHQL_MAP_KEY: &str = "map";

impl TryInto<web_sys::FormData> for MultipartGraphqlOperation {
    type Error = FormCreationError;

    fn try_into(self) -> Result<web_sys::FormData, Self::Error> {
        let form_data = web_sys::FormData::new().map_err(FormCreationError::FormCreation)?;

        form_data
            .set_with_str(MULTIPART_GRAPHQL_OPERATIONS_KEY, &self.operation)
            .map_err(FormCreationError::FormCreation)?;

        let (file_map, files) = self
            .file_mapper
            .map(|mapper| (mapper.to_form_data_map(), mapper.files))
            .unwrap_or_default();

        form_data
            .set_with_str(
                MULTIPART_GRAPHQL_MAP_KEY,
                &serde_json::to_string(&file_map).map_err(Rc::new)?,
            )
            .map_err(FormCreationError::FormCreation)?;

        for (index, file) in files.into_iter().enumerate() {
            form_data
                .set_with_blob_and_filename(&index.to_string(), &file, &file.name())
                .map_err(FormCreationError::FormCreation)?;
        }

        Ok(form_data)
    }
}

#[derive(thiserror::Error, Debug, Clone)]
#[error("Sending the graphql query failed")]
pub enum RequestError {
    FormCreation(#[from] FormCreationError),
    RequestExecution(#[from] Rc<gloo_net::Error>),
}

static API_AUTH_HEADER: &str = "AUTHORIZATION";
static API_AUTH_BEARER_PREFIX: &str = "Bearer";

pub async fn send_form_data(
    login_token: Option<String>,
    multipart_operation: MultipartGraphqlOperation,
) -> Result<gloo_net::http::Response, RequestError> {
    let mut request_builder = gloo_net::http::Request::post(API_PATH);
    if let Some(login_token) = login_token {
        request_builder = request_builder.header(
            API_AUTH_HEADER,
            &format!("{} {}", API_AUTH_BEARER_PREFIX, login_token),
        );
    }

    let form_data: web_sys::FormData = multipart_operation.try_into()?;

    let response = request_builder
        .body(&form_data)
        .map_err(Rc::new)?
        .send()
        .await
        .map_err(Rc::new)?;

    Ok(response)
}
