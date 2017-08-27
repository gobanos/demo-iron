use iron::prelude::*;
use iron::status;
use router::Router;

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use msg::MsgHandler;

type UserList = HashMap<String, User>;

pub struct User {
    id: String,
    name: String,
    password: String,
}

impl User {
    fn new(name: &str, password: &str) -> Self {
        User {
            id: User::generate_id(name),
            name: name.into(),
            password: password.into(),
        }
    }

    fn generate_id(name: &str) -> String {
        use std::ascii::AsciiExt;
        name.chars()
            .filter(|c| c.is_ascii() && c.is_alphanumeric())
            .collect::<String>()
            .to_lowercase()
    }
}

struct UserHandler {
    users: UserList,
    msg_handler: MsgHandler,
}

impl UserHandler {
    fn list_user(&self, _: &mut Request) -> IronResult<Response> {
        let mut users = self.users
            .iter()
            .map(|(_id, user)| user)
            .collect::<Vec<_>>();

        users.sort_by(|a, b| a.id.cmp(&b.id));

        let body = users
            .iter()
            .map(|user| user.name.clone() + "\n")
            .collect::<String>();

        Ok(Response::with((status::Ok, body)))
    }

    fn list_user_msg(&self, _: &mut Request) -> IronResult<Response> {
        unimplemented!()
    }

    fn create_user(&mut self, req: &mut Request) -> IronResult<Response> {
        use std::io::Read;

        let mut user_name = String::new();
        itry!(req.body.read_to_string(&mut user_name));

        let new_user = User::new(&user_name, "123456");

        if self.users.contains_key(&new_user.id) {
            Ok(Response::with(status::BadRequest))
        } else {
            self.users.insert(new_user.id.clone(), new_user);
            Ok(Response::with(status::Ok))
        }
    }
}

pub fn get_router() -> Router {
    let mut router = Router::new();

    let handler = UserHandler {
        users: UserList::new(),
        msg_handler: MsgHandler {},
    };

    let handler = Arc::new(RwLock::new(handler));

    {
        let handler = handler.clone();
        router.get(
            "/",
            move |req: &mut Request| {
                handler
                    .read()
                    .expect("UserHandler is poisoned")
                    .list_user(req)
            },
            "list-user",
        );
    }

    {
        let handler = handler.clone();
        router.post(
            "/",
            move |req: &mut Request| {
                handler
                    .write()
                    .expect("UserHandler is poisoned")
                    .create_user(req)
            },
            "create-user",
        );
    }

    {
        let handler = handler.clone();
        router.get(
            "/:id",
            move |req: &mut Request| {
                handler
                    .read()
                    .expect("UserHandler is poisoned")
                    .list_user_msg(req)
            },
            "list-user-msg",
        );
    }

    router
}
