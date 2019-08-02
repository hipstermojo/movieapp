use bson;
use mongodb::coll::results::InsertOneResult;
use mongodb::db::{Database, ThreadedDatabase};
use mongodb::oid::ObjectId;
use mongodb::DecoderError;
use r2d2::{Pool, PooledConnection};
use r2d2_mongodb::MongodbConnectionManager;

use crate::utils;
use crate::utils::HandlerErrors;

pub type APIKey = String;

pub type MongoPool = Pool<MongodbConnectionManager>;
type MongoPooledConnection = PooledConnection<MongodbConnectionManager>;

fn get_db_conn(pool: &MongoPool) -> Result<MongoPooledConnection, &'static str> {
    pool.get().map_err(|_| "Can't get database connection")
}

pub type ImagePath = Option<String>;

#[derive(Debug, Serialize, Deserialize)]
pub struct APIMovieData {
    pub backdrop_path: ImagePath,
    pub genre_ids: Vec<u32>,
    pub id: u32,
    pub original_language: String,
    pub title: String,
    pub overview: String,
    pub poster_path: ImagePath,
    pub release_date: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct APIResponse {
    pub results: Vec<APIMovieData>,
}

#[derive(Debug, Deserialize)]
pub struct NewUserForm {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub _id: ObjectId,
    pub name: String,
    pub email: String,
    pub password: String,
}

impl User {
    // The create method adds user to the database. It raises an error if a user
    // account already exists or if a database operation goes wrong.
    pub fn create(
        new_user: NewUserForm,
        pool: &MongoPool,
    ) -> Result<InsertOneResult, HandlerErrors> {
        let db_conn: &Database = &get_db_conn(pool).unwrap();
        let users_coll = db_conn.collection("users");
        match users_coll.find_one(Some(doc! {"email":&new_user.email}), None) {
            Ok(search_result) => match search_result {
                Some(_) => {
                    return Err(HandlerErrors::ValidationError(utils::ExistingUserError));
                }
                None => {
                    let hashed_password = utils::encrypt_password(&new_user.password);
                    match hashed_password {
                        Ok(encrypted_password) => {
                            return users_coll.insert_one(
                                doc! {"name":new_user.name,"email":new_user.email,"password":encrypted_password},
                                None,
                            ).map_err(|e| HandlerErrors::DatabaseError(e));
                        }
                        Err(_) => return Err(HandlerErrors::HashingError),
                    }
                }
            },
            Err(e) => {
                return Err(HandlerErrors::DatabaseError(e));
            }
        };
    }

    pub fn find_by_email(login_form: LoginForm, pool: &MongoPool) -> Result<User, HandlerErrors> {
        let db_conn: &Database = &get_db_conn(pool).unwrap();
        let users_coll = db_conn.collection("users");
        let search_result = users_coll.find_one(Some(doc! {"email":login_form.email}), None);
        match search_result {
            Ok(result) => match result {
                Some(user_doc) => {
                    let decoded_user: Result<User, DecoderError> =
                        bson::from_bson(bson::Bson::Document(user_doc));
                    match decoded_user {
                        Ok(user) => return Ok(user),
                        Err(e) => return Err(HandlerErrors::DecoderError(e)),
                    };
                }
                None => return Err(HandlerErrors::UserNotExistError),
            },
            Err(e) => return Err(HandlerErrors::DatabaseError(e)),
        };
    }

    pub fn find_by_id(id: &str, pool: &MongoPool) -> Result<User, HandlerErrors> {
        let db_conn: &Database = &get_db_conn(pool).unwrap();
        let users_coll = db_conn.collection("users");
        let obj_id = ObjectId::with_string(id).unwrap();
        match users_coll.find_one(Some(doc! {"_id":obj_id}), None) {
            Ok(search_result) => match search_result {
                Some(user_doc) => {
                    let decoded_user: Result<User, DecoderError> =
                        bson::from_bson(bson::Bson::Document(user_doc));
                    match decoded_user {
                        Ok(user) => return Ok(user),
                        Err(e) => return Err(HandlerErrors::DecoderError(e)),
                    };
                }
                None => return Err(HandlerErrors::UserNotExistError),
            },
            Err(e) => {
                return Err(HandlerErrors::DatabaseError(e));
            }
        }
    }
}
