use iron::prelude::*;
use router::Router;

pub fn get_router() -> Router {
    let mut router = Router::new();

    router.get("/", list_user, "list-user");
    router.post("/", create_user, "create-user");
    router.get("/:id", list_user_msg, "list-user-msg");

    router
}

fn list_user(_: &mut Request) -> IronResult<Response> { unimplemented!() }
fn create_user(_: &mut Request) -> IronResult<Response> { unimplemented!() }
fn list_user_msg(_: &mut Request) -> IronResult<Response> { unimplemented!() }