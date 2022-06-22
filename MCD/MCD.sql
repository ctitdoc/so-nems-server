create table member
(
    member_id       serial
        constraint id
            primary key,
    prenom          varchar not null,
    date_naissance  varchar not null,
    adresse_mail    varchar not null,
    confirmation_mp varchar not null,
    adresse         varchar not null,
    numero_tel      varchar not null,
    nom             varchar not null,
    mot_de_passe    varchar not null
);

alter table member
    owner to itdoc;

create table commande
(
    commande_id  serial
        constraint commande_pk
            primary key,
    quantite_cmd integer not null,
    member_id    integer not null
        constraint member_commande_fk
            references member
);

alter table commande
    owner to itdoc;

create table produit
(
    produit_id  serial
        constraint produit_id
            primary key,
    nom_produit varchar not null
);

alter table produit
    owner to itdoc;

create table annonce
(
    annonce_id serial
        constraint annonce_id
            primary key,
    date       varchar not null
);

alter table annonce
    owner to itdoc;

create table cmd_prod
(
    produit_id  integer not null
        constraint produit_cmd_prod_fk
            references produit,
    commande_id integer not null
        constraint commande_cmd_prod_fk
            references commande
);

alter table cmd_prod
    owner to itdoc;

create table annonce_prod
(
    annonce_id       integer not null
        constraint annonce_annonce_prod_fk
            references annonce,
    produit_id       integer not null
        constraint produit_annonce_prod_fk
            references produit,
    quantite_annonce varchar not null
);

alter table annonce_prod
    owner to itdoc;

create table subscribe
(
    subscribe_id       serial
        constraint id
            primary key,
    prenom          varchar not null,
    date_naissance  varchar not null,
    adresse_mail    varchar not null,
    confirmation_mp varchar not null,
    adresse         varchar not null,
    numero_tel      varchar not null,
    nom             varchar not null,
    mot_de_passe    varchar not null,
    ville           varchar not null,
    codePostal      varchar not null
);

alter table subscribe
    owner to itdoc;
