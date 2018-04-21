extern crate rusqlite;

pub trait IDatabaseTable {
    pub fn getCreateTableQuery() -> String;
    pub fn getDropTableQuery() -> String;
    pub fn getDeleteAllQuery() -> String;

    pub fn getInsertQuery(&self) -> (String, &[&rusqlite::types::ToSql]);
    pub fn getUpdateQuery(&self) -> (String, &[&rusqlite::types::ToSql]);
    pub fn getDeleteQuery(&self) -> (String, &[&rusqlite::types::ToSql]);

    pub fn getSelectAllQuery(&self) -> (String, Fn(rusqlite::Row,Self)
}