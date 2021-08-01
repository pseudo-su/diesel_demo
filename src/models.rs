use super::schema::{users, user_groups, permission_sets, permissions};
use uuid::Uuid;
use chrono::{DateTime, Utc};

// https://kotiri.com/2018/01/31/postgresql-diesel-rust-types.html

#[derive(Debug, Queryable)]
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

#[derive(Debug, Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub mobile: Option<&'a str>,
}

#[derive(Debug, Identifiable, AsChangeset)]
#[table_name="users"]
pub struct UpdateUserValues<'a>{
    pub id: Uuid,
    pub email: Option<&'a str>,
    pub first_name: Option<&'a str>,
    pub last_name: Option<&'a str>,
    pub mobile: Option<&'a str>,
}

#[derive(Debug, Queryable)]
pub struct UserGroupMembership {
    pub user_id: Uuid,
    pub group_id: Uuid,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Queryable)]
pub struct UserGroup {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Insertable)]
#[table_name="user_groups"]
pub struct NewUserGroup<'a> {
    pub name: &'a str,
    pub description: Option<&'a str>,
}


#[derive(Debug, Queryable)]
pub struct PermissionSetGrant {
    pub group_id: Uuid,
    pub permission_set_code: String,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Queryable)]
pub struct PermissionSet {
    pub id: Uuid,
    pub code: String,
    pub description: Option<String>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Insertable)]
#[table_name="permission_sets"]
pub struct NewPermissionSet<'a> {
    pub code: &'a str,
    pub description: Option<&'a str>,
}

#[derive(Debug, Queryable)]
pub struct PermissionSetPermissionAssignment {
    pub permission_set_code: String,
    pub permission_code: String,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Queryable)]
pub struct Permission {
    pub id: Uuid,
    pub code: String,
    pub description: Option<String>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Insertable)]
#[table_name="permissions"]
pub struct NewPermission<'a> {
    pub code: &'a str,
    pub description: Option<&'a str>,
}
