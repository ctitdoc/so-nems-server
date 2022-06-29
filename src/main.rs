#[macro_use]
extern crate rocket;

use test::*;


mod test;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![subscribe,new_subscribe,produit,new_produit, commande, annonce, cmd_prod, annonce_prod])//
}