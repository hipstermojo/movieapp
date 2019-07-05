use actix_web::{client::Client, error, web, Error, HttpResponse};
use futures::Future;
use tera::Tera;

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
                let rendered_body = tmpl
                    .render("index.tera", &body)
                    .map_err(|e| error::ErrorInternalServerError(e.description().to_owned()))?;
                Ok(HttpResponse::Ok().body(rendered_body))
            }
            Err(e) => Err(e),
        })
}