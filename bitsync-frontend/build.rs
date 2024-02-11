static PUBLIC_SCHEMA_FILE_NAME: &str = "public_api_schema.graphql";
static PRIVATE_SCHEMA_FILE_NAME: &str = "private_api_schema.graphql";

fn main() {
    println!("cargo:rerun-if-changed={}", PUBLIC_SCHEMA_FILE_NAME);
    println!("cargo:rerun-if-changed={}", PRIVATE_SCHEMA_FILE_NAME);

    update_schemas_if_outdated();
    register_schemas();
}

fn abort_with_error<E: std::error::Error>(message: &str, error: E) {
    panic!("{}:\n{:#?}", message, error);
}

fn update_schemas_if_outdated() {
    let public_schema = bitsync::public_graphql_schema_string();
    let private_schema = bitsync::private_graphql_schema_string();

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

fn register_schemas() {
    if let Err(e) = cynic_codegen::register_schema("public").from_sdl_file(PUBLIC_SCHEMA_FILE_NAME)
    {
        abort_with_error(
            &format!(
                "Failed to register public schema file `{}`",
                PUBLIC_SCHEMA_FILE_NAME
            ),
            e,
        );
    }

    if let Err(e) =
        cynic_codegen::register_schema("private").from_sdl_file(PRIVATE_SCHEMA_FILE_NAME)
    {
        abort_with_error(
            &format!(
                "Failed to register private schema file `{}`",
                PRIVATE_SCHEMA_FILE_NAME
            ),
            e,
        );
    }
}
