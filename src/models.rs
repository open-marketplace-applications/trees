use serde::{Deserialize, Serialize};


// USERS
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
use crate::schema::trees;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct Tree {
    pub id: String,
    pub name: String,
    pub genus: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewTree {
    pub name: String,
    pub genus: String,
}
