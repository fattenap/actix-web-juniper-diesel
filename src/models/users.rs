use crate::schema::users;

#[derive(Debug, Serialize, Queryable)]
pub struct User {
    pub id: String,
    pub name: String,
}

#[derive(juniper::GraphQLInputObject, Debug, Insertable)]
#[graphql(description = "A New User")]
#[table_name = "users"]
pub struct NewUser {
    pub id: String,
    pub name: String,
}

#[juniper::object]
/// Information about a User
impl User {
    /// user id
    fn id(&self) -> &str {
        &self.id
    }
    /// user name
    fn name(&self) -> &str {
        &self.name
    }
}
