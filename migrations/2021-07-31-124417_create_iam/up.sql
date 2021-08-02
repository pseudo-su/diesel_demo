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
    deleted_at TIMESTAMP WITH TIME ZONE NULL,

    PRIMARY KEY (id)
);
ALTER TABLE users ALTER COLUMN deleted_at DROP NOT NULL;

SELECT manage_auto_updated_at('users');
SELECT manage_auto_soft_delete('users');

CREATE TABLE user_groups (
    id uuid DEFAULT uuid_generate_v4() NOT NULL,
    name VARCHAR NOT NULL,
    description VARCHAR,

    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    deleted_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,

    PRIMARY KEY (id)
);
SELECT manage_auto_updated_at('user_groups');
SELECT manage_auto_soft_delete('user_groups');

CREATE TABLE permission_sets (
    id uuid DEFAULT uuid_generate_v4() NOT NULL,
    code VARCHAR NOT NULL,
    description VARCHAR,

    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    deleted_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,

    PRIMARY KEY (id),
    UNIQUE(code)
);
SELECT manage_auto_updated_at('permission_sets');
SELECT manage_auto_soft_delete('permission_sets');

CREATE TABLE permissions (
    id uuid DEFAULT uuid_generate_v4() NOT NULL,
    code VARCHAR NOT NULL,
    description VARCHAR,

    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    deleted_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,

    PRIMARY KEY (id),
    UNIQUE(code)
);
SELECT manage_auto_updated_at('permissions');
SELECT manage_auto_soft_delete('permissions');

-- Relations

CREATE TABLE user_group_memberships (
    id uuid DEFAULT uuid_generate_v4() NOT NULL,

    user_id uuid NOT NULL,
    user_group_id uuid NOT NULL,

    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    deleted_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,

    PRIMARY KEY(id),
    UNIQUE(user_id, user_group_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (user_group_id) REFERENCES user_groups(id) ON DELETE CASCADE
);
SELECT manage_auto_updated_at('user_group_memberships');
SELECT manage_auto_soft_delete('user_group_memberships');

CREATE TABLE permission_set_grants (
    id uuid DEFAULT uuid_generate_v4() NOT NULL,

    user_group_id uuid NOT NULL,
    permission_set_id uuid NOT NULL,

    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    deleted_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,

    PRIMARY KEY(id),
    UNIQUE(user_group_id, permission_set_id),
    FOREIGN KEY (user_group_id) REFERENCES user_groups(id) ON DELETE CASCADE,
    FOREIGN KEY (permission_set_id) REFERENCES permission_sets(id) ON DELETE CASCADE
);
SELECT manage_auto_updated_at('permission_set_grants');
SELECT manage_auto_soft_delete('permission_set_grants');

CREATE TABLE permission_set_permission_assignments (
    id uuid DEFAULT uuid_generate_v4() NOT NULL,
    permission_set_id uuid NOT NULL,
    permission_id uuid NOT NULL,

    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    deleted_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,

    PRIMARY KEY(id),
    UNIQUE(permission_set_id, permission_id),
    FOREIGN KEY (permission_set_id) REFERENCES permission_sets(id) ON DELETE CASCADE,
    FOREIGN KEY (permission_id) REFERENCES permissions(id) ON DELETE CASCADE
);
SELECT manage_auto_updated_at('permission_set_permission_assignments');
SELECT manage_auto_soft_delete('permission_set_permission_assignments');
