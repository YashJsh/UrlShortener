use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::urls;

#[derive(Debug, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = urls)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Url {
    pub id: i32,
    pub original_url: String,
    pub short_url: String,
    pub created_at: chrono::NaiveDateTime,
    pub visits: i32,
}

#[derive(Insertable)]
#[diesel(table_name = urls)]
pub struct NewUrl<'a> {
    pub original_url: &'a str,
    pub short_url: &'a str,
}