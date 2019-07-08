use actix_web::{client::Client, error, http, web, Error, HttpResponse};
use futures::Future;
use tera::{Context, Tera};

use crate::model;

pub fn fetch_movies_now_playing(
    client: web::Data<Client>,
    api_key: web::Data<model::APIKey>,
    tmpl: web::Data<Tera>,
) -> impl Future<Item = HttpResponse, Error = impl Into<Error>> {
    let tmdb_url = format!(
        "https://api.themoviedb.org/3/movie/now_playing?api_key={}&language=en-US&page=1",
        api_key.get_ref()
    );
    info!("Fetching now playing movies from TMDB API");
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
                let rendered_body = tmpl
                    .render("index.tera", &ctxt)
                    .map_err(|e| error::ErrorInternalServerError(e.description().to_owned()))?;
                Ok(HttpResponse::Ok().body(rendered_body))
            }
            Err(e) => Err(e),
        })
}

pub fn login_view(tmpl: web::Data<Tera>) -> Result<HttpResponse, Error> {
    let rendered_body = tmpl
        .render("login.tera", &Context::new())
        .map_err(|e| error::ErrorInternalServerError(e.description().to_owned()))?;
    Ok(HttpResponse::Ok().body(rendered_body))
}

pub fn signup_view(tmpl: web::Data<Tera>) -> Result<HttpResponse, Error> {
    let rendered_body = tmpl
        .render("signup.tera", &Context::new())
        .map_err(|e| error::ErrorInternalServerError(e.description().to_owned()))?;
    Ok(HttpResponse::Ok().body(rendered_body))
}

pub fn new_user_view(
    pool: web::Data<model::MongoPool>,
    new_user_form: web::Form<model::NewUserForm>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || model::User::create(new_user_form.into_inner(), &pool))
        .from_err()
        .then(move |res| match res {
            Ok(_) => Ok(HttpResponse::Found()
                .header(http::header::LOCATION, "/")
                .finish()),
            Err(e) => Err(e),
        })
}
