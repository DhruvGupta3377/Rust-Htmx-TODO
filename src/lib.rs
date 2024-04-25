pub mod datastruct;

use datastruct::Task;

pub async fn listgenerator(vec: Vec<Task>) -> String {
    // println!("{:?}", vec.get(1).unwrap()._id.to_string());
    let mut htmlcontent = String::new();
    for item in vec.iter() {
        let temp = format!("<li>{}<t/><button hx-post='http://127.0.0.1:8000/delete/{}' hx-target='#result' hx-swap='innerHTML' >Done</button></li>", item.task,item._id);

        htmlcontent.push_str(temp.as_str());
    }
    println!("{:?}", htmlcontent);
    return htmlcontent;
}
