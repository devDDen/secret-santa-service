use crate::models::{User, NewUser, Group, NewGroup, Member, NewMember, Role, Santa, NewSanta};
use diesel::prelude::*;
use diesel::{Connection, PgConnection};
use dotenv::dotenv;
use std::env;
use tide::log;
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Clone)]
pub struct Database;
impl Database {
    pub fn create_user(&self, username: &str) -> Result<(), diesel::result::Error> {
      let mut db = DB::connect();  
      db.create_user(username)?;
        Ok(())
    }

     pub fn create_group_by_user(
        &self,
        username: &str,
        group_name: &str,
    ) -> Result<usize, diesel::result::Error> {
        log::debug!("Creating group {group_name} by user {username}");

        let mut db = DB::connect();
        let user = db.get_user(username)?;
        db.create_group(group_name)?;
        let group = db.get_group(group_name)?;
        db.create_member(&user, &group, Role::Admin)
    }

    pub fn add_user_to_group(
        &self,
        username: &str,
        group_name: &str,
    ) -> Result<usize, diesel::result::Error> {
        log::debug!("Adding user {username} to group {group_name}");

        let mut db = DB::connect();
        let user = db.get_user(username)?;
        let group = db.get_group(group_name)?;
        match group.is_close {
            true => Err(diesel::result::Error::NotFound),
            false => db.create_member(&user, &group, Role::Member),
        }
    }

    pub fn get_recipient_name(
        &self,
        santa_name: &str,
        group_name: &str,
    ) -> Result<String, diesel::result::Error> {
        log::debug!("Getting recipient for santa {santa_name} in group {group_name}");

        let mut db = DB::connect();
        let santa = db.get_user(santa_name)?;
        let group = db.get_group(group_name)?;
        match group.is_close {
            true => {
                let recipient = db.get_santa_recipient(&group, &santa)?;
                Ok(recipient.name)
            }
            false => Err(diesel::result::Error::NotFound)
        }
    }

    pub fn delete_group_by_admin(
        &self,
        username: &str,
        group_name: &str
    ) -> Result<usize, diesel::result::Error> {
        log::debug!("Deleting group {group_name} by Admin");

        let mut db = DB::connect();
        let user = db.get_user(username)?;
        let group = db.get_group(group_name)?;
        let member = db.get_member(&user, &group)?;
        match member.urole.eq(&Role::Admin) {
            true => {
                let group_for_delete = db.get_group(group_name)?;
                db.delete_group(group_for_delete)
            }
            false => Err(diesel::result::Error::NotFound)
        }
    }

    pub fn close_group(
        &self,
        username: &str,
        group_name: &str,
    ) -> Result<(), diesel::result::Error> {
        log::debug!("Try to start secret Santa by {username} in group {group_name}");

        let mut db = DB::connect();
        let user = db.get_user(username)?;
        let mut group = db.get_group(group_name)?;

        let admin_member = db.get_member(&user, &group)?;
        if admin_member.urole != crate::models::Role::Admin || group.is_close {
            return Err(diesel::result::Error::NotFound);
        }

        let mut members = db.get_members(&group)?;
        if members.len() < 2 {
            return Err(diesel::result::Error::NotFound);
        }
        let mut rng = thread_rng();
        members.shuffle(&mut rng);

        let cur_santa = db.get_user_from_member(members.get(members.len() - 1).unwrap())?;
        let cur_recipient = db.get_user_from_member(members.get(0).unwrap())?;
        db.set_santa(&group, &cur_santa, &cur_recipient)?;

        for i in 0..members.len() - 1 {
            let cur_santa = db.get_user_from_member(members.get(i).unwrap())?;
            let cur_recipient = db.get_user_from_member(members.get(i + 1).unwrap())?;
            db.set_santa(&group, &cur_santa, &cur_recipient)?;            
        }

        group.is_close = true;
        db.update_group(&group)?;

        Ok(())
    }

    pub fn get_group_members(
        &self,
        username: &str,
        group_name: &str,
    ) -> Result<Vec<String>, diesel::result::Error> {
        log::debug!("Getting members of group {group_name} by user {username}");

        let mut db = DB::connect();
        let user = db.get_user(username)?;
        let group = db.get_group(group_name)?;
        let member = db.get_member(&user, &group)?;

        match member.urole {
            Role::Admin => {
                let members = db.get_members(&group)?;

                let mut users = vec![];
                for member in members {
                    let user = db.get_user_from_member(&member)?;
                    users.push(user.name);
                }

                Ok(users)
            }
            Role::Member => Err(diesel::result::Error::NotFound)
        }
    }
    
    pub fn add_admin_to_group(
        &self,
        username: &str,
        new_admin: &str,
        group_name: &str
    ) -> Result<usize, diesel::result::Error> {
        log::debug!("Creating user {new_admin} as admin in group {group_name}");
        let mut db = DB::connect();
        let group = db.get_group(group_name)?;
        let user_setter = db.get_user(username)?;
        let setter_member = db.get_member(&user_setter, &group)?;
        match setter_member.urole.eq(&Role::Admin) {
            true => {
                let user_new_admin = db.get_user(new_admin)?;
                let new_admin_member = db.get_member(&user_new_admin, &group)?;
                let changed_member = new_admin_member.set_role(Role::Admin);
                db.update_member(changed_member) 
            }
            false => Err(diesel::result::Error::NotFound)
        }
    } 
} 

struct DB
{
  conn: PgConnection
}

impl DB {
    fn connect() -> Self {
        log::debug!("Connect enter point");
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        log::debug!("connection established");
        Self {
        conn: PgConnection::establish(&database_url)
            .expect(&format!("Error connecting to {}", &database_url))
        }
    }

    fn create_user(&mut self, username: &str) -> Result<usize, diesel::result::Error> {
        log::debug!("Create user {username}");
        let new_user = NewUser { name: username };

        use crate::schema::users::dsl::*;
        log::debug!("User {username} created");
        diesel::insert_into(users).values(&new_user).execute(&mut self.conn)
    }

     fn update_user(&mut self, user: &User) -> Result<usize, diesel::result::Error> {
        log::debug!("Update user with id {} to {:?}", user.id, user);

        use crate::schema::users::dsl::*;
        log::debug!("User info updated");
        diesel::update(users.filter(id.eq(user.id)))
            .set(user)
            .execute(&mut self.conn)
    }

     fn get_user(&mut self, username: &str) -> Result<User, diesel::result::Error> {
        log::debug!("Try to find user {username}");

        use crate::schema::users::dsl::*;
        users.filter(name.eq(username)).first(&mut self.conn)
    }

    fn get_users(&mut self) -> Result<Vec<User>, diesel::result::Error> {
        log::debug!("Get all users");

        use crate::schema::users::dsl::users;
        users.load(&mut self.conn)
    }

    fn delete_user(&mut self, user: User) -> Result<usize, diesel::result::Error> {
        log::debug!("Delete user {user:?}");

        use crate::schema::users::dsl::*;
        diesel::delete(users.filter(id.eq(user.id)))
            .execute(&mut self.conn)
    }

    fn create_group(&mut self, group_name: &str) -> Result<usize, diesel::result::Error> {
        log::debug!("Create group {}", group_name);
        let new_group = NewGroup { gname: group_name };

        use crate::schema::sgroups::dsl::*;
        diesel::insert_into(sgroups)
            .values(&new_group)
            .execute(&mut self.conn)
    }

    fn get_group(&mut self, group_name: &str) -> Result<Group, diesel::result::Error> {
        log::debug!("Try to find group {group_name}");

        use crate::schema::sgroups::dsl::*;
        sgroups.filter(gname.eq(group_name)).first(&mut self.conn)
    }

    fn get_groups(&mut self) -> Result<Vec<Group>, diesel::result::Error> {
        log::debug!("Get all groups");

        use crate::schema::sgroups::dsl::sgroups;
        sgroups.load(&mut self.conn)
    }

    fn update_group(&mut self, group: &Group) -> Result<usize, diesel::result::Error> {
        log::debug!("Update group with id {} to {:?}", group.id, group);

        use crate::schema::sgroups::dsl::*;
        diesel::update(sgroups.filter(id.eq(group.id)))
            .set(group)
            .execute(&mut self.conn)
    }

    fn delete_group(&mut self, group: Group) -> Result<usize, diesel::result::Error> {
        log::debug!("Delete group {group:?}");

        use crate::schema::sgroups::dsl::*;
        diesel::delete(sgroups.filter(id.eq(group.id)))
            .execute(&mut self.conn)
    }

    fn create_member(
        &mut self,
        user: &User,
        group: &Group,
        role: Role,
    ) -> Result<usize, diesel::result::Error> {
        log::debug!("Add member {user:?} to group {group:?}");

        let new_group_member = NewMember {
            user_id: user.id,
            group_id: group.id,
            urole: role,
        };
        log::debug!("New group member created");
        use crate::schema::members::dsl::*;
        diesel::insert_into(members)
            .values(new_group_member)
            .execute(&mut self.conn)
    }

    fn get_member(&mut self, user: &User, group: &Group) -> Result<Member, diesel::result::Error> {
        log::debug!("Try to find member {user:?} of group {group:?}");

        use crate::schema::members::dsl::*;
        members
            .filter(user_id.eq(user.id))
            .filter(group_id.eq(group.id))
            .first(&mut self.conn)
    }

    fn get_members(&mut self,group: &Group) -> Result<Vec<Member>, diesel::result::Error> {
        log::debug!("Get members of group {group:?}");

        use crate::schema::members::dsl::*;
        members
            .filter(group_id.eq(group.id))
            .load(&mut self.conn)
    }

    fn update_member(&mut self, member: Member) -> Result<usize, diesel::result::Error> {
        log::debug!("Update member with id {} to {:?}", member.id, member);

        use crate::schema::members::dsl::*;
        diesel::update(members.filter(id.eq(member.id)))
            .set(member)
            .execute(&mut self.conn)
    }

    fn count_admins(&mut self, group: &Group) -> Result<i64, diesel::result::Error> {
        log::debug!("Count admins in group {group:?}");
        let conn = &mut DB::connect();

        use crate::schema::members::dsl::*;
        members
            .filter(group_id.eq(group.id))
            .filter(urole.eq(Role::Admin))
            .count()
            .get_result(&mut self.conn)
    }

    fn get_user_from_member(&mut self, member: &Member) -> Result<User, diesel::result::Error> {
        log::debug!("Try to find user from member {member:?}");
        let conn = &mut DB::connect();

        use crate::schema::users::dsl::*;
        users
            .filter(id.eq(member.user_id))
            .first(&mut self.conn)
    }

    fn set_santa(
        &mut self,
        group: &Group,
        santa: &User,
        recipient: &User,
    ) -> Result<usize, diesel::result::Error> {
        log::debug!("Add santa {santa:?} in group {group:?} to {recipient:?}");

        let new_santa = NewSanta {
            group_id: group.id,
            santa_id: santa.id,
            recipient_id: recipient.id,
        };
        log::debug!("New santa created");
        use crate::schema::santas::dsl::*;
        diesel::insert_into(santas).values(new_santa).execute(&mut self.conn)
    }

    fn get_santa_recipient(&mut self, group: &Group, santa: &User) -> Result<User, diesel::result::Error> {
        log::debug!("Get recipient for santa {santa:?} in group {group:?}");

        use crate::schema::santas;
        use crate::schema::users;

        let recitient_id_select: i32 = santas::dsl::santas
            .filter(santas::dsl::group_id.eq(group.id))
            .filter(santas::dsl::santa_id.eq(santa.id))
            .select(santas::dsl::recipient_id)
            .first(&mut self.conn)?;

        users::dsl::users
            .filter(users::dsl::id.eq(recitient_id_select))
            .first(&mut self.conn)
    }
}
