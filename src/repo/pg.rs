extern crate postgres;

use postgres::{Connection, TlsMode};

pub fn connect() -> Connection {
    Connection::connect("postgres://hugo:InRainbows@localhost:5432/hugo", TlsMode::None).unwrap()
}