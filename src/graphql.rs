use crate::db::PgPool;
use crate::models::users::{NewUser, User};
use crate::services::user::{create_user, get_user};

use juniper::{FieldResult, RootNode};

#[derive(Clone)]
pub struct Context {
    pub db: PgPool,
}

impl juniper::Context for Context {}

pub struct QueryRoot;

#[juniper::object(Context = Context,
    description = "Query Root",)]
impl QueryRoot {
    #[graphql(description = "get a user")]
    fn user(context: &Context, id: String) -> FieldResult<User> {
        let conn = context.db.get()?;
        Ok(get_user(&conn, id).expect("User does not exist"))
    }
}

pub struct MutationRoot;

#[juniper::object(Context = Context,
    description = "Mutation Root",)]
impl MutationRoot {
    #[graphql(description = "create a new user")]
    fn createUser(context: &Context, new_user: NewUser) -> FieldResult<User> {
        let conn = context.db.get()?;
        Ok(create_user(&conn, new_user).expect("Could not create user"))
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}
