#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::{Route, State};
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};


#[derive(
    Serialize,
    Deserialize,
)]
pub struct Post {
    pub title: String,
    pub body: String
}

impl Post {
    pub fn new(title: String, body: String) -> Post{
        Post {
            title,
            body,
        }
    }
}

pub struct Cache {
    pub posts: Vec<Post>
}

impl Cache {
    pub fn new() -> Cache{
        let mut posts = vec![
            Post::new(String::from("Article1"), String::from("Blah")),
            Post::new(String::from("Article2"), String::from("Blah")),
            Post::new(String::from("Article3"), String::from("Blah")),
            Post::new(String::from("Article4"), String::from("Blah")),
            Post::new(String::from("Article5"), String::from("Blah")),
        ];

        Cache {
            posts
        }
    }
}

#[derive(Responder)]
pub enum IndexResponder {
    #[response(status = 200)]
    Found(Json<Vec<Post>>),
    #[response(status = 404)]
    NotFound(String)
}

#[get("/")]
pub fn index(cache: State<Cache>) -> IndexResponder {
    return IndexResponder::Found(Json(cache.posts));
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .manage(Cache::new())
        .launch();
}
