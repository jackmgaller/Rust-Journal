use std::fs;
use std::collections::HashMap;
use std::env;
use std::path::Path;
use chrono::{Datelike, Utc};

mod print_journal;

pub use crate::print_journal::*;

fn main() {

  let args: Vec<String> = env::args().collect();


  let mut tags: HashMap<String, Vec<String>> = HashMap::new();
  import_tags(&mut tags);

  let file_names = get_file_names();

  for (i, arg) in args.iter().enumerate() {
    if arg == "--print-all" {
      print_all(&file_names, &tags);
    } else if arg == "--print" {
      let file_search = args[i + 1].as_str();
      print_one(&file_search, &file_names, &tags);
    } else if arg == "--store" {
      let file_path = args[i + 1].as_str();
      store_entry(Path::new(file_path));
    } else if arg == "--add-tags" {
      let file_path = args[i + 1].as_str();
      let tags: Vec<String> = args[i + 2]
                                .as_str()
                                .split(",")
                                .map(|x| String::from(x))
                                .collect::<Vec<String>>();

      add_tags(Path::new(file_path), &tags)
    } else if arg == "--template" {
      template()
    }
  }

}

fn print_one(file_search: &str, file_names: &Vec<String>, tags: &HashMap<String, Vec<String>>) {
  for file_name in file_names {
    if file_name.as_str().contains(file_search) {
      println!("{}", file_name);
      print_file_metadata(&file_name);
      print_file_data(&file_name);
      print_file_tags(&file_name, &tags);
      break;
    }
  }
}

fn print_all(file_names: &Vec<String>, tags: &HashMap<String, Vec<String>>) {
  for file_name in file_names {
    println!("{}", file_name);
    print_file_metadata(&file_name);
    print_file_data(&file_name);
    print_file_tags(&file_name, &tags);
  }
}

fn get_file_names() -> Vec<String> {
  let files = fs::read_dir("data")
                .unwrap()
                .map(|file| {
                  let file = file.unwrap();
                  return String::from(file.path().to_str().unwrap());
                })
                .collect();
  return files;
}

fn import_tags(tags: &mut HashMap<String, Vec<String>>) {
  let tags_data = fs::read_to_string("meta/tags.txt").unwrap();
  let tags_data = tags_data.as_str().split("\n").collect::<Vec<&str>>();

  for tag_line in tags_data {

    let (file_name, file_tags) = tag_line.split_at(tag_line.find(",").unwrap());
    let file_name = String::from(file_name);
    
    let file_tags = file_tags.split("-").map(|tag| tag.replace(",", "")).collect::<Vec<String>>();

    &tags.insert(file_name, file_tags);
  }
}

fn store_entry(path: &Path) {
  let file_contents = fs::read_to_string(path).unwrap();
  
  let now = Utc::now();
  let file_name = format!("data/{}-{}-{}.md", now.year(), now.month(), now.day());
  let file_path = Path::new(&file_name);

  if !file_path.exists() {
    fs::write(&file_path, &file_contents).unwrap();
    println!("Successfully wrote entry to {}", &file_path.to_str().unwrap());
  } else {
    println!("A file already exists at {}", &file_path.to_str().unwrap());
  }
}

fn add_tags(path: &Path, tags: &Vec<String>) {
  let existing_tags = fs::read_to_string("meta/tags.txt").unwrap();
  let new_tags = String::from(path.to_str().unwrap()) + "," + &tags.join("-");
  let file_contents = existing_tags + "\n" + &new_tags;

  fs::write("meta/tags.txt", &file_contents).unwrap();
  print!("Added tags: ");

  for (i, tag) in tags.iter().enumerate() {
    if i != tags.len() - 1 {
      print!("{}, ", tag);
    } else {
      println!("{}", tag);
    }
  }
}

fn template() {
  fs::copy("res/template.md", "template.txt").unwrap();
}












