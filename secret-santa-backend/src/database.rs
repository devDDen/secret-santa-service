use crate::models::{User, NewUser, Group, NewGroup, Member, NewMember, Role, Santa, NewSanta};
use diesel::prelude::*;
use diesel::{Connection, PgConnection};
use dotenv::dotenv;
use std::env;
use tide::log;
#[derive(Clone)]
pub struct Database;
impl Database {
    pub fn create_user(&self, username: &str) -> Result<(), diesel::result::Error> {
        DB::create_user(username)?;
        Ok(())
    }

    pub fn create_group_by_user(
        &self,
        username: &str,
        group_name: &str,
    ) -> Result<usize, diesel::result::Error> {
        log::debug!("Creating group {group_name} by user {username}");

        let user = DB::get_user(username)?;
        DB::create_group(group_name)?;
        let group = DB::get_group(group_name)?;
        DB::create_member(&user, &group, Role::Admin)
    }

    pub fn add_user_to_group(
        &self,
        username: &str,
        group_name: &str,
    ) -> Result<usize, diesel::result::Error> {
        log::debug!("Adding user {username} to group {group_name}");

        let user = DB::get_user(username)?;
        let group = DB::get_group(group_name)?;
        match group.is_close {
            true => Err(diesel::result::Error::NotFound),
            false => DB::create_member(&user, &group, Role::Member),
        }
    }
}

struct DB;

impl DB {
    fn connect() -> PgConnection {
        log::debug!("connect enter point");
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        log::debug!("connection established");
        PgConnection::establish(&database_url)
            .expect(&format!("Error connecting to {}", &database_url))
    }

    fn create_user(username: &str) -> Result<usize, diesel::result::Error> {
        log::debug!("Create user {username}");
        let conn = &mut DB::connect();
        let new_user = NewUser { name: username };

        use crate::schema::users::dsl::*;
        log::debug!("User {username} created");
        diesel::insert_into(users).values(&new_user).execute(conn)
    }

    fn update_user(user: &User) -> Result<usize, diesel::result::Error> {
        log::debug!("Update user with id {} to {:?}", user.id, user);
        let conn = &mut DB::connect();

        use crate::schema::users::dsl::*;
        log::debug!("User info updated");
        diesel::update(users.filter(id.eq(user.id)))
            .set(user)
            .execute(conn)
    }

    fn get_user(username: &str) -> Result<User, diesel::result::Error> {
        log::debug!("Try to find user {username}");
        let conn = &mut DB::connect();

        use crate::schema::users::dsl::*;
        users.filter(name.eq(username)).first(conn)
    }

    fn get_users() -> Result<Vec<User>, diesel::result::Error> {
        log::debug!("Get all users");
        let conn = &mut DB::connect();

        use crate::schema::users::dsl::users;
        users.load(conn)
    }

    fn delete_user(user: User) -> Result<usize, diesel::result::Error> {
        log::debug!("Delete user {user:?}");
        let conn = &mut DB::connect();

        use crate::schema::users::dsl::*;
        diesel::delete(users.filter(id.eq(user.id)))
            .execute(conn)
    }

    fn create_group(group_name: &str) -> Result<usize, diesel::result::Error> {
        log::debug!("Create group {}", group_name);
        let conn = &mut DB::connect();
        let new_group = NewGroup { gname: group_name };

        use crate::schema::sgroups::dsl::*;
        diesel::insert_into(sgroups)
            .values(&new_group)
            .execute(conn)
    }

    fn get_group(group_name: &str) -> Result<Group, diesel::result::Error> {
        log::debug!("Try to find group {group_name}");
        let conn = &mut DB::connect();

        use crate::schema::sgroups::dsl::*;
        sgroups.filter(gname.eq(group_name)).first(conn)
    }

    fn get_groups() -> Result<Vec<Group>, diesel::result::Error> {
        log::debug!("Get all groups");
        let conn = &mut DB::connect();

        use crate::schema::sgroups::dsl::sgroups;
        sgroups.load(conn)
    }

    fn update_group(group: &Group) -> Result<usize, diesel::result::Error> {
        log::debug!("Update group with id {} to {:?}", group.id, group);
        let conn = &mut DB::connect();

        use crate::schema::sgroups::dsl::*;
        diesel::update(sgroups.filter(id.eq(group.id)))
            .set(group)
            .execute(conn)
    }

    fn delete_group(group: Group) {
        log::debug!("Delete group {group:?}");
        let conn = &mut DB::connect();

        use crate::schema::sgroups::dsl::*;
        diesel::delete(sgroups.filter(id.eq(group.id)))
            .execute(conn)
            .expect("Error deleting group");
    }

    fn create_member(
        user: &User,
        group: &Group,
        role: Role,
    ) -> Result<usize, diesel::result::Error> {
        log::debug!("Add member {user:?} to group {group:?}");

        let conn = &mut DB::connect();

        let new_group_member = NewMember {
            user_id: user.id,
            group_id: group.id,
            urole: role,
        };
        log::debug!("new group member created");
        use crate::schema::members::dsl::*;
        diesel::insert_into(members)
            .values(new_group_member)
            .execute(conn)
    }

    fn get_member(user: &User, group: &Group) -> Result<Member, diesel::result::Error> {
        log::debug!("Try to find member {user:?} of group {group:?}");
        let conn = &mut DB::connect();

        use crate::schema::members::dsl::*;
        members
            .filter(user_id.eq(user.id))
            .filter(group_id.eq(group.id))
            .first(conn)
    }

    fn update_member(member: Member) -> Result<usize, diesel::result::Error> {
        log::debug!("Update member with id {} to {:?}", member.id, member);
        let conn = &mut DB::connect();

        use crate::schema::members::dsl::*;
        diesel::update(members.filter(id.eq(member.id)))
            .set(member)
            .execute(conn)
    }

    fn count_admins(group: &Group) -> Result<i64, diesel::result::Error> {
        log::debug!("Count admins in group {group:?}");
        let conn = &mut DB::connect();

        use crate::schema::members::dsl::*;
        members
            .filter(group_id.eq(group.id))
            .filter(urole.eq(Role::Admin))
            .count()
            .get_result(conn)
    }

    fn set_santa(
        group: &Group,
        santa: &User,
        recipient: &User,
    ) -> Result<usize, diesel::result::Error> {
        log::debug!("Add santa {santa:?} in group {group:?} to {recipient:?}");
        let conn = &mut DB::connect();

        let new_santa = NewSanta {
            group_id: group.id,
            santa_id: santa.id,
            recipient_id: recipient.id,
        };
        log::debug!("new santa created");
        use crate::schema::santas::dsl::*;
        diesel::insert_into(santas).values(new_santa).execute(conn)
    }

    fn get_santa_recipient(group: &Group, santa: &User) -> Result<User, diesel::result::Error> {
        log::debug!("Get recipient for santa {santa:?} in group {group:?}");
        let conn = &mut DB::connect();

        use crate::schema::santas;
        use crate::schema::users;

        let recitient_id_select: i32 = santas::dsl::santas
            .filter(santas::dsl::group_id.eq(group.id))
            .filter(santas::dsl::santa_id.eq(santa.id))
            .select(santas::dsl::recipient_id)
            .first(conn)?;

        users::dsl::users
            .filter(users::dsl::id.eq(recitient_id_select))
            .first(conn)
    }
}
