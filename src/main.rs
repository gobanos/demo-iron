#[macro_use] extern crate iron;
extern crate mount;
extern crate router;

use iron::prelude::*;
use mount::Mount;

mod user;
mod msg;

fn main() {
    let mut server = Mount::new();

    server.mount("/", msg::get_router());
    server.mount("/user", user::get_router());

    Iron::new(server)
        .http("0.0.0.0:3000")
        .expect("Failed to bind address");
}
