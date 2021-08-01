# Diesel IAM poc

This is a test project to create an Identity and Access Management (IAM) implementation using rust and diesel.

Features:

- Users and groups use UUID primary keys.
- Soft delete on entities by using `deleted_at` fields
- Automatic timestamps for `created_at` and `updated_at`
