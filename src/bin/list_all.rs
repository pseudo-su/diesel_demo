extern crate diesel;
extern crate diesel_iam_poc;

use self::diesel::prelude::*;
use self::diesel_iam_poc::*;
use self::models::*;

#[macro_use] extern crate prettytable;
use prettytable::{Table, Row, Cell};

fn print_users_table(users: &Vec<User>) {
    let mut table = Table::new();
    table.add_row(row!["FIRST_NAME", "LAST_NAME", "EMAIL", "MOBILE", "CREATED_AT", "UPDATED_AT"]);

    for user in users {
        table.add_row(Row::new(vec![
            // Cell::new(format!("{}", user.id).as_str()),
            Cell::new(user.email.as_str()),
            Cell::new(user.first_name.clone().unwrap_or("-".to_string()).as_str()),
            Cell::new(user.last_name.clone().unwrap_or("-".to_string()).as_str()),
            Cell::new(user.mobile.clone().unwrap_or("-".to_string()).as_str()),
            Cell::new(user.created_at.to_rfc3339().as_str()),
            Cell::new(user.updated_at.to_rfc3339().as_str()),
        ]));
    }

    table.printstd();
}

fn print_groups_table(groups: &Vec<Group>) {
    let mut table = Table::new();
    table.add_row(row!["NAME", "DESCRIPTION", "CREATED_AT", "UPDATED_AT"]);

    for group in groups {
        table.add_row(Row::new(vec![
            // Cell::new(format!("{}", group.id).as_str()),
            Cell::new(group.name.as_str()),
            Cell::new(group.description.clone().unwrap_or("-".to_string()).as_str()),
            Cell::new(group.created_at.to_rfc3339().as_str()),
            Cell::new(group.updated_at.to_rfc3339().as_str()),
        ]));
    }

    table.printstd();
}

fn print_permission_sets_table(permission_sets: &Vec<PermissionSet>) {
    let mut table = Table::new();
    table.add_row(row!["NAME", "DESCRIPTION", "CREATED_AT", "UPDATED_AT"]);

    for permission_set in permission_sets {
        table.add_row(Row::new(vec![
            // Cell::new(format!("{}", permission_set.id).as_str()),
            Cell::new(permission_set.name.as_str()),
            Cell::new(permission_set.description.clone().unwrap_or("-".to_string()).as_str()),
            Cell::new(permission_set.created_at.to_rfc3339().as_str()),
            Cell::new(permission_set.updated_at.to_rfc3339().as_str()),
        ]));
    }

    table.printstd();
}

fn print_permissions_table(permissions: &Vec<Permission>) {
    let mut table = Table::new();
    table.add_row(row!["NAME", "DESCRIPTION", "CREATED_AT", "UPDATED_AT"]);

    for permission in permissions {
        table.add_row(Row::new(vec![
            // Cell::new(format!("{}", permission_set.id).as_str()),
            Cell::new(permission.name.as_str()),
            Cell::new(permission.description.clone().unwrap_or("-".to_string()).as_str()),
            Cell::new(permission.created_at.to_rfc3339().as_str()),
            Cell::new(permission.updated_at.to_rfc3339().as_str()),
        ]));
    }

    table.printstd();
}

fn main() {
    use diesel_iam_poc::schema::users::dsl::*;

    let connection = establish_connection();

    // Users
    let user_results = users
        .load::<User>(&connection)
        .expect("Error loading users");

    println!("");
    println!("-------------------");
    println!("| USERS TABLE ({}) |", user_results.len());
    print_users_table(&user_results);

    // Groups
    use diesel_iam_poc::schema::groups::dsl::*;
    let group_results = groups
        .load::<Group>(&connection)
        .expect("Error loading groups");

    println!("");
    println!("--------------------");
    println!("| GROUPS TABLE ({}) |", group_results.len());
    print_groups_table(&group_results);

    // Permission sets
    use diesel_iam_poc::schema::permission_sets::dsl::*;
    let permission_set_results = permission_sets
        .load::<PermissionSet>(&connection)
        .expect("Error loading groups");


    println!("");
    println!("-----------------------------");
    println!("| PERMISSION_SETS TABLE ({}) |", permission_set_results.len());
    print_permission_sets_table(&permission_set_results);

    // Permission sets
    use diesel_iam_poc::schema::permissions::dsl::*;
    let permissions_results = permissions
        .load::<Permission>(&connection)
        .expect("Error loading groups");


    println!("");
    println!("-------------------------");
    println!("| PERMISSIONS TABLE ({}) |", permissions_results.len());
    print_permissions_table(&permissions_results);
}
