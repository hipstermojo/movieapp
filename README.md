# movieapp
A simple Actix-web application that uses the TMDb (*T*he *M*ovie *D*atabase) API.
```
This is still unfinished though :P
```


## Prerequisites
- Rust 1.36 or newer
- MongoDB

## Getting started
```sh
git clone https://github.com/hipstermojo/movieapp.git
cd movieapp
MONGO_HOST=XXX\
MONGO_PORT=XXX\
MONGO_DB_NAME=XXX\
HOST=127.0.0.1\
PORT=XXX\
TMDB_API_KEY=XXX cargo run
```
- `MONGO_HOST` - The IP address of the MongoDB database running
- `MONGO_PORT` - The port number of the MongoDB database running
- `MONGO_DB_NAME` - The name of the MongoDB database the application will use
- `HOST` - The IP address of the application
- `PORT` - The port number of the application
- `TMDB_API_KEY` - The API key used to access the TMDb API. To get this API key first create an account on TMDb.

