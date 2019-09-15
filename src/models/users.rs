use crate::schema::users;

#[derive(juniper::GraphQLObject, Debug, Serialize, Queryable)]
#[graphql(description = "A User")]
pub struct User {
    pub id: String,
    pub name: String,
}

#[derive(juniper::GraphQLInputObject, Debug, Insertable)]
#[graphql(description = "A new User")]
#[table_name = "users"]
pub struct NewUser {
    pub id: String,
    pub name: String,
}
