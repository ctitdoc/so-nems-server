
use crate::test::test_data_model::cnx;
use serde_json::json;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
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
    let mut json_member_list = "[\n".to_string();

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
        json_member_list = format!("{},{}\n", json_member_list, serde_json::to_string(&person).unwrap());



        res = format!("form : {}\n{}, {}, {}, {}, {}, {}, {}, {}",
                      res,person.nom, person.prenom, person.date_naissance, person.numero_tel, person.adresse_mail, person.mot_de_passe, person.confirmation_mp, person.adresse);

        //serialized_user = format!("{} {} {} {} {} {} {} {}", person.nom, person.prenom, person.date_naissance, person.numero_tel, person.adresse_mail, person.mot_de_passe, person.confirmation_mp, person.adresse);

    };
    json_member_list = format!("{}}}", json_member_list);


//res
    //return the member list as a json Value

    json_member_list







}