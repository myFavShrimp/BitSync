use uuid::Uuid;

use crate::api::schema::private as schema;

cynic::impl_scalar!(Uuid, schema::UUID);

#[derive(Clone, Debug, cynic::QueryFragment)]
#[cynic(schema = "private", graphql_type = "User")]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub is_admin: bool,
    pub color_palette: Option<String>,
}

#[derive(cynic::QueryFragment, Debug, Clone)]
#[cynic(schema = "private", graphql_type = "Query")]
pub struct MeQuery {
    pub me: User,
}
