extern crate diesel_iam_poc;
extern crate diesel;

use diesel_iam_poc::models::{UpdateUserValues};

use self::diesel_iam_poc::*;

fn main() {

    let connection = establish_connection();

    let users = search_users(&connection);

    for user in users {
        let updated_name = user.first_name.map(|s| { s + "- Updated" } );
        update_user(&connection, UpdateUserValues{
            id: user.id,
            email: None,
            first_name: updated_name.as_deref(),
            last_name: None,
            mobile: None,
        });
    }
    // let user1 = create_user(&connection, NewUser {
    //   email: "user1@superlegit.business",
    //   first_name: "Peter",
    //   last_name: "Rand",
    //   mobile: None,
    // });

    // let user2 = create_user(&connection, NewUser {
    //   email: "user2@superlegit.business",
    //   first_name: "Bob",
    //   last_name: "Smith",
    //   mobile: None,
    // });

    // let user3 = create_user(&connection, NewUser {
    //   email: "user3@superlegit.business",
    //   first_name: "Ian",
    //   last_name: "Kemp",
    //   mobile: None,
    // });
}
