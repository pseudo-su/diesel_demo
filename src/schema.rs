table! {
    group_permission_set_assignments (group_id, permission_set_id) {
        group_id -> Uuid,
        permission_set_id -> Uuid,
    }
}

table! {
    groups (id) {
        id -> Uuid,
        name -> Varchar,
        description -> Nullable<Varchar>,
    }
}

table! {
    permission_set_permission_assignments (permission_set_id, permission_id) {
        permission_set_id -> Uuid,
        permission_id -> Uuid,
    }
}

table! {
    permission_sets (id) {
        id -> Uuid,
        name -> Varchar,
        description -> Nullable<Varchar>,
    }
}

table! {
    permissions (id) {
        id -> Uuid,
        name -> Varchar,
        description -> Nullable<Varchar>,
    }
}

table! {
    user_group_assignments (user_id, group_id) {
        user_id -> Uuid,
        group_id -> Uuid,
    }
}

table! {
    users (id) {
        id -> Uuid,
        email -> Varchar,
        first_name -> Nullable<Varchar>,
        last_name -> Nullable<Varchar>,
        mobile -> Nullable<Varchar>,
    }
}

joinable!(group_permission_set_assignments -> groups (group_id));
joinable!(group_permission_set_assignments -> permission_sets (permission_set_id));
joinable!(permission_set_permission_assignments -> permission_sets (permission_set_id));
joinable!(permission_set_permission_assignments -> permissions (permission_id));
joinable!(user_group_assignments -> groups (group_id));
joinable!(user_group_assignments -> users (user_id));

allow_tables_to_appear_in_same_query!(
    group_permission_set_assignments,
    groups,
    permission_set_permission_assignments,
    permission_sets,
    permissions,
    user_group_assignments,
    users,
);
