use std::fs;
use std::collections::HashMap;

fn main() {
  let mut tags: HashMap<String, Vec<String>> = HashMap::new();

  import_tags(&mut tags);

  let file_names = get_file_names();

  for file_name in file_names {
    println!("{}", file_name);

    print_file_metadata(&file_name);
    print_file_data(&file_name);

    if tags.contains_key(&file_name) {
      print!("Tags: ");
      let file_tags: Vec<String> = tags.get(&file_name).unwrap().to_vec();

      for (i, file_tag) in file_tags.iter().enumerate() {
        if i != file_tags.len() - 1 {
          print!("{}, ", file_tag);
        } else {
          println!("{}", file_tag);
        }
      }
      println!("");
    }
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