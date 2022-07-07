extern crate rocket;
extern crate postgres;

use postgres::{Connection, SslMode};
use postgres::types::ToSql;
use serde_json::json;
use serde::{Serialize, Deserialize};
use rocket::form::Form;
use log::{info, warn};
use std::collections::HashMap;


pub fn cnx() -> Result<Connection, ::postgres::error::ConnectError> {
    let url = "postgresql://itdoc:itdoc@localhost:5432/sonems";
    //format : postgresql://user:passwd@host:port/db
    Connection::connect(url, &SslMode::None)
}


#[derive(Serialize, Deserialize, Debug)]
struct Produit {
    produit_id: i32,
    nom_produit: String,
    ingredients: String,
    prix: f64,
}

#[post("/api/new_produit", data="<json>")]
pub fn new_produit(json : String)-> String {
    let conn = cnx().unwrap();

    info!("json............: {}", json.as_str());
    let me : Produit = serde_json::from_str(json.as_str()).unwrap();

    conn.execute("INSERT INTO produit (nom_produit,ingredients, prix)\
    VALUES ($1,$2,$3)",
                 &[
                     &me.nom_produit,
                     &me.ingredients,
                     &me.prix
                 ]).unwrap();
    serde_json::to_string("ok").unwrap()


}


#[get("/api/produit")]
pub fn produit() -> String {
    let conn = cnx().unwrap();

    let stmt = conn.prepare("SELECT produit_id, nom_produit, ingredients, prix FROM produit where prix is not null ORDER BY produit_id").unwrap();
    let mut res = "".to_string();
    let mut json_prod_list = "[\n".to_string();
    for row in stmt.query(&[]).unwrap() {
        let prod = Produit {
            produit_id:row.get(0),
            nom_produit: row.get(1),
            ingredients:row.get(2),
            prix:row.get(3),

        };

        json_prod_list = format!("{}{},", json_prod_list, serde_json::to_string(&prod).unwrap());


        res = format!("form : {} {},{},{}\n{}",
                      res,prod.produit_id, prod.nom_produit, prod.ingredients, prod.prix)
    };
    if json_prod_list.as_str().chars().last() == Some(','){
        json_prod_list.pop();

    }
    json_prod_list = format!("{}\n]", json_prod_list);

    json_prod_list


    //res


}


#[derive(Serialize, Deserialize, Debug)]
struct Annonce {
    date :String,
}

#[get("/api/annonce")]
pub fn annonce() -> String {
    let conn = cnx().unwrap();



    let me = Annonce {
        date : "09/09/09".to_string(),



    };
    conn.execute("INSERT INTO annonce (date)\
    VALUES ($1)",
                 &[&me.date]).unwrap();




    let stmt = conn.prepare("SELECT date FROM annonce").unwrap();
    let mut res = "".to_string();
    let mut json_annonce_list = "[\n".to_string();
    for row in stmt.query(&[]).unwrap() {
        let ano = Annonce {
            date: row.get(0),

        };


        json_annonce_list = format!("{}{},", json_annonce_list, serde_json::to_string(&ano).unwrap());
        res = format!("form : {}\n{}",
                      res,ano.date)
    };

    if json_annonce_list.as_str().chars().last() ==Some(',') {
        json_annonce_list.pop();
    }

    json_annonce_list = format!("{}\n]", json_annonce_list);

    json_annonce_list

    //res

}
#[derive(Serialize, Deserialize, Debug)]
struct Cmd_prod{
    produit_id: i32,
    commande_id: i32,
}

#[get("/api/cmd_prod")]
pub fn cmd_prod() -> String {
    let conn = cnx().unwrap();


    let me = Cmd_prod {
        produit_id: 1,
        commande_id: 1
    };
    conn.execute("INSERT INTO cmd_prod (produit_id, commande_id)\
    VALUES ($1, $2)",
                 &[&me.produit_id, &me.commande_id]).unwrap();




    let stmt = conn.prepare("SELECT produit_id, commande_id FROM cmd_prod").unwrap();
    let mut res = "".to_string();
    let mut json_cmd_prod_list = "[\n".to_string();
    for row in stmt.query(&[]).unwrap() {
        let cmp = Cmd_prod {
            produit_id: row.get(0),
            commande_id: row.get(1),

        };
        json_cmd_prod_list = format!("{}{},", json_cmd_prod_list, serde_json::to_string(&cmp).unwrap());



        //res = format!("form : {}\n{}, {}",
                   //   res, cmp.produit_id, cmp.commande_id)
    };
    if json_cmd_prod_list.as_str().chars().last() == Some(','){
        json_cmd_prod_list.pop();
    }

    json_cmd_prod_list= format!("{}\n]", json_cmd_prod_list);

    json_cmd_prod_list

    //res
}
#[derive(Serialize, Deserialize, Debug)]
struct Annonce_prod{
    annonce_id: i32,
    produit_id: i32,
    quantite_annonce: String
}

#[get("/api/annonce_prod")]
pub fn annonce_prod() -> String {
    let conn = cnx().unwrap();


    let me = Annonce_prod {
        annonce_id: 1,
        produit_id: 1,
        quantite_annonce: "3".to_string()
    };
    conn.execute("INSERT INTO annonce_prod (annonce_id, produit_id, quantite_annonce)\
    VALUES ($1, $2, $3)",
                 &[&me.annonce_id, &me.produit_id, &me.quantite_annonce]).unwrap();




    let stmt = conn.prepare("SELECT annonce_id, produit_id, quantite_annonce FROM annonce_prod").unwrap();
    let mut res = "".to_string();
    let mut json_ano_prod_list = "[\n".to_string();
    for row in stmt.query(&[]).unwrap() {
        let anop = Annonce_prod {
            annonce_id: row.get(0),
            produit_id: row.get(1),
            quantite_annonce: row.get(2),

        };

        json_ano_prod_list = format!("{}{},",json_ano_prod_list, serde_json::to_string(&anop).unwrap());


        res = format!("form : {}\n{}, {}, {}",
                      res, anop.annonce_id, anop.produit_id, anop.quantite_annonce)
    };
    if json_ano_prod_list.as_str().chars().last() ==Some (','){
        json_ano_prod_list.pop();
    }

    json_ano_prod_list =format!("{}\n]", json_ano_prod_list);
    json_ano_prod_list

    //res
}

#[derive(Serialize, Deserialize, Debug)]
struct Commande {
    quantite_cmd: i32,
    member_id: i32,
    items: HashMap<i32,i32>
}
#[post("/api/new_order", data="<json>")]
pub fn new_order (json : String) -> String {
    let conn = cnx().unwrap();

    info!("json............: {}", json.as_str());
    let me : Commande = serde_json::from_str(json.as_str()).unwrap();

    conn.execute("INSERT INTO commande (quantite_cmd, member_id)\
    VALUES ($1, $2)",
                 &[&me.quantite_cmd, &me.member_id]).unwrap();
    serde_json::to_string("ok").unwrap()


}
#[get("/api/commande")]
pub fn commande() -> String {
    let conn = cnx().unwrap();

    let stmt = conn.prepare("SELECT quantite_cmd, member_id FROM commande").unwrap();
    let mut res = "".to_string();
    let mut json_commande_list = "[\n".to_string();
    for row in stmt.query(&[]).unwrap() {
        let cmd = Commande {
            quantite_cmd: row.get(0),
            member_id: row.get(1),
            items: HashMap::new()
        };

        json_commande_list = format!("{}{},",json_commande_list,serde_json::to_string(&cmd).unwrap());



        res = format!("form : {}\n{}, {}",
                      res,cmd.quantite_cmd, cmd.member_id)
    };
    if json_commande_list.as_str().chars().last() == Some(','){
        json_commande_list.pop();
    }
    json_commande_list = format!("{}\n]", json_commande_list);

    json_commande_list

    //res

}