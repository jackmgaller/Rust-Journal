use std::fs;
use std::collections::HashMap;

pub fn print_file_metadata(path: &String) {
  let metadata = fs::metadata(path).unwrap();

  println!("{} bytes", metadata.len());
  println!("===========");
}

pub fn print_file_data(path: &String) {
  let data = fs::read_to_string(path).unwrap();
  println!("{}", data);  
}

pub fn print_file_tags(file_name: &String, tags: &HashMap<String, Vec<String>>) {
  if tags.contains_key(file_name) {
    print!("Tags: ");
    let file_tags: Vec<String> = tags.get(file_name).unwrap().to_vec();

    for (i, file_tag) in file_tags.iter().enumerate() {
      if i != file_tags.len() - 1 {
        print!("{}, ", file_tag);
      } else {
        println!("{}", file_tag);
      }
    }
    println!("");
  } else {
    println!("No tags for {}", file_name);
  }
}