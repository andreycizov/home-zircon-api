#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

use rocket::{routes, get, post, Outcome, Responder};
use rocket::http::Status;
use rocket::http::hyper::header::Authorization;
use rocket::{State, Request};
use rocket::request::{self, FromRequest};
use rocket_contrib::json::Json;
use serde::{Serialize, Deserialize};
use std::collections::{HashSet, HashMap};
use std::sync::RwLock;
use rocket::response::status;
use std::ops::{Deref, DerefMut};
use home_zircon_shared::*;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Error, Guard};


#[derive(Clone, Eq, PartialEq, Serialize, Deserialize, Responder)]
#[response(status = 500, content_type = "json")]
pub struct Token {
    token: String,
}

impl Token {
    pub fn new() -> Self {
        use base64::encode;
        use rand;
        use rand::Rng;
        let random_bytes = rand::thread_rng().gen::<[u8; 32]>();
        let token = encode(&random_bytes).to_string();

        Token {
            token
        }
    }
}

pub struct Tokens {
    tokens: HashSet<Token>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct User {
    id: u64,
    username: String,
    password: String,
}


#[derive(Serialize, Deserialize)]
pub(crate) struct Users {
    users: HashMap<u64, User>,
    tokens: HashMap<String, (Token, u64)>,
}

impl Users {

}

pub struct UsersState {
    i: RwLock<Users>,
}


impl UsersState {
    pub fn new() -> Self {
        let mut users = HashMap::with_capacity(1);

        users.insert(1, User {
            id: 1,
            username: "example@email.com".into(),
            password: "000".into()
        });

        let tokens = HashMap::with_capacity(0);

        let users = Users {
            users,
            tokens
        };

        UsersState {
            i: RwLock::new(users)
        }
    }

    pub fn get_user_by_password(&self, username: &str, password: &str) -> Option<u64> {
        let inner = self.i.read().unwrap();

        for (_, x) in inner.users.iter() {
            if &x.username == username {
                if &x.password == password {
                    return Some(x.id)
                }
            }
        }

        None
    }

    pub fn get_user_token(&self, token: &str) -> Option<u64> {
        let inner = self.i.read().unwrap();

        if let Some(token) = inner.tokens.get(token) {
            let (_, user_id) = token;

            Some(*user_id)
        } else {
            None
        }
    }

    pub fn get_user(&self, user_id: u64) -> Option<User> {
        let inner = self.i.read().unwrap();

        inner.users.get(&user_id).map(|x| x.clone())
    }

    pub fn create_user_token(&self, user_id: u64) -> Option<Token> {
        let mut inner = self.i.write().unwrap();

        let user_id = if let Some(user) = inner.users.get(&user_id) {
            Some(user.id)
        } else {
            None
        };

        if let Some(user_id) = user_id {
            let token = Token::new();

            inner.tokens.insert(token.token.clone(), (token.clone(), user_id));

            Some(token)
        } else{
            None
        }
    }
}

#[derive(Debug)]
pub enum TokenError {
    Invalid,
}

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = TokenError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        println!("asdasdada");
        println!("asdasdada");
        println!("asdasdada");
        println!("asdasdada");
        println!("asdasdada");
        println!("asdasdada");
        println!("asdasdada");
        let td = TypeId::of::<State<'_, UsersState>>();
        eprintln!("td: {:?}", td);

        let users = request.guard::<State<'_, UsersState>>().expect("must be OKAY");

        let keys: Vec<_> = request.headers().get("Authorization").collect();
        for key in keys {
            let mut splitted = key.split(" ");
            if let Some(type_) = splitted.next() {
                if type_ == "Bearer:" {
                    if let Some(payload) = splitted.next() {
                        if let Some(user_id) = users.get_user_token(payload) {
                            if let Some(user) = users.get_user(user_id) {
                                return Outcome::Success(user)
                            }
                        }
                    }
                }
            }
        }

        return Outcome::Failure((Status::Unauthorized, TokenError::Invalid))
    }
}

use rocket::response::status::NotFound;
use std::any::TypeId;


#[post("/authorize", data = "<login>")]
pub fn authorize(users: State<'_, UsersState>, login: Json<LoginForm>) -> Result<Json<Token>, NotFound<&'static str>> {
    if let Some(user_id) = users.get_user_by_password(&login.username, &login.password) {
        if let Some(token) = users.create_user_token(user_id) {
            Ok(Json(token))
        } else {
            Err(NotFound("invalid password"))
        }

    } else {
        Err(NotFound("user not found"))
    }
}

#[post("/check")]
pub fn check(users: User) -> Json<bool> {
    Json(true)
}
