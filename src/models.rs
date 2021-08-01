use super::schema::{users, groups, permission_sets, permissions};
use uuid::Uuid;
use chrono::{DateTime, Utc};

// https://kotiri.com/2018/01/31/postgresql-diesel-rust-types.html

#[derive(Queryable)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub mobile: Option<String>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub mobile: Option<&'a str>,
}

#[derive(AsChangeset)]
#[table_name="users"]
pub struct UpdateUserValues<'a>{
    pub id: Uuid,
    pub email: Option<&'a str>,
    pub first_name: Option<&'a str>,
    pub last_name: Option<&'a str>,
    pub mobile: Option<&'a str>,
}

#[derive(Queryable)]
pub struct UserGroupAssignment {
    pub user_id: Uuid,
    pub group_id: Uuid,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Queryable)]
pub struct Group {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Insertable)]
#[table_name="groups"]
pub struct NewGroup<'a> {
    pub name: &'a str,
    pub description: Option<&'a str>,
}


#[derive(Queryable)]
pub struct GroupPermissionSetAssignment {
    pub group_id: Uuid,
    pub permission_set_id: Uuid,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Queryable)]
pub struct PermissionSet {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Insertable)]
#[table_name="permission_sets"]
pub struct NewPermissionSet<'a> {
    pub name: &'a str,
    pub description: Option<&'a str>,
}

#[derive(Queryable)]
pub struct PermissionSetPermissionAssignment {
    pub permission_set_id: Uuid,
    pub permission_id: Uuid,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Queryable)]
pub struct Permission {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Insertable)]
#[table_name="permissions"]
pub struct NewPermission<'a> {
    pub name: &'a str,
    pub description: Option<&'a str>,
}
