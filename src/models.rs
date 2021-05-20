use serde::{Deserialize, Serialize};


// USERS https://github.com/SakaDream/actix-web-rest-api-with-jwt
use crate::schema::users;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct User {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUser {
    pub name: String,
}

// TREES

// TODO: Enum for tree genus: https://docs.rs/diesel/1.4.5/diesel/deserialize/trait.FromSql.html#impl-FromSql%3CDatetime%2C%20Mysql%3E-for-MYSQL_TIME

use crate::schema::trees;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct Tree {
    pub id: String,
    pub name: String,
    pub genus: String,
    pub lng: String,
    pub lat: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewTree {
    pub name: String,
    pub genus: String,
    pub lng: String,
    pub lat: String
}

