
CREATE TABLE produit (
                produit_id INTEGER NOT NULL,
                nom_produit VARCHAR NOT NULL,
                CONSTRAINT id PRIMARY KEY (produit_id)
);


CREATE TABLE annonce (
                annonce_id INTEGER NOT NULL,
                date VARCHAR NOT NULL,
                CONSTRAINT id PRIMARY KEY (annonce_id)
);


CREATE TABLE annonce_prod (
                annonce_id INTEGER NOT NULL,
                produit_id INTEGER NOT NULL,
                quantite_annonce VARCHAR NOT NULL
);


CREATE TABLE member (
                member_id INTEGER NOT NULL,
                prenom VARCHAR NOT NULL,
                date_naissance VARCHAR NOT NULL,
                adresse_mail VARCHAR NOT NULL,
                confirmation_mp VARCHAR NOT NULL,
                adresse VARCHAR NOT NULL,
                numero_tel VARCHAR NOT NULL,
                nom VARCHAR NOT NULL,
                CONSTRAINT id PRIMARY KEY (member_id)
);


CREATE TABLE commande (
                commande_id INTEGER NOT NULL,
                quantit INTEGER NOT NULL,
                member_id INTEGER NOT NULL,
                CONSTRAINT commande_pk PRIMARY KEY (commande_id)
);


CREATE TABLE cmd_prod (
                produit_id INTEGER NOT NULL,
                commande_id INTEGER NOT NULL
);


ALTER TABLE annonce_prod ADD CONSTRAINT produit_annonce_prod_fk
FOREIGN KEY (produit_id)
REFERENCES produit (produit_id)
ON DELETE NO ACTION
ON UPDATE NO ACTION
NOT DEFERRABLE;

ALTER TABLE cmd_prod ADD CONSTRAINT produit_cmd_prod_fk
FOREIGN KEY (produit_id)
REFERENCES produit (produit_id)
ON DELETE NO ACTION
ON UPDATE NO ACTION
NOT DEFERRABLE;

ALTER TABLE annonce_prod ADD CONSTRAINT annonce_annonce_prod_fk
FOREIGN KEY (annonce_id)
REFERENCES annonce (annonce_id)
ON DELETE NO ACTION
ON UPDATE NO ACTION
NOT DEFERRABLE;

ALTER TABLE commande ADD CONSTRAINT member_commande_fk
FOREIGN KEY (member_id)
REFERENCES member (member_id)
ON DELETE NO ACTION
ON UPDATE NO ACTION
NOT DEFERRABLE;

ALTER TABLE cmd_prod ADD CONSTRAINT commande_cmd_prod_fk
FOREIGN KEY (commande_id)
REFERENCES commande (commande_id)
ON DELETE NO ACTION
ON UPDATE NO ACTION
NOT DEFERRABLE;
