table! {
    permission_set_grants (user_group_id, user_group_deleted_at, permission_set_code, permission_set_deleted_at) {
        user_group_id -> Uuid,
        user_group_deleted_at -> Timestamptz,
        permission_set_code -> Varchar,
        permission_set_deleted_at -> Timestamptz,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    permission_set_permission_assignments (permission_set_code, permission_set_deleted_at, permission_code, permission_deleted_at) {
        permission_set_code -> Varchar,
        permission_set_deleted_at -> Timestamptz,
        permission_code -> Varchar,
        permission_deleted_at -> Timestamptz,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    permission_sets (code, deleted_at) {
        code -> Varchar,
        description -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Timestamptz,
    }
}

table! {
    permissions (code, deleted_at) {
        code -> Varchar,
        description -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Timestamptz,
    }
}

table! {
    user_group_memberships (user_id, user_deleted_at, user_group_id, user_group_deleted_at) {
        user_id -> Uuid,
        user_deleted_at -> Timestamptz,
        user_group_id -> Uuid,
        user_group_deleted_at -> Timestamptz,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    user_groups (id, deleted_at) {
        id -> Uuid,
        name -> Varchar,
        description -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Timestamptz,
    }
}

table! {
    users (id, deleted_at) {
        id -> Uuid,
        email -> Varchar,
        first_name -> Nullable<Varchar>,
        last_name -> Nullable<Varchar>,
        mobile -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Timestamptz,
    }
}

allow_tables_to_appear_in_same_query!(
    permission_set_grants,
    permission_set_permission_assignments,
    permission_sets,
    permissions,
    user_group_memberships,
    user_groups,
    users,
);
