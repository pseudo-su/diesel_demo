#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

use self::models::{
    UserGroup,
    NewUserGroup,
    NewPermission,
    NewPermissionSet,
    NewUser,
    Permission,
    PermissionSet,
    User,
    UpdateUserValues,
};
use diesel::pg::{PgConnection};
use diesel::{prelude::*};
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn search_users(conn: &PgConnection) -> Vec<User> {

    use schema::users::dsl::users;

    // Users
    let user_results = users
        .load::<User>(conn)
        .expect("Error loading users");

    user_results
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
    update_values.save_changes(conn)
        .expect("Error updating user")
}

pub fn create_group<'a>(conn: &PgConnection, new_group: NewUserGroup) -> UserGroup {
    use schema::user_groups;

    diesel::insert_into(user_groups::table)
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
    use schema::user_group_memberships::{columns, table};

    diesel::insert_into(table)
        .values((columns::user_id.eq(user_id), columns::group_id.eq(group_id)))
        .execute(conn)
        .expect("Error saving assignment");
}

pub fn remove_user_from_group(conn: &PgConnection, user_id: uuid::Uuid, group_id: uuid::Uuid) -> () {
    use schema::user_group_memberships::columns;
    use schema::user_group_memberships::dsl::user_group_memberships;

    diesel::delete(
        user_group_memberships
            .filter(columns::user_id.eq(user_id))
            .filter(columns::group_id.eq(group_id)),
    )
    .execute(conn)
    .expect("Error removing assignment");
}

pub fn add_permission_set_to_group<'a>(conn: &PgConnection, code: &'a str, group_id: uuid::Uuid) -> () {
    use schema::permission_set_grants::columns;
    use schema::permission_set_grants::dsl::permission_set_grants;

    diesel::insert_into(permission_set_grants)
        .values((
            columns::permission_set_code.eq(code),
            columns::group_id.eq(group_id),
        ))
        .execute(conn)
        .expect("Error saving assignment");
}

pub fn remove_permission_set_from_group<'a>(conn: &PgConnection, permission_set_code: &'a str, group_id: uuid::Uuid) -> () {
    use schema::permission_set_grants::columns;
    use schema::permission_set_grants::dsl::permission_set_grants;

    diesel::delete(
        permission_set_grants
            .filter(columns::permission_set_code.eq(permission_set_code))
            .filter(columns::group_id.eq(group_id)),
    )
    .execute(conn)
    .expect("Error removing assignment");
}

pub fn add_permission_to_set<'a>(conn: &PgConnection, permission_code: &'a str, set_code: &'a str) -> () {
    use schema::permission_set_permission_assignments::columns;
    use schema::permission_set_permission_assignments::dsl::permission_set_permission_assignments;

    diesel::insert_into(permission_set_permission_assignments)
        .values((
            columns::permission_set_code.eq(set_code),
            columns::permission_code.eq(permission_code),
        ))
        .execute(conn)
        .expect("Error saving assignment");
}

pub fn remove_permission_from_set<'a>(conn: &PgConnection, permission_code: &'a str, set_code: &'a str) -> () {
    use schema::permission_set_permission_assignments::columns;
    use schema::permission_set_permission_assignments::dsl::permission_set_permission_assignments;

    diesel::delete(
        permission_set_permission_assignments
        .filter(columns::permission_set_code.eq(set_code))
        .filter(columns::permission_code.eq(permission_code)),
    )
    .execute(conn)
    .expect("Error removing assignment");
}
