use iron::prelude::*;
use router::Router;

pub struct MsgHandler {}

pub fn get_router() -> Router {
    let mut router = Router::new();

    router.get("/", list_msg, "list-msg");
    router.post("/", create_msg, "create-msg");

    router
}

fn list_msg(_: &mut Request) -> IronResult<Response> {
    unimplemented!()
}
fn create_msg(_: &mut Request) -> IronResult<Response> {
    unimplemented!()
}
