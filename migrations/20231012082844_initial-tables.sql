
create table beer_brand (
  id bigserial primary key,
  name varchar(255) not null,
  city varchar(255) not null
);

-- create table short_urls (
--   id bigserial primary key,
--   domain_id bigint not null,
--   short varchar(255) not null,
--   long varchar(255) not null,
--   constraint fk_domain foreign key (domain_id) references domains(id) on delete cascade,
--   constraint unique_domain_and_short unique (domain_id, short)
-- );


--sqlx migrate add initial-tables
--sqlx migrate run

