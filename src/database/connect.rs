use diesel::{Connection, PgConnection};

pub fn connect() {
    PgConnection::establish("0.0.0.0:4444").unwrap_or_else(|_| panic!());
}