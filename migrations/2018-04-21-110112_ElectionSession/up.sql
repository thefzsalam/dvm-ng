-- Your SQL goes here

create table ElectionSession (
    id integer primary key autoincrement,
    name text not null,
    isPresidential BOOLEAN NOT NULL CHECK (isPresidential IN (0,1))
)
