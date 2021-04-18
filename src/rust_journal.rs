use std::fs;
use std::collections::HashMap;
use std::env;

fn main() {

  let args: Vec<String> = env::args().collect();


  let mut tags: HashMap<String, Vec<String>> = HashMap::new();
  import_tags(&mut tags);

  let file_names = get_file_names();

  for (i, arg) in args.iter().enumerate() {
    if arg == "--print-all" {
      print_all(&file_names, &tags);
    } else if arg == "--print" {
      print_one(&args[i + 1].as_str(), &file_names, &tags);
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

fn print_file_metadata(path: &String) {
  let metadata = fs::metadata(path).unwrap();

  println!("{} bytes", metadata.len());
  println!("===========");
}

fn print_file_data(path: &String) {
  let data = fs::read_to_string(path).unwrap();
  println!("{}", data);  
}

fn print_file_tags(file_name: &String, tags: &HashMap<String, Vec<String>>) {
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