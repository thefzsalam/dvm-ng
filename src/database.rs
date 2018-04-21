extern crate diesel;

pub struct Database {
    connection: diesel::connection::Connection;
}


impl Database {
    pub fn new(connection: diesel::connection::Connection) {
        let db = Database {
            connection
        };
    }


}