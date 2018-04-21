-- Your SQL goes here
create table Candidate (
    id integer primary key autoincrement,
    name text not null,
    electionSessionId integer not null,
    constituencyId integer
)
