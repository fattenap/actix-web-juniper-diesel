use crate::models::users::{NewUser, User};
use crate::schema::users::dsl::*;
use diesel::prelude::*;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

pub fn create_user(conn: &PgConnection, new_user: NewUser) -> QueryResult<User> {
    let user = diesel::insert_into(users)
        .values(&new_user)
        .get_result(conn);

    user
}

pub fn get_user(conn: &PgConnection, identifier: String) -> QueryResult<User> {
    let user = users.find(identifier).first(conn);

    user
}
