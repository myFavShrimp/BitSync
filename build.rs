static PUBLIC_SCHEMA_FILE_NAME: &str = "public_api_schema.graphql";
static PRIVATE_SCHEMA_FILE_NAME: &str = "private_api_schema.graphql";

fn main() {
    update_schemas_if_outdated();
}

fn abort_with_error<E: std::error::Error>(message: &str, error: E) {
    panic!("{}:\n{:#?}", message, error);
}

fn update_schemas_if_outdated() {
    let public_schema = bitsync_core::public_graphql_schema_string();
    let private_schema = bitsync_core::private_graphql_schema_string();

    let write_public_schema = match std::fs::read_to_string(PUBLIC_SCHEMA_FILE_NAME) {
        Ok(current_public_schema) => current_public_schema != public_schema,
        _ => true,
    };
    let write_private_schema = match std::fs::read_to_string(PRIVATE_SCHEMA_FILE_NAME) {
        Ok(current_private_schema) => current_private_schema != private_schema,
        _ => true,
    };

    if write_public_schema {
        if let Err(e) = std::fs::write(PUBLIC_SCHEMA_FILE_NAME, public_schema) {
            abort_with_error(
                &format!(
                    "Failed to update public schema at `{}`",
                    PUBLIC_SCHEMA_FILE_NAME
                ),
                e,
            );
        };
    };

    if write_private_schema {
        if let Err(e) = std::fs::write(PRIVATE_SCHEMA_FILE_NAME, private_schema) {
            abort_with_error(
                &format!(
                    "Failed to update private schema at `{}`",
                    PRIVATE_SCHEMA_FILE_NAME
                ),
                e,
            )
        };
    };
}
