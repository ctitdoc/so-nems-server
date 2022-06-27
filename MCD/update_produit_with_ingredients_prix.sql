alter table produit
    add ingredients varchar;
alter table produit
    add prix float8;

-- set no ingredients and default prix to existing products
update produit
set
    ingredients = '',
    prix = 0.80;