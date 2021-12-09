extern crate rocket;
extern crate postgres;

use postgres::{Connection, SslMode};
use postgres::types::ToSql;


pub fn cnx() -> Result<Connection, ::postgres::error::ConnectError> {
    let url = "postgresql://itdoc:itdoc@localhost:5432/sonems";
    //format : postgresql://user:passwd@host:port/db
    Connection::connect(url, &SslMode::None)
}


struct Member {
    nom: String,
    prenom: String,
    date_naissance: String,
    numero_tel: String,
    adresse_mail:String,
    mot_de_passe: String,
    confirmation_mp: String,
    adresse : String,

}


#[get("/member")]
pub fn index() -> String {
    let conn = cnx().unwrap();



    let me = Member {
        nom: "Trump".to_string(),
        prenom: "Donald".to_string(),
        date_naissance: "2009-11-31".to_string(),
        numero_tel: "06.09.88.76.44".to_string(),
        adresse_mail:"boubou@bubu.fr".to_string(),
        mot_de_passe: "*********".to_string(),
        confirmation_mp: "**********".to_string(),
        adresse : "5 rue jean".to_string(),


    };
    conn.execute("INSERT INTO member (nom, prenom, date_naissance, numero_tel, adresse_mail, mot_de_passe, confirmation_mp, adresse)\
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
                 &[&me.nom, &me.prenom, &me.date_naissance, &me.numero_tel, &me.adresse_mail, &me.mot_de_passe, &me.confirmation_mp, &me.adresse]).unwrap();



    let stmt = conn.prepare("SELECT nom, prenom, date_naissance, numero_tel, adresse_mail, mot_de_passe, confirmation_mp, adresse FROM member").unwrap();
    let mut res = "".to_string();
    for row in stmt.query(&[]).unwrap() {
        let person = Member {
            nom: row.get(0),
            prenom:row.get(1),
            date_naissance: row.get(2),
            numero_tel: row.get(3),
            adresse_mail: row.get(4),
            mot_de_passe: row.get(5),
            confirmation_mp: row.get(6),
            adresse: row.get(7),

        };



        res = format!("form : {}\n{}, {}, {}, {}, {}, {}, {}, {}",
                      res,person.nom, person.prenom, person.date_naissance, person.numero_tel, person.adresse_mail, person.mot_de_passe, person.confirmation_mp, person.adresse)
    };

    res

}


struct Produit {
    nom_produit: String,
}


#[get("/produit")]
pub fn produit() -> String {
    let conn = cnx().unwrap();



    let me = Produit {
        nom_produit : "nems porc".to_string(),



    };
    conn.execute("INSERT INTO produit (nom_produit)\
    VALUES ($1)",
                 &[&me.nom_produit]).unwrap();




    let stmt = conn.prepare("SELECT nom_produit FROM produit").unwrap();
    let mut res = "".to_string();
    for row in stmt.query(&[]).unwrap() {
        let prod = Produit {
            nom_produit: row.get(0)

        };



        res = format!("form : {}\n{}",
                      res, prod.nom_produit)
    };

    res

}

struct Annonce {
    date :String,
}

#[get("/annonce")]
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
    for row in stmt.query(&[]).unwrap() {
        let ano = Annonce {
            date: row.get(0),

        };



        res = format!("form : {}\n{}",
                      res,ano.date)
    };

    res

}

struct Cmd_prod{
    produit_id: i32,
    commande_id: i32,
}

#[get("/cmd_prod")]
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
    for row in stmt.query(&[]).unwrap() {
        let cmp = Cmd_prod {
            produit_id: row.get(0),
            commande_id: row.get(1),

        };


        res = format!("form : {}\n{}, {}",
                      res, cmp.produit_id, cmp.commande_id)
    };

    res
}

struct Annonce_prod{
    annonce_id: i32,
    produit_id: i32,
    quantite_annonce: String
}

#[get("/annonce_prod")]
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
    for row in stmt.query(&[]).unwrap() {
        let anop = Annonce_prod {
            annonce_id: row.get(0),
            produit_id: row.get(1),
            quantite_annonce: row.get(2),

        };


        res = format!("form : {}\n{}, {}, {}",
                      res, anop.annonce_id, anop.produit_id, anop.quantite_annonce)
    };

    res
}


struct Commande {
    quantite_cmd: i32,
    member_id: i32,
}

#[get("/commande")]
pub fn commande() -> String {
    let conn = cnx().unwrap();



    let me = Commande {
        quantite_cmd : 20,
        member_id: 1,




    };
    conn.execute("INSERT INTO commande (quantite_cmd, member_id)\
    VALUES ($1, $2)",
                 &[&me.quantite_cmd, &me.member_id]).unwrap();


    ///annonce de tournée de nems avec date et type de nems(porc,crevette, végé, poulet, nutella)

    let stmt = conn.prepare("SELECT quantite_cmd, member_id FROM commande").unwrap();
    let mut res = "".to_string();
    for row in stmt.query(&[]).unwrap() {
        let cmd = Commande {
            quantite_cmd: row.get(0),
            member_id: row.get(1)
        };



        res = format!("form : {}\n{}, {}",
                      res,cmd.quantite_cmd, cmd.member_id)
    };

    res

}