use rocket::{futures::TryStreamExt , http::ContentType};
use rust_htmx_todo::listgenerator;
use std::{fs::File, vec};
use std::io::Read;


use mongodb::{bson::{doc, oid::ObjectId}, Client, Collection};

use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
struct Task{
    _id: ObjectId,
    task:String
}

async fn find_all() -> mongodb::error::Result<()> {
    let uri = "mongodb+srv://dhruvgupta3377:fD4Sn5RSdFGRvzE4@todos.dviwdft.mongodb.net/?retryWrites=true&w=majority&appName=todos";
    let client = Client::with_uri_str(uri).await?;
    println!("doing some thing");
    let my_coll: Collection<Task> = client
        .database("todo-db")
        .collection("todo-collection");
    let mut cursor = my_coll.find(doc! {}, None).await?;
    

    let mut vec:Vec<Task> = Vec::new();
    while let Some(doc) = cursor.try_next().await? {
        vec.push(doc);
        // println!("{:?}", doc);
    }

    listgenerator(vec);
    Ok(())  
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
async fn create() -> (ContentType, &'static str) {
    println!("created");
    let _ = find_all().await;
    (ContentType::HTML, "<p>gg</p>")

}


#[launch]
async fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![home])
        .mount("/create", routes![create])
}

// TODO: Connect to a database
// TODO: Learn Tokio
// TODO: Make an rs library
// TODO: Add new TODO functionality
// TODO: Add delete functionality

// fD4Sn5RSdFGRvzE4
// dhruvgupta3377
