use diesel::{Connection, PgConnection};

pub fn connect(url:&str) {
    PgConnection::establish(url).unwrap_or_else(|_| panic!());
}