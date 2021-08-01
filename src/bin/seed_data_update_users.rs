extern crate diesel;
extern crate diesel_iam_poc;

use diesel_iam_poc::models::UpdateUserValues;

use self::diesel_iam_poc::*;

fn main() {
    let connection = establish_connection();

    let users = search_users(&connection);

    for user in users {
        let updated_name = user.first_name.map(|s| s + " - Updated");
        update_user(
            &connection,
            UpdateUserValues {
                id: user.id,
                email: None,
                first_name: updated_name.as_deref(),
                last_name: None,
                mobile: None,
            },
        );
    }
}
