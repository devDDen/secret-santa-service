use crate::schema::{members, santas, sgroups, users};
use diesel::prelude::*;
use diesel::sql_types::Integer;
use diesel::{AsChangeset, AsExpression, FromSqlRow, Queryable};
use diesel_enum::DbEnum;

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub name: &'a str,
}

#[derive(Debug, Queryable, AsChangeset)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[table_name = "sgroups"]
pub struct NewGroup<'a> {
    pub gname: &'a str,
}

#[derive(Debug, Queryable, AsChangeset)]
#[table_name = "sgroups"]
pub struct Group {
    pub id: i32,
    pub gname: String,
    pub is_close: bool,
}

impl Group {
    pub fn close_group(self) -> Self {
        Self {
            id: self.id,
            gname: self.gname,
            is_close: true,
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct ConversionError {
    msg: String,
    status: u16,
}

impl ConversionError {
    fn not_found(msg: String) -> Self {
        Self { msg, status: 404 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, AsExpression, FromSqlRow, DbEnum)]
#[sql_type = "Integer"]
#[error_fn = "ConversionError::not_found"]
#[error_type = "ConversionError"]
pub enum Role {
    Member,
    Admin,
}

#[derive(Insertable)]
#[table_name = "members"]
pub struct NewMember {
    pub user_id: i32,
    pub group_id: i32,
    pub urole: Role,
}

#[derive(Debug, Queryable, AsChangeset)]
#[table_name = "members"]
pub struct Member {
    pub id: i32,
    pub user_id: i32,
    pub group_id: i32,
    pub urole: Role,
}

impl Member {
    pub fn set_role(self, new_role: Role) -> Self {
        Self {
            id: self.id,
            user_id: self.user_id,
            group_id: self.group_id,
            urole: new_role,
        }
    }
}

#[derive(Insertable)]
#[table_name = "santas"]
pub struct NewSanta {
    pub group_id: i32,
    pub santa_id: i32,
    pub recipient_id: i32,
}

#[derive(Debug, Queryable, AsChangeset)]
#[table_name = "santas"]
pub struct Santa {
    pub id: i32,
    pub group_id: i32,
    pub santa_id: i32,
    pub recipient_id: i32,
}
