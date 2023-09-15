use postgres::{ Client, NoTls };
use postgres::Error as PostgresError;
use std::net::{ TcpListener, TcpStream };
use std::io::{ Read, Write };
use std::env;

#[macro_use]
extern crate serde_derive;

// create person model
#[derive(Serialize, Deserialize)]
struct Person {
    id: Option<i32>,
    name: String,
    email: String,
}

fn main(){

}