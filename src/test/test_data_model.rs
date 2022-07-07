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


#[derive(Serialize, Deserialize, Debug)]
struct Member {
    nom: String,
    prenom: String,
    date_naissance: String,
    numero_tel: String,
    adresse_mail:String,
    mot_de_passe: String,
    confirmation_mp: String,
    adresse : String
}

#[post("/api/new_member", data="<json>")]
pub fn new_member(json : String)-> String {
    let conn = cnx().unwrap();

    info!("json............: {}", json.as_str());
    let me : Member = serde_json::from_str(json.as_str()).unwrap();

    conn.execute("INSERT INTO member (nom,prenom, date_naissance,numero_tel,adresse_mail,mot_de_passe,confirmation_mp,adresse)\
    VALUES ($1,$2,$3,$4,$5,$6,$7,$8)",
                 &[
                     &me.nom,
                     &me.prenom,
                     &me.date_naissance,
                     &me.numero_tel,
                     &me.adresse_mail,
                     &me.mot_de_passe,
                     &me.confirmation_mp,
                     &me.adresse
                 ]).unwrap();
    serde_json::to_string("ok").unwrap()
}

#[get("/api/member")]
pub fn member() -> String {
    let conn = cnx().unwrap();

    let stmt = conn.prepare("SELECT nom, prenom, date_naissance, numero_tel, adresse_mail, mot_de_passe, confirmation_mp, adresse FROM member").unwrap();
    let mut res = "".to_string();
    let mut json_member_list = "[\n".to_string();


    for row in stmt.query(&[]).unwrap() {
        let person = Member {
            nom: row.get(0),
            prenom: row.get(1),
            date_naissance: row.get(2),
            numero_tel: row.get(3),
            adresse_mail: row.get(4),
            mot_de_passe: row.get(5),
            confirmation_mp: row.get(6),
            adresse: row.get(7),

        };
        json_member_list = format!("{}{},", json_member_list, serde_json::to_string(&person).unwrap());


        res = format!("form : {}\n{}, {}, {}, {}, {}, {}, {}, {}",
                      res, person.nom, person.prenom, person.date_naissance, person.numero_tel, person.adresse_mail, person.mot_de_passe, person.confirmation_mp, person.adresse);

        //serialized_user = format!("{} {} {} {} {} {} {} {}", person.nom, person.prenom, person.date_naissance, person.numero_tel, person.adresse_mail, person.mot_de_passe, person.confirmation_mp, person.adresse);
    };
    //<supprimer le dernier caractère de json_member_list qui est la virgule de trop >;


    if json_member_list.as_str().chars().last() == Some(',') {
        json_member_list.pop();
    }
    json_member_list = format!("{}\n]", json_member_list);
//res
    //return the member list as a json Value

    json_member_list
}