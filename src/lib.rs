use mongodb::bson::oid::ObjectId;

struct Task {
    _id: ObjectId,
    task: String,
}

pub fn listgenerator<T>(vec: Vec<T>)
where
T: std::fmt::Debug,
{ 
  // println!("{:?}",vec.get(1))
}

// println!("{:?}", vec.len());
// match vec.get(1).unwrap() {
//       Task { _id, .. } => None,
//       _ => None,
//     }