use diesel::{insert_into, RunQueryDsl, QueryDsl, ExpressionMethods};
use bcrypt::{DEFAULT_COST, hash, verify};
use crate::db::establish_connection;
use crate::schema::users;
use crate::models::{User, NewUser, UserPassword};

pub fn create_user(email: &String, name: &String, password: &String) -> User {
    let conn = &mut establish_connection();

    let new_user = NewUser { email, name, password_hash: &hash(password, DEFAULT_COST).unwrap() };
    insert_into(users::dsl::users).values(&new_user)
        .execute(conn)
        .expect("Error createing new user");
    users::dsl::users.select((users::id, users::name, users::created_at, users::updated_at)).first(conn).expect("Error getting new user")
}

pub fn check_user(email: &String, password: &String) -> Result<UserPassword, diesel::result::Error> {
    let conn = &mut establish_connection();

    let result: Result<UserPassword, diesel::result::Error> = users::dsl::users.select((users::id, users::password_hash))
        .filter(users::email.eq(email))
        .first::<UserPassword>(conn);
    let value = match result {
        Ok(val) => val,
        Err(e) => return Err(e)
    };
    return match verify(password, &value.password_hash) {
        Ok(_) => Ok(value),
        Err(_) => Err(diesel::result::Error::NotFound)
    }
}
