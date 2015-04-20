use std::env;
use std::path::Path;
use std::error::Error;
use std::fs::File;
use std::io::Read;

struct Valid_Data {
    exists: bool,
    data: String,
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let mut compile_option = String::new();
    let mut build_option = String::new();
    let mut add_c_option = false;
    let mut add_b_option = false;
    
    for i in args {
        if add_c_option {
            compile_option = i.clone();
            add_c_option = false;
        } else if add_b_option {
            build_option = i.clone();
            add_b_option = false;
        }
        
        if i == "-c" {
            add_c_option = true;
        } else if i == "-b" {
            add_b_option = true;
        }
    }
    // !!! EARLY RETURNS !!!
    // If a compile_option was not supplied, we can't continue.
    if compile_option == "" {
        println!("No compile (-c) option was supplied! Aborting.");
        return;
    }
    // !!! END EARLY RETURNS !!!
    
    if build_option == "" {
        build_option = ".build".to_string();
    }
    
    
    let main_file = get_file_contents(&*compile_option); // :String

    inline_replace_html_file(main_file, build_option);
}

fn inline_replace_html_file(main_file: String, build_loc: String) {
    let mut index: i32 = 0;
    let mut tag_name = String::new();
    let mut new_data = String::new();
    
    for car in main_file.chars() {
        if car == '<' {
            // Tag. Find tag name.
            tag_name = get_tag_name(main_file.clone(), index);
            if tag_name == "include" {
                // Get the replacement name.
                let replacement_name = get_replacement_id(main_file.clone(), index + tag_name.len() as i32 + 1);
                new_data = get_new_data(replacement_name, build_loc.clone());
            }
        }
        
        index += 1;
    }
}

fn get_new_data(replacement_id: String, build_loc: String) -> String {
    // build_loc is a file that we need to read so that we can get the locations of our .ohtml files.
    let build_file_contents = get_file_contents(&*build_loc);
    let files: Vec<String> = get_substrings_from_delims(build_file_contents, '[', ']');
    
    for each in files {
        // We're searching each file for the replacement_id now.
        let file_contents = get_file_contents(&*each);
        
    }
    
    return "".to_string();
}

fn does_replacement_exist(file: String, replacement_id: String) -> Valid_Data {
    // Return the valid replacement string if it exists, otherwise, Valid_Data.exists should be false and
    // the developer should verify that before reading the Valid_Data.data.
    
    let mut indexer = 0;
    let mut tag_name = String::new();
    let mut return_data = Valid_Data{exists: false, data: "".to_string()};
    let mut flag = true;
    for car in file.chars() {
        if car == '<' && flag {
            tag_name = get_tag_name(file.clone(), indexer);
            if tag_name == "begin" {
                let replacement_name = get_replacement_id(file.clone(), indexer + tag_name.len() as i32 + 1);
                if replacement_name == replacement_id {
                    // Found the replacement. Get its content and return.
                    return_data.exists = true;
                    return_data.data = get_replacement_data(file.clone(), indexer + tag_name.len() as i32 + replacement_id.len() as i32 + 1);
                    flag = false;
                }
            }
        }
        indexer += 1;
    }
    
    return return_data;
}

fn get_replacement_data(file_contents: String, start_pos: i32) -> String {
    let mut read_data = String::new();
    let mut indexer = 0;
    let mut tag_name = String::new();
    let mut flag = true;
    for car in file_contents.chars() {
        if indexer > start_pos && flag == true {
            if car == '<' {
                tag_name = get_tag_name(file_contents.clone(), indexer);
                if tag_name == "/begin" {
                    // STOP READING
                    flag == false;
                }
            }
            
            if flag {
                read_data.push_str(&*car.to_string());
            }
        }
        indexer += 1;
    }
    
    return read_data;
}

fn get_substrings_from_delims(main_string: String, start_delim: char, end_delim: char) -> Vec<String> {
    let mut substrings: Vec<String> = Vec::<String>::new();
    
    let mut currently_matching = false;
    let mut temp_data = String::new();
    
    for car in main_string.chars() {
        if currently_matching && car != end_delim {
            temp_data.push_str(&*car.to_string());
        } else if car == end_delim {
            currently_matching = false;
            substrings.push(temp_data);
            temp_data = "".to_string();
        }
        
        if car == start_delim && !currently_matching {
            currently_matching = true;
        }
    }
    
    return substrings;
}

fn get_tag_name(input_data: String, start_index: i32) -> String {
    // Starting at the start_index, collect characters until we hit the ' '.
    
    let mut name = String::new();
    let mut counter = 0;
    
    for car in input_data.chars() {
        if counter > start_index {
            if car != ' ' {
                name.push_str(&*car.to_string());
            } else {
                break;
            }
        }
        counter += 1;
    }
    
    return name;
}

fn get_replacement_id(input_data: String, start_index: i32) -> String {
    // We're looking for object="foo". Specifically:
    // Check that the next 8 chars == object=" and then, 
    // Capture chars in a string until a " appears.
    
    let mut label = String::new();
    let mut counter = 0;
    let mut alt_counter = 0;
    let to_match = vec!['o', 'b', 'j', 'e', 'c', 't', '=', '"'];
    let mut should_match = true;
    let mut should_continue = true;
    
    for car in input_data.chars() {
        if counter > start_index && should_continue {

            if alt_counter + 1 > to_match.len() {
                // Start
                should_match = false;
                
                if car == '"' {
                    break;
                }
                
                label.push_str(&*car.to_string());
            }
        
            if should_match && car != to_match[alt_counter] {
                should_continue = false;
            }
            alt_counter += 1;
        }
        counter += 1;
    }
    
    return label;
}

fn get_file_contents(p: &str) -> String {
    // Create a path to the desired file
    let path = Path::new(p);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   Error::description(&why)),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   Error::description(&why)),
        Ok(_) => print!("{} contains:\n{}", display, s),
    };
    
    // `file` goes out of scope, and the "hello.txt" file gets closed
    return s;
}
