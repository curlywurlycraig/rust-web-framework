use bincode;
use serde::{Serialize, Deserialize};
use warp::Filter;
use remote_attr::remote;

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    pub id: u8,
    pub content: String,
    pub completed: bool
}

fn get_todo() -> Todo {
    Todo { id: 10, content: String::from("Hello craig!"), completed: false }
}

#[remote]
fn add_todo(mut new_todo: Todo) {
    println!("Got a new todo! {:?}", new_todo);
}

fn encode_todo(todo: &Todo) -> &str {
    // encode as utf8 str
    let encoded: Vec<u8> = bincode::serialize(todo).unwrap();
    let result = String::from_utf8(encoded).unwrap();
    return format!("{}", result);
}

fn decode_todo(bytes: &[u8]) -> Todo {
    bincode::deserialize(bytes).unwrap()
}

#[tokio::main]
async fn main() {
    let get_todo = warp::path!("get_todo")
        .map(|| {
            get_todo()
        }).with(warp::cors().allow_any_origin());

    let add_todo = warp::path!("add_todo")
        .and(warp::body::bytes())
        .map(|body: warp::hyper::body::Bytes| {
            add_todo(&body)
        }).with(warp::cors().allow_any_origin());

    let routes = get_todo
        .or(add_todo);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}