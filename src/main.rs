use rocket::{futures::TryStreamExt , http::ContentType};
use rust_htmx_todo::datastruct::Task;


use rust_htmx_todo::listgenerator;
use std::{fs::File, vec};
use std::io::Read;


use mongodb::{bson::doc, Client, Collection};


async fn find_all() -> String{
    let uri = "mongodb+srv://dhruvgupta3377:fD4Sn5RSdFGRvzE4@todos.dviwdft.mongodb.net/?retryWrites=true&w=majority&appName=todos";
    let client = Client::with_uri_str(uri).await.unwrap();
    println!("doing some thing");
    let my_coll: Collection<Task> = client
        .database("todo-db")
        .collection("todo-collection");
    let mut cursor = my_coll.find(doc! {}, None).await.unwrap();
    

    let mut vec:Vec<Task> = Vec::new();
    while let Some(doc) = cursor.try_next().await.unwrap() {
        vec.push(doc);
    }
    return listgenerator(vec)  
}


#[macro_use]
extern crate rocket;

#[get("/")]
fn home() -> (ContentType, String) {
    let file_path = "index.html";
    let mut file = File::open(file_path).expect("Can't open the file");

    let mut html_content = String::new();
    file.read_to_string(&mut html_content)
        .expect("Can't convert to string");
    (ContentType::HTML, html_content)
}

#[post("/")]
async fn create() -> (ContentType, String) {
    let s = find_all().await;
    (ContentType::HTML, s)
}


#[post("/")]
async fn delete() -> (ContentType, String) {
let s = find_all().await;
(ContentType::HTML, s)
}


#[launch]
async fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![home])
        .mount("/create", routes![create])
        .mount("/delete",routes![delete])
}

// TODO: Connect to a database
// TODO: Learn Tokio
// TODO: Make an rs library
// TODO: Add new TODO functionality
// TODO: Add delete functionality

// fD4Sn5RSdFGRvzE4
// dhruvgupta3377
