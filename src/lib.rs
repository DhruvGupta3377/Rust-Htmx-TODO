pub mod datastruct;

use datastruct::Task;

pub fn listgenerator(vec: Vec<Task>)->String 
{
    // println!("{:?}", vec.get(1).unwrap()._id.to_string());
    let mut htmlcontent = String::new();
    for item in vec.iter(){
        let temp = format!("<li>{}</li>", item.task);
        htmlcontent.push_str(temp.as_str());
    }
    println!("{:?}", htmlcontent);
    return htmlcontent;
}
