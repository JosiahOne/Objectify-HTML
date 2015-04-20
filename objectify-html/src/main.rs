use std::env;
use std::path::Path;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

struct ValidData {
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

    let data_to_write = inline_replace_html_file(main_file, build_option);
    
    println!("{}", data_to_write);
}

fn inline_replace_html_file(main_file: String, build_loc: String) -> String {
    let mut index: i32 = 0;
    let mut tag_name;
    let mut new_data;
    let mut mut_main = String::new();
    let mut alt_mut_main = main_file.clone();
    let mut while_flag = true;
    while while_flag {
        if alt_mut_main == mut_main {
            while_flag = false;
        } else {
            mut_main = alt_mut_main.clone();
            for car in mut_main.chars() {
                if car == '<'{
                    // Tag. Find tag name.
                    tag_name = get_tag_name(mut_main.clone(), index);
                    if tag_name == "include" {
                        // Get the replacement name.
                        let replacement_name = get_replacement_id(mut_main.clone(), index + tag_name.len() as i32 + 1);
                        new_data = get_new_data(replacement_name, build_loc.clone());
                        if new_data != "ERROR" {
                            alt_mut_main = remove_substring_at_pos(mut_main.clone(), index, index + tag_name.len() as i32 + 17);        
                            alt_mut_main = insert_substring_at_pos(alt_mut_main.clone(), new_data, index);
                            break;
                        }
                    }
                }
                
                index += 1;
            }
        }
    }
    
    return mut_main;
}

fn insert_substring_at_pos(main_string: String, substring: String, start_pos: i32 ) -> String {
    let mut indexer = start_pos;
    let mut return_string = main_string.clone();
    for car in substring.chars() {
        return_string.insert(indexer as usize, car);
        indexer += 1;
    }
    
    return return_string;
}

fn remove_substring_at_pos(main_string: String, start_pos: i32, end_pos: i32) -> String {
    let mut new_string = main_string;
    
    for i in start_pos..end_pos {
        new_string.remove(start_pos as usize);
    }
    
    return new_string;
}

fn get_new_data(replacement_id: String, build_loc: String) -> String {
    // build_loc is a file that we need to read so that we can get the locations of our .ohtml files.
    let build_file_contents = get_file_contents(&*build_loc);
    let files: Vec<String> = get_substrings_from_delims(build_file_contents, '[', ']');
    let mut some_valid_data;
    let mut final_return_data = String::new();
    let mut flag = true;
    for each in files {
        if flag {
            // We're searching each file for the replacement_id now.
            let file_contents = get_file_contents(&*each);
            some_valid_data = does_replacement_exist(file_contents.clone(), replacement_id.clone());
            if some_valid_data.exists {
                final_return_data = some_valid_data.data;
                flag = false;
            }
        }
    }
    
    return final_return_data;
}

fn does_replacement_exist(file: String, replacement_id: String) -> ValidData {
    // Return the valid replacement string if it exists, otherwise, ValidData.exists should be false and
    // the developer should verify that before reading the ValidData.data.
    
    let mut indexer = 0;
    let mut tag_name;
    let mut return_data = ValidData{exists: false, data: "".to_string()};
    let mut flag = true;
    for car in file.chars() {
        if car == '<' && flag {
            tag_name = get_tag_name(file.clone(), indexer);
            if tag_name == "begin" {
                let replacement_name = get_replacement_id(file.clone(), indexer + tag_name.len() as i32 + 1);
                if replacement_name == replacement_id {
                    // Found the replacement. Get its content and return.
                    return_data.exists = true;
                    return_data.data = get_replacement_data(file.clone(), indexer + tag_name.len() as i32 + replacement_id.len() as i32 + 11);
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
    let mut tag_name;
    let mut flag = true;
    for car in file_contents.chars() {
        if indexer > start_pos && flag == true {
            if car == '<' {
                tag_name = get_tag_name(file_contents.clone(), indexer);
                if tag_name == "/begin".to_string() {
                    // STOP READING
                    flag = false;
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
        Ok(_) => print!(""),
    };
    
    // `file` goes out of scope, and the "hello.txt" file gets closed
    return s;
}
