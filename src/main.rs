use rocket::form::Form;
use rocket::{futures::TryStreamExt , http::ContentType};
use rust_htmx_todo::datastruct::Task;
use serde::{Deserialize, Serialize};
use rocket::serde::json::Json;

use rust_htmx_todo::listgenerator;
use std::{fs::File, vec};
use std::io::Read;


use mongodb::{bson::doc, Client, Collection};
// use mongodb::{ bson::doc, sync::{ Client, Collection } };

#[derive(Serialize, Deserialize, Debug, FromForm)]
struct TaskDoc{
    task:String
}



static URL: &str = "mongodb+srv://dhruvgupta3377:fD4Sn5RSdFGRvzE4@todos.dviwdft.mongodb.net/?retryWrites=true&w=majority&appName=todos";

async fn find_all() -> String{
    let client = Client::with_uri_str(URL).await.unwrap();
    println!("doing some thing");
    let my_coll: Collection<Task> = client
        .database("todo-db")
        .collection("todo-collection");
    let mut cursor = my_coll.find(None, None).await.expect("fucked up here");
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

#[post("/", data = "<task>")]
async fn create(task:Form<TaskDoc>) -> (ContentType, String) {
    let client = Client::with_uri_str(URL).await.unwrap();
    println!("doing some thing");
    let my_coll: Collection<TaskDoc> = client
        .database("todo-db")
        .collection("todo-collection");
    if task.task.clone() != "".to_string(){
        let doc = TaskDoc {
            task: task.task.clone()
        };
        my_coll.insert_one(doc, None).await.expect("Error in creating Task");
    }
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

