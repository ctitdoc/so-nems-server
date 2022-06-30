create table subscribe
(
    subscribe_id       serial
        constraint subscribe_id_primary_key_constraint
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

alter table subscribe
    owner to itdoc;
