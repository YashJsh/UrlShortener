use actix_web::{post, web, HttpResponse, Responder};
use diesel::{query_dsl::methods::FilterDsl, ExpressionMethods, RunQueryDsl};
use serde::{Deserialize, Serialize};

// use crate::establish_connection;
use crate::{
    models::url::{NewUrl, Url},
};
use url_shortener::establish_connection;

#[derive(Deserialize)]
pub struct UrlSchema {
    url: String,
}

#[derive(Serialize, Deserialize)]
pub struct ShortUrlResponse {
    original_url: String,
    short_url: String,
}

#[post("/create")]
pub async fn create_short_url(data: web::Json<UrlSchema>) -> impl Responder {
    use crate::schema::urls::dsl::*;

    let conn = &mut establish_connection();
    let url_input = &data.url;

    let existing: Option<Url> = urls
        .filter(original_url.eq(url_input))
        .first::<Url>(conn)
        .ok();
    
    if let Some(found) = existing{
        return HttpResponse::Ok().json(ShortUrlResponse {
            original_url: found.original_url,
            short_url: format!("http://tiny/{}", found.short_url),
        });
    }

    let short = nanoid::nanoid!(7);

    let new_url = NewUrl{
        original_url : url_input,
        short_url : &short
    };

    let _ = diesel::insert_into(urls)
        .values(&new_url)
        .execute(conn);

    HttpResponse::Ok().json(ShortUrlResponse {
        original_url : url_input.clone(),
        short_url : format!("http://tiny/{}", short),
    })
}
