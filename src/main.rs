#[macro_use]
extern crate rocket;

use test::*;


mod test;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![login_member,member,new_member,produit,new_produit,new_order, commande, annonce, cmd_prod, annonce_prod])//
}