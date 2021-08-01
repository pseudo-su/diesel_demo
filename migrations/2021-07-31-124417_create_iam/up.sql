CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Tables

CREATE TABLE users (
    id uuid DEFAULT uuid_generate_v4() NOT NULL,
    email VARCHAR NOT NULL,
    first_name VARCHAR,
    last_name VARCHAR,
    mobile VARCHAR,

    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    deleted_at TIMESTAMP WITH TIME ZONE,

    PRIMARY KEY (id, deleted_at)
);
SELECT diesel_manage_updated_at('users');

CREATE TABLE user_groups (
    id uuid DEFAULT uuid_generate_v4() NOT NULL,
    name VARCHAR NOT NULL,
    description VARCHAR,

    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    deleted_at TIMESTAMP WITH TIME ZONE,

    PRIMARY KEY (id, deleted_at)
);
SELECT diesel_manage_updated_at('user_groups');

CREATE TABLE permission_sets (
    code VARCHAR NOT NULL,
    description VARCHAR,

    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    deleted_at TIMESTAMP WITH TIME ZONE,

    PRIMARY KEY (code, deleted_at)
);
SELECT diesel_manage_updated_at('permission_sets');

CREATE TABLE permissions (
    code VARCHAR NOT NULL,
    description VARCHAR,

    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    deleted_at TIMESTAMP WITH TIME ZONE,

    PRIMARY KEY (code, deleted_at)
);
SELECT diesel_manage_updated_at('permissions');

-- Relations

CREATE TABLE user_group_memberships (
    user_id uuid NOT NULL,
    user_deleted_at TIMESTAMP WITH TIME ZONE,
    user_group_id uuid NOT NULL,
    user_group_deleted_at TIMESTAMP WITH TIME ZONE,

    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,

    PRIMARY KEY(user_id, user_deleted_at, user_group_id, user_group_deleted_at),
    FOREIGN KEY (user_id, user_deleted_at) REFERENCES users(id, deleted_at) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (user_group_id, user_group_deleted_at) REFERENCES user_groups(id, deleted_at) ON DELETE CASCADE ON UPDATE CASCADE
);
SELECT diesel_manage_updated_at('user_group_memberships');

CREATE TABLE permission_set_grants (
    user_group_id uuid NOT NULL,
    user_group_deleted_at TIMESTAMP WITH TIME ZONE,
    permission_set_code VARCHAR NOT NULL,
    permission_set_deleted_at TIMESTAMP WITH TIME ZONE,

    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,

    PRIMARY KEY(user_group_id, user_group_deleted_at, permission_set_code, permission_set_deleted_at),

    FOREIGN KEY (user_group_id, user_group_deleted_at) REFERENCES user_groups(id, deleted_at) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (permission_set_code, permission_set_deleted_at) REFERENCES permission_sets(code, deleted_at) ON DELETE CASCADE ON UPDATE CASCADE
);
SELECT diesel_manage_updated_at('permission_set_grants');

CREATE TABLE permission_set_permission_assignments (
    permission_set_code VARCHAR NOT NULL,
    permission_set_deleted_at TIMESTAMP WITH TIME ZONE,
    permission_code VARCHAR NOT NULL,
    permission_deleted_at TIMESTAMP WITH TIME ZONE,

    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,

    PRIMARY KEY(permission_set_code, permission_set_deleted_at, permission_code, permission_deleted_at),
    FOREIGN KEY (permission_set_code, permission_set_deleted_at) REFERENCES permission_sets(code, deleted_at) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (permission_code, permission_deleted_at) REFERENCES permissions(code, deleted_at) ON DELETE CASCADE ON UPDATE CASCADE
);
SELECT diesel_manage_updated_at('permission_set_permission_assignments');
