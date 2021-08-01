extern crate diesel;
extern crate diesel_iam_poc;

use diesel_iam_poc::models::{NewPermission, NewPermissionSet, NewUser, NewUserGroup};

use self::diesel_iam_poc::*;

fn main() {
    let connection = establish_connection();

    // Users

    let user1 = create_user(
        &connection,
        NewUser {
            email: "user1@superlegit.business",
            first_name: "Peter",
            last_name: "Rand",
            mobile: None,
        },
    );

    let user2 = create_user(
        &connection,
        NewUser {
            email: "user2@superlegit.business",
            first_name: "Bob",
            last_name: "Smith",
            mobile: None,
        },
    );

    let user3 = create_user(
        &connection,
        NewUser {
            email: "user3@superlegit.business",
            first_name: "Ian",
            last_name: "Kemp",
            mobile: None,
        },
    );

    // Groups
    let sysopsteam = create_group(
        &connection,
        NewUserGroup {
            name: "SysOpsTeam",
            description: Some("Systems operations team"),
        },
    );

    let sysopsteam_admins = create_group(
        &connection,
        NewUserGroup {
            name: "SysOpsTeamAdmins",
            description: Some("Systems operations team admins"),
        },
    );

    let bizopsteam = create_group(
        &connection,
        NewUserGroup {
            name: "BizOpsTeam",
            description: Some("Business operations team"),
        },
    );

    let bizopsteam_admins = create_group(
        &connection,
        NewUserGroup {
            name: "BizOpsTeamAdmins",
            description: Some("Business operations team admins"),
        },
    );

    let financeteam = create_group(
        &connection,
        NewUserGroup {
            name: "FinanceTeam",
            description: Some("Finance team"),
        },
    );

    // Users assigned to groups

    add_user_to_group(&connection, user1.id, sysopsteam.id);
    add_user_to_group(&connection, user1.id, sysopsteam_admins.id);

    add_user_to_group(&connection, user2.id, bizopsteam.id);
    add_user_to_group(&connection, user2.id, bizopsteam_admins.id);

    add_user_to_group(&connection, user3.id, financeteam.id);

    // PermissionSets

    let manageiam_adminaccess = create_permission_set(
        &connection,
        NewPermissionSet {
            code: "ManageIAMAdministratorAccess",
            description: Some("Access to perform IAM management administrative functions"),
        },
    );

    let manageiam_viewaccess = create_permission_set(
        &connection,
        NewPermissionSet {
            code: "ManageIAMViewOnlyAccess",
            description: Some("Access to view IAM management information"),
        },
    );

    let managehubs_adminaccess = create_permission_set(
        &connection,
        NewPermissionSet {
            code: "ManageHubsAdministratorAccess",
            description: Some("Access to perform hub management administrative functions"),
        },
    );

    let managehubs_viewaccess = create_permission_set(
        &connection,
        NewPermissionSet {
            code: "ManageHubsViewOnlyAccess",
            description: Some("Access to view hub management information"),
        },
    );

    // PermissionSets assigned to groups

    add_permission_set_to_group(&connection, manageiam_viewaccess.id, sysopsteam.id);
    add_permission_set_to_group(&connection, manageiam_adminaccess.id, sysopsteam_admins.id);

    add_permission_set_to_group(&connection, managehubs_viewaccess.id, bizopsteam.id);
    add_permission_set_to_group(&connection, managehubs_adminaccess.id, bizopsteam_admins.id);
    add_permission_set_to_group(&connection, manageiam_viewaccess.id, bizopsteam_admins.id);

    add_permission_set_to_group(&connection, managehubs_viewaccess.id, financeteam.id);

    // --- Manage Hubs permissions

    let pcreate_hubs = create_permission(
        &connection,
        NewPermission {
            code: "ManageHubs:CreateHubs",
            description: Some("Create new hubs"),
        },
    );

    add_permission_to_set(&connection, pcreate_hubs.id, managehubs_adminaccess.id);

    let pviewhubdetails = create_permission(
        &connection,
        NewPermission {
            code: "ManageHubs:ViewHubDetails",
            description: Some("View hub details"),
        },
    );
    add_permission_to_set(&connection, pviewhubdetails.id, managehubs_adminaccess.id);
    add_permission_to_set(&connection, pviewhubdetails.id, managehubs_viewaccess.id);

    let psearchhubs = create_permission(
        &connection,
        NewPermission {
            code: "ManageHubs:SearchHubs",
            description: Some("Search hubs"),
        },
    );
    add_permission_to_set(&connection, psearchhubs.id, managehubs_adminaccess.id);
    add_permission_to_set(&connection, psearchhubs.id, managehubs_viewaccess.id);

    let pupdatehubs = create_permission(
        &connection,
        NewPermission {
            code: "ManageHubs:UpdateHubDetails",
            description: Some("Update hub details"),
        },
    );
    add_permission_to_set(&connection, pupdatehubs.id, managehubs_adminaccess.id);

    let pdeletehubs = create_permission(
        &connection,
        NewPermission {
            code: "ManageHubs:DeleteHubs",
            description: Some("Delete hubs"),
        },
    );
    add_permission_to_set(&connection, pdeletehubs.id, managehubs_adminaccess.id);
    add_permission_to_set(&connection, pdeletehubs.id, managehubs_viewaccess.id);

    // --- Manage IAM permissions

    // Groups
    let pcreateusergroups = create_permission(
        &connection,
        NewPermission {
            code: "ManageIAM:CreateGroups",
            description: Some("Create IAM user groups"),
        },
    );
    add_permission_to_set(&connection, pcreateusergroups.id, manageiam_adminaccess.id);

    let psearchgroups = create_permission(
        &connection,
        NewPermission {
            code: "ManageIAM:SearchGroups",
            description: Some("Search IAM groups"),
        },
    );
    add_permission_to_set(&connection, psearchgroups.id, manageiam_adminaccess.id);
    add_permission_to_set(&connection, psearchgroups.id, manageiam_viewaccess.id);

    let pupdateusergroups = create_permission(
        &connection,
        NewPermission {
            code: "ManageIAM:UpdateGroupDetails",
            description: Some("Update IAM group details"),
        },
    );
    add_permission_to_set(&connection, pupdateusergroups.id, manageiam_adminaccess.id);

    let pdeleteusergroups = create_permission(
        &connection,
        NewPermission {
            code: "ManageIAM:DeleteGroups",
            description: Some("Delete IAM user groups"),
        },
    );
    add_permission_to_set(&connection, pdeleteusergroups.id, manageiam_adminaccess.id);

    // PermissionSets
    let pcreatepermissionsets = create_permission(
        &connection,
        NewPermission {
            code: "ManageIAM:CreatePermissionSets",
            description: Some("Create IAM PermissionSets"),
        },
    );
    add_permission_to_set(
        &connection,
        pcreatepermissionsets.id,
        manageiam_adminaccess.id,
    );

    // Permissions
    let pcreatepermissions = create_permission(
        &connection,
        NewPermission {
            code: "ManageIAM:CreatePermissions",
            description: Some("Create IAM Permissions"),
        },
    );
    add_permission_to_set(&connection, pcreatepermissions.id, manageiam_adminaccess.id);
}
