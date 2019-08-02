use actix_identity::Identity;
use actix_web::{client::Client, error, http, web, Error, HttpResponse};
use futures::Future;
use tera::{Context, Tera};

use crate::model;
use crate::utils;
use crate::utils::HandlerErrors;

pub fn index_view(
    client: web::Data<Client>,
    api_key: web::Data<model::APIKey>,
    pool: web::Data<model::MongoPool>,
    tmpl: web::Data<Tera>,
    id: Identity,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let tmdb_url = format!(
        "https://api.themoviedb.org/3/movie/now_playing?api_key={}&language=en-US&page=1",
        api_key.get_ref()
    );
    info!("Fetching now playing movies from TMDB API");
    let user_id = id.identity();
    client
        .get(tmdb_url)
        .send()
        .map_err(Error::from)
        .and_then(|mut response| {
            info!("Successfully fetched now playing movies");
            return response.json::<model::APIResponse>().from_err();
        })
        .then(move |body| match body {
            Ok(body) => {
                let mut ctxt = Context::new();
                ctxt.insert("results", &body.results);
                ctxt.insert("is_auth", &id.identity());
                return Ok(ctxt);
            }
            Err(e) => return Err(e),
        })
        .and_then(move |mut ctxt| {
            web::block(move || {
                if let Some(id) = user_id {
                    match model::User::find_by_id(&id, &pool) {
                        Ok(user) => return Ok(Some(user)),
                        Err(e) => return Err(e),
                    };
                } else {
                    return Ok(None);
                }
            })
            .map_err(Error::from)
            .then(move |res| match res {
                Ok(user) => {
                    ctxt.insert("user", &user);
                    let rendered_body = tmpl
                        .render("index.tera", &ctxt)
                        .map_err(|e| error::ErrorInternalServerError(e.description().to_owned()))?;
                    return Ok(HttpResponse::Ok().body(rendered_body));
                }
                Err(e) => return Err(e),
            })
        })
}

pub fn login_view(tmpl: web::Data<Tera>) -> Result<HttpResponse, Error> {
    let rendered_body = tmpl
        .render("login.tera", &Context::new())
        .map_err(|e| error::ErrorInternalServerError(e.description().to_owned()))?;
    Ok(HttpResponse::Ok().body(rendered_body))
}

pub fn login_handler(
    tmpl: web::Data<Tera>,
    pool: web::Data<model::MongoPool>,
    login_form: web::Form<model::LoginForm>,
    id: Identity,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let login_info = login_form.clone();
    web::block(move || model::User::find_by_email(login_form.into_inner(), &pool)).then(
        move |res| {
            match res {
                Ok(user) => match utils::verify_password(&user.password, &login_info.password) {
                    Ok(is_verified) => {
                        if is_verified {
                            id.remember(user._id.to_string());
                            let mut ctxt = Context::new();
                            ctxt.insert("user", &user);
                            let rendered_body =
                                tmpl.render("profile.tera", &ctxt).map_err(|e| {
                                    error::ErrorInternalServerError(e.description().to_owned())
                                })?;
                            return Ok(HttpResponse::Ok().body(rendered_body));
                        } else {
                            return Ok(
                                HttpResponse::BadRequest().body("Incorrect email or password")
                            );
                        }
                    }
                    Err(e) => return Err(error::ErrorInternalServerError(e)),
                },
                Err(e) => return Err(error::ErrorBadRequest(e)),
            };
        },
    )
}

pub fn signup_view(tmpl: web::Data<Tera>) -> Result<HttpResponse, Error> {
    let rendered_body = tmpl
        .render("signup.tera", &Context::new())
        .map_err(|e| error::ErrorInternalServerError(e.description().to_owned()))?;
    Ok(HttpResponse::Ok().body(rendered_body))
}

pub fn new_user_handler(
    pool: web::Data<model::MongoPool>,
    new_user_form: web::Form<model::NewUserForm>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        model::User::create(new_user_form.into_inner(), &pool).map_err(|e| {
            match e {
                // TO DO
                HandlerErrors::HashingError => "Error while hashing".to_owned(),
                HandlerErrors::ValidationError(_) => "User email is already taken".to_owned(),
                HandlerErrors::DatabaseError(err) => err.to_string(),
                _ => "Unexpected error occurred".to_owned(),
            }
        })
    })
    .from_err()
    .then(|res| match res {
        Ok(_) => Ok(HttpResponse::Found()
            .header(http::header::LOCATION, "/login")
            .finish()),
        Err(e) => Err(e),
    })
}
