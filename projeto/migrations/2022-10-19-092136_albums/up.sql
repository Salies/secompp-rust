CREATE TABLE albums (
    id serial primary key,
    artist varchar(64) not null,
    title varchar(64) not null,
    cover_art_url varchar(256) not null,
    label varchar(32) not null,
    release_date date not null
);