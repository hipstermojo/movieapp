use mongodb::coll::results::InsertOneResult;
use mongodb::db::{Database, ThreadedDatabase};
use mongodb::oid::ObjectId;
use mongodb::Error;
use r2d2::{Pool, PooledConnection};
use r2d2_mongodb::MongodbConnectionManager;

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

#[derive(Debug, Serialize)]
pub struct User {
    pub _id: ObjectId,
    pub name: String,
    pub email: String,
    pub password: String,
}

impl User {
    pub fn create(new_user: NewUserForm, pool: &MongoPool) -> Result<InsertOneResult, Error> {
        let db_conn: &Database = &get_db_conn(pool).unwrap();
        let users_coll = db_conn.collection("users");
        users_coll.insert_one(
            doc! {"name":new_user.name,"email":new_user.email,"password":new_user.password},
            None,
        )
    }
}
