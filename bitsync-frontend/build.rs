static PUBLIC_SCHEMA_FILE_NAME: &str = "public_api_schema.graphql";
static PRIVATE_SCHEMA_FILE_NAME: &str = "private_api_schema.graphql";

fn main() {
    println!("cargo:rerun-if-changed={}", PUBLIC_SCHEMA_FILE_NAME);
    println!("cargo:rerun-if-changed={}", PRIVATE_SCHEMA_FILE_NAME);

    register_schemas();
}

fn abort_with_error<E: std::error::Error>(message: &str, error: E) {
    panic!("{}:\n{:#?}", message, error);
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
