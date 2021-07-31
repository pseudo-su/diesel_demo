#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

use self::models::{
    Group,
    NewGroup,
    NewPermission,
    NewPermissionSet,
    NewUser,
    Permission,
    PermissionSet,
    User,
    UpdateUserValues,
};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn create_user<'a>(conn: &PgConnection, new_user: NewUser) -> User {
    use schema::users;

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
        .expect("Error saving new post")

    // TODO: return Result<Ok, Err>
}

pub fn update_user<'a>(conn: &PgConnection, update_values: UpdateUserValues) -> User {
    use schema::users;
    // https://diesel.rs/guides/all-about-updates.html

    diesel::update(users::table)
        .set(&update_values)
        .get_result(conn)
        .expect("Error saving new user")

    // TODO: return Result<Ok, Err>
}

pub fn create_group<'a>(conn: &PgConnection, new_group: NewGroup) -> Group {
    use schema::groups;

    diesel::insert_into(groups::table)
        .values(&new_group)
        .get_result(conn)
        .expect("Error saving new group")

    // TODO: return Result<Ok, Err>
}

pub fn create_permission_set<'a>(
    conn: &PgConnection,
    new_permission_set: NewPermissionSet,
) -> PermissionSet {
    use schema::permission_sets;

    diesel::insert_into(permission_sets::table)
        .values(&new_permission_set)
        .get_result(conn)
        .expect("Error saving new permission set")

    // TODO: return Result<Ok, Err>
}

pub fn create_permission<'a>(conn: &PgConnection, new_permission: NewPermission) -> Permission {
    use schema::permissions;

    diesel::insert_into(permissions::table)
        .values(&new_permission)
        .get_result(conn)
        .expect("Error saving new permission set")

    // TODO: return Result<Ok, Err>
}

pub fn add_user_to_group<'a>(conn: &PgConnection, user_id: uuid::Uuid, group_id: uuid::Uuid) -> () {
    use schema::user_group_assignments;
    use schema::user_group_assignments::columns;

    diesel::insert_into(user_group_assignments::table)
        .values((columns::user_id.eq(user_id), columns::group_id.eq(group_id)))
        .execute(conn)
        .expect("Error saving assignment");
}

pub fn remove_user_from_group(conn: &PgConnection, user_id: uuid::Uuid, group_id: uuid::Uuid) -> () {
    use schema::user_group_assignments::columns;
    use schema::user_group_assignments::dsl::user_group_assignments;

    diesel::delete(
        user_group_assignments
            .filter(columns::user_id.eq(user_id))
            .filter(columns::group_id.eq(group_id)),
    )
    .execute(conn)
    .expect("Error removing assignment");
}

pub fn add_permission_set_to_group<'a>(conn: &PgConnection, permission_set_id: uuid::Uuid, group_id: uuid::Uuid) -> () {
    use schema::group_permission_set_assignments::columns;
    use schema::group_permission_set_assignments::dsl::group_permission_set_assignments;

    diesel::insert_into(group_permission_set_assignments)
        .values((
            columns::permission_set_id.eq(permission_set_id),
            columns::group_id.eq(group_id),
        ))
        .execute(conn)
        .expect("Error saving assignment");
}

pub fn remove_permission_set_from_group(conn: &PgConnection, permission_set_id: uuid::Uuid, group_id: uuid::Uuid) -> () {
    use schema::group_permission_set_assignments::columns;
    use schema::group_permission_set_assignments::dsl::group_permission_set_assignments;

    diesel::delete(
        group_permission_set_assignments
            .filter(columns::permission_set_id.eq(permission_set_id))
            .filter(columns::group_id.eq(group_id)),
    )
    .execute(conn)
    .expect("Error removing assignment");
}

pub fn add_permission_to_set<'a>(conn: &PgConnection, permission_id: uuid::Uuid, set_id: uuid::Uuid) -> () {
    use schema::permission_set_permission_assignments::columns;
    use schema::permission_set_permission_assignments::dsl::permission_set_permission_assignments;

    diesel::insert_into(permission_set_permission_assignments)
        .values((
            columns::permission_set_id.eq(set_id),
            columns::permission_id.eq(permission_id),
        ))
        .execute(conn)
        .expect("Error saving assignment");
}

pub fn remove_permission_from_set(conn: &PgConnection, permission_id: uuid::Uuid, set_id: uuid::Uuid) -> () {
    use schema::permission_set_permission_assignments::columns;
    use schema::permission_set_permission_assignments::dsl::permission_set_permission_assignments;

    diesel::delete(
        permission_set_permission_assignments
        .filter(columns::permission_set_id.eq(set_id))
        .filter(columns::permission_id.eq(permission_id)),
    )
    .execute(conn)
    .expect("Error removing assignment");
}
