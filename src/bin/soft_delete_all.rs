extern crate diesel;
extern crate diesel_iam_poc;

use self::diesel::prelude::*;
use self::diesel_iam_poc::*;

fn main() {
    let connection = establish_connection();
    // Delete users
    {
        use diesel_iam_poc::schema::users::dsl::*;
        diesel::delete(users.filter(deleted_at.is_null()))
            .execute(&connection)
            .expect("Error deleting users");
    }
    // Delete user_groups
    {
        use diesel_iam_poc::schema::user_groups::dsl::*;
        diesel::delete(user_groups.filter(deleted_at.is_null()))
            .execute(&connection)
            .expect("Error deleting user_groups");
    }
    // Delete permission_sets
    {
        use diesel_iam_poc::schema::permission_sets::dsl::*;
        diesel::delete(permission_sets.filter(deleted_at.is_null()))
            .execute(&connection)
            .expect("Error deleting permission_sets");
    }
    // Delete permissions
    {
        use diesel_iam_poc::schema::permissions::dsl::*;
        diesel::delete(permissions.filter(deleted_at.is_null()))
            .execute(&connection)
            .expect("Error deleting permissions");
    }
}
