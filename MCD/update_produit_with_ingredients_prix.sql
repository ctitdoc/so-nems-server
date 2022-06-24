alter table produit
    add ingredients varchar;
alter table produit
    add prix varchar;

-- set no ingredients and default prix to existing products
update produit
set
    ingredients = '',
    prix = 0.80;