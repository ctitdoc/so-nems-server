#[macro_use]
extern crate rocket;

use test::*;


mod test;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index,produit, commande, annonce, cmd_prod, annonce_prod])//
}