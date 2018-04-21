mod IDatabaseTable;
use IDatabaseTable::IDatabaseTable;

pub struct Admin {
    pub id: u64,z
    pub userId: String,
    pub password: String
}
/*
impl IDatabaseTable for Admin {
    pub fn getCreateTableQuery() -> String {
        "CREATE TABLE IF NOT EXIST ADMIN (
            ID          INT     PRIMARY KEY AUTOINCREMENT,
            USERID      TEXT                NOT NULL,
            PASSWORD    TEXT                NOT NULL
         )"
    }
    pub fn getDropTableQuery() -> String {
        "DROP TABLE ADMIN"
    }
    pub fn getDeleteAllQuery() -> String {
        "DELETE FROM ADMIN"
    }

    pub fn getInsertQuery(&self) -> (String, &[&rusqlite::types::ToSql]) {
        (
            "INSERT INTO ADMIN (USERID, PASSWORD) VALUES (?,?)",
            &[&self.userId, &self.password]
        )
    }
    pub fn getUpdateQuery(&self) -> (String, &[&rusqlite::types::ToSql]) {
            (
            "UPDATE ADMIN SET USERID = ?, PASSWORD = ? WHERE ID = ?",
            &[&self.userId, &self.password, &self.id]
        )
    }

    pub fn getDeleteQuery(&self) -> (String, &[&rusqlite::types::ToSql]) {
        (
            "DELETE FROM ADMIN WHERE ID = ?",
            &[&self.id]
        )
    }

}
*/