extern crate iron;
extern crate router;
extern crate mount;

use iron::prelude::*;
use mount::Mount;

mod user;
mod msg;

fn main() {
    let mut server = Mount::new();

    server.mount("/", msg::get_router());
    server.mount("/user", user::get_router());

    Iron::new(server)
        .http("localhost:3000")
        .expect("Failed to bind address");
}