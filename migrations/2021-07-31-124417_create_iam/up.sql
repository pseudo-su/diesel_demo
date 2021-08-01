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

CREATE TABLE groups (
    id uuid DEFAULT uuid_generate_v4() NOT NULL,
    name VARCHAR NOT NULL,
    description VARCHAR,

    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    deleted_at TIMESTAMP WITH TIME ZONE,

    PRIMARY KEY (id)
);
SELECT auto_manage_updated_at('groups');

CREATE TABLE permission_sets (
    id uuid DEFAULT uuid_generate_v4() NOT NULL,
    name VARCHAR NOT NULL,
    description VARCHAR,

    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    deleted_at TIMESTAMP WITH TIME ZONE,

    PRIMARY KEY (id)
);
SELECT auto_manage_updated_at('permission_sets');

CREATE TABLE permissions (
    id uuid DEFAULT uuid_generate_v4() NOT NULL,
    name VARCHAR NOT NULL,
    description VARCHAR,

    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    deleted_at TIMESTAMP WITH TIME ZONE,

    PRIMARY KEY (id)
);
SELECT auto_manage_updated_at('permissions');

-- Relations

CREATE TABLE user_group_assignments (
    user_id uuid NOT NULL,
    group_id uuid NOT NULL,

    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    deleted_at TIMESTAMP WITH TIME ZONE,

    PRIMARY KEY(user_id, group_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (group_id) REFERENCES groups(id) ON DELETE CASCADE
);
SELECT auto_manage_updated_at('user_group_assignments');

CREATE TABLE group_permission_set_assignments (
    group_id uuid NOT NULL,
    permission_set_id uuid NOT NULL,

    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    deleted_at TIMESTAMP WITH TIME ZONE,

    PRIMARY KEY(group_id, permission_set_id),
    FOREIGN KEY (group_id) REFERENCES groups(id) ON DELETE CASCADE,
    FOREIGN KEY (permission_set_id) REFERENCES permission_sets(id) ON DELETE CASCADE
);
SELECT auto_manage_updated_at('group_permission_set_assignments');

CREATE TABLE permission_set_permission_assignments (
    permission_set_id uuid NOT NULL,
    permission_id uuid NOT NULL,

    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    deleted_at TIMESTAMP WITH TIME ZONE,

    PRIMARY KEY(permission_set_id, permission_id),
    FOREIGN KEY (permission_set_id) REFERENCES permission_sets(id) ON DELETE CASCADE,
    FOREIGN KEY (permission_id) REFERENCES permissions(id) ON DELETE CASCADE
);
SELECT auto_manage_updated_at('permission_set_permission_assignments');
