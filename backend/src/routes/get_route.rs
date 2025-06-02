use actix_web::{get, web, HttpResponse, Responder};
use diesel::{query_dsl::methods::FilterDsl, ExpressionMethods, RunQueryDsl};
use serde::{Deserialize, Serialize};
use url_shortener::establish_connection;

use crate::{
    models::url::{NewUrl, Url},
};

#[get("/{path}")]
async fn get(path : web::Path<String>)-> impl Responder{
    use crate::schema::urls::dsl::*;

    let conn = &mut establish_connection();
    let short = path.into_inner();

    match urls.filter(short_url.eq(short)).first::<Url>(conn) {
        Ok(result) => {
            HttpResponse::Found()
                .append_header(("Location", result.original_url))
                .finish()
        }
        Err(_) => HttpResponse::NotFound().body("Short URL not found."),
    }
}
