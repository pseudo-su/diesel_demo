table! {
    permission_set_grants (group_id, permission_set_code) {
        group_id -> Uuid,
        permission_set_code -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
    }
}

table! {
    permission_set_permission_assignments (permission_set_code, permission_code) {
        permission_set_code -> Varchar,
        permission_code -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
    }
}

table! {
    permission_sets (code) {
        code -> Varchar,
        description -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
    }
}

table! {
    permissions (code) {
        code -> Varchar,
        description -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
    }
}

table! {
    user_group_memberships (user_id, group_id) {
        user_id -> Uuid,
        group_id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
    }
}

table! {
    user_groups (id) {
        id -> Uuid,
        name -> Varchar,
        description -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
    }
}

table! {
    users (id) {
        id -> Uuid,
        email -> Varchar,
        first_name -> Nullable<Varchar>,
        last_name -> Nullable<Varchar>,
        mobile -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
    }
}

joinable!(permission_set_grants -> permission_sets (permission_set_code));
joinable!(permission_set_grants -> user_groups (group_id));
joinable!(permission_set_permission_assignments -> permission_sets (permission_set_code));
joinable!(permission_set_permission_assignments -> permissions (permission_code));
joinable!(user_group_memberships -> user_groups (group_id));
joinable!(user_group_memberships -> users (user_id));

allow_tables_to_appear_in_same_query!(
    permission_set_grants,
    permission_set_permission_assignments,
    permission_sets,
    permissions,
    user_group_memberships,
    user_groups,
    users,
);
