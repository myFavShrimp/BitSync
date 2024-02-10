static PUBLIC_SCHEMA_FILE_NAME: &str = "public_api_schema.graphql";
static PRIVATE_SCHEMA_FILE_NAME: &str = "private_api_schema.graphql";

fn main() {
    println!("cargo:rerun-if-changed={}", PUBLIC_SCHEMA_FILE_NAME);
    println!("cargo:rerun-if-changed={}", PRIVATE_SCHEMA_FILE_NAME);

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
        match std::fs::write(PUBLIC_SCHEMA_FILE_NAME, public_schema) {
            Ok(_) => {}
            Err(e) => panic!(
                "Failed to update public schema at `{}`:\n{:#?}",
                PUBLIC_SCHEMA_FILE_NAME, e
            ),
        };
    };

    if write_private_schema {
        match std::fs::write(PRIVATE_SCHEMA_FILE_NAME, private_schema) {
            Ok(_) => {}
            Err(e) => panic!(
                "Failed to update private schema at `{}`:\n{:#?}",
                PRIVATE_SCHEMA_FILE_NAME, e
            ),
        };
    };
}
