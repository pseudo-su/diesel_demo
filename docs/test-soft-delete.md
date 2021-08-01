# Test cascading soft deletes

Cascading soft deletes have been implemented following [this example](https://stackoverflow.com/questions/506432/cascading-soft-delete/53046345#53046345).

Every regular table should have an associated "deleted" table denoted by the name prefix `_deleted_`.

In order to test/validate the soft deletion you can load the `seed_data_iam` and then run some sql scripts.

```sh
cargo run --bin seed_data_iam
```

```sql
DELETE FROM users WHERE email = 'user1@superlegit.business';

-- Validate users get soft deleted by setting deleted_at and getting moved to the _deleted_ table
SELECT * FROM users;
EXPLAIN SELECT * FROM users;

SELECT * FROM users WHERE deleted_at IS NULL;
EXPLAIN SELECT * FROM users WHERE deleted_at IS NULL;

SELECT * FROM _deleted_users;
EXPLAIN SELECT * FROM _deleted_users;

-- Validate that the user_group_memberships for the deleted user was also soft deleted
SELECT * FROM user_group_memberships;
EXPLAIN SELECT * FROM user_group_memberships;

SELECT * FROM user_group_memberships WHERE deleted_at IS NULL;
EXPLAIN SELECT * FROM user_group_memberships WHERE deleted_at IS NULL;

SELECT * FROM _deleted_user_group_memberships;
EXPLAIN SELECT * FROM _deleteduser_group_memberships;
```
