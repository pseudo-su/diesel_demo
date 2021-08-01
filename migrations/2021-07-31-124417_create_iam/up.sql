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

    PRIMARY KEY (id)
);
SELECT auto_manage_updated_at('users');

CREATE TABLE user_groups (
    id uuid DEFAULT uuid_generate_v4() NOT NULL,
    name VARCHAR NOT NULL,
    description VARCHAR,

    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    deleted_at TIMESTAMP WITH TIME ZONE,

    PRIMARY KEY (id)
);
SELECT auto_manage_updated_at('user_groups');

CREATE TABLE permission_sets (
    code VARCHAR NOT NULL,
    description VARCHAR,

    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    deleted_at TIMESTAMP WITH TIME ZONE,

    PRIMARY KEY (code)
);
SELECT auto_manage_updated_at('permission_sets');

CREATE TABLE permissions (
    code VARCHAR NOT NULL,
    description VARCHAR,

    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    deleted_at TIMESTAMP WITH TIME ZONE,

    PRIMARY KEY (code)
);
SELECT auto_manage_updated_at('permissions');

-- Relations

CREATE TABLE user_group_memberships (
    user_id uuid NOT NULL,
    group_id uuid NOT NULL,

    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    deleted_at TIMESTAMP WITH TIME ZONE,

    PRIMARY KEY(user_id, group_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (group_id) REFERENCES user_groups(id) ON DELETE CASCADE
);
SELECT auto_manage_updated_at('user_group_memberships');

CREATE TABLE permission_set_grants (
    group_id uuid NOT NULL,
    permission_set_code VARCHAR NOT NULL,

    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    deleted_at TIMESTAMP WITH TIME ZONE,

    PRIMARY KEY(group_id, permission_set_code),
    FOREIGN KEY (group_id) REFERENCES user_groups(id) ON DELETE CASCADE,
    FOREIGN KEY (permission_set_code) REFERENCES permission_sets(code) ON DELETE CASCADE
);
SELECT auto_manage_updated_at('permission_set_grants');

CREATE TABLE permission_set_permission_assignments (
    permission_set_code VARCHAR NOT NULL,
    permission_code VARCHAR NOT NULL,

    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    deleted_at TIMESTAMP WITH TIME ZONE,

    PRIMARY KEY(permission_set_code, permission_code),
    FOREIGN KEY (permission_set_code) REFERENCES permission_sets(code) ON DELETE CASCADE,
    FOREIGN KEY (permission_code) REFERENCES permissions(code) ON DELETE CASCADE
);
SELECT auto_manage_updated_at('permission_set_permission_assignments');
