use rocket::http::ContentType;
use std::fs::File;
use std::io::Read;

#[macro_use]
extern crate rocket;

#[get("/")]
fn home() -> (ContentType, String) {
    let file_path = "index.html";
    let mut file = File::open(file_path).expect("Can't open the file");
    let mut html_content = String::new();
    file.read_to_string(&mut html_content).expect("Can't convert to string");
    (ContentType::HTML, html_content)
}

#[post("/")]
fn create() -> (ContentType, &'static str) {
    println!("created");
    // "created"
    (ContentType::HTML, "<p>gg</p>")
}



#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![home]).mount("/create", routes![create])
}
