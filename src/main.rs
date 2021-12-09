#[macro_use]
extern crate rocket;
extern crate postgres;

use postgres::{Connection, SslMode};
use postgres::types::ToSql;
mod test;
use test::*;













#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index,produit, commande, annonce, cmd_prod, annonce_prod])//
}