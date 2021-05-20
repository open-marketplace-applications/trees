use diesel::prelude::*;
use uuid::Uuid;

use crate::models;
use crate::database::PooledConnection;


// -------
// -------
// USERS
// -------
// -------


/// Run query using Diesel to find user by uid and return it.
pub fn find_user_by_uid(
    uid: Uuid,
    conn: &PooledConnection,
) -> Result<Option<models::User>, diesel::result::Error> {
    use crate::schema::users::dsl::*;

    let user = users
        .filter(id.eq(uid.to_string()))
        .first::<models::User>(conn)
        .optional()?;

    Ok(user)
}

/// Run query using Diesel to insert a new database row and return the result.
pub fn insert_new_user(
    // prevent collision with `name` column imported inside the function
    nm: &str,
    conn: &PooledConnection,
) -> Result<models::User, diesel::result::Error> {
    // It is common when using Diesel with Actix web to import schema-related
    // modules inside a function's scope (rather than the normal module's scope)
    // to prevent import collisions and namespace pollution.
    use crate::schema::users::dsl::*;

    let new_user = models::User {
        id: Uuid::new_v4().to_string(),
        name: nm.to_owned(),
    };

    diesel::insert_into(users).values(&new_user).execute(conn)?;

    Ok(new_user)
}



// -------
// -------
// TREES
// -------
// -------

/// Run query using Diesel to find tree by uid and return it.
pub fn find_tree_by_uid(
    uid: Uuid,
    conn: &PooledConnection,
) -> Result<Option<models::Tree>, diesel::result::Error> {
    use crate::schema::trees::dsl::*;

    let tree = trees
        .filter(id.eq(uid.to_string()))
        .first::<models::Tree>(conn)
        .optional()?;

    Ok(tree)
}

/// Run query using Diesel to insert a new database row and return the result.
pub fn insert_new_tree(
    // prevent collision with `name` column imported inside the function
    nm: &str,
    gn: &str,
    lg: &str,
    lt: &str,
    conn: &PooledConnection,
) -> Result<models::Tree, diesel::result::Error> {
    // It is common when using Diesel with Actix web to import schema-related
    // modules inside a function's scope (rather than the normal module's scope)
    // to prevent import collisions and namespace pollution.
    use crate::schema::trees::dsl::*;

    let new_tree = models::Tree {
        id: Uuid::new_v4().to_string(),
        name: nm.to_owned(),
        genus: gn.to_owned(),
        lng: lg.to_owned(),
        lat: lt.to_owned(),
    };

    diesel::insert_into(trees).values(&new_tree).execute(conn)?;

    Ok(new_tree)
}

/// Returns all trees.
pub fn get_all_trees(
    // prevent collision with `name` column imported inside the function
    conn: &PooledConnection,
) -> Result<Vec<models::Tree>, diesel::result::Error> {
    // It is common when using Diesel with Actix web to import schema-related
    // modules inside a function's scope (rather than the normal module's scope)
    // to prevent import collisions and namespace pollution.
    use crate::schema::trees::dsl::*;

    let results = trees
        .load::<models::Tree>(conn)
        .expect("Error loading trees");

    println!("results: {:?}", results);

    Ok(results)
}
