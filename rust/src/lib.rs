// lib.rs
// This crate is a library
#![crate_type = "lib"]
// The library is named "rary"
#![crate_name = "objectify_html"]

use std::path::Path;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct ValidData {
    pub exists: bool,
    pub data: String,
}

pub struct ParamContainer {
    pub children: Vec<ParamChild>,
}

pub struct ParamChild {
    pub param_name: String,
    pub param_content: String,
}

pub fn inline_replace_html_file(main_data: String, build_loc: String) -> String {
    let mut index: i32 = 0;
    let mut tag_name;
    let mut new_data;
    let mut mut_main = String::new();
    let mut alt_mut_main = main_data.clone();
    let mut while_flag = true;
    
    while while_flag {
        if alt_mut_main == mut_main {
            while_flag = false;
        } else {
            mut_main = alt_mut_main.clone();
            for car in mut_main.chars() {
                if car == '<' {
                    // Tag. Find tag name.
                    tag_name = get_tag_name(mut_main.clone(), index);
                    if tag_name == "include" {
                        // Get the replacement name.
                        let replacement_name = get_replacement_id(mut_main.clone(), index + tag_name.len() as i32 + 1);
                        let tag_length = get_total_tag_length(mut_main.clone(), index);
                        let parameters = get_params(mut_main.clone(), index);
                        new_data = get_new_data(replacement_name, build_loc.clone());
                        new_data = insert_parameters(new_data, parameters);
                        if new_data != "ERROR" {
                            alt_mut_main = remove_substring_at_pos(mut_main.clone(), index, index + tag_length);
                            alt_mut_main = insert_substring_at_pos(alt_mut_main.clone(), new_data, index);
                            index = 0;
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

pub fn insert_parameters(some_string: String, params: ParamContainer) -> String {
    // Substitute in parameters and return the new string.
    let mut return_data = String::new();
    
    for param in params.children {
        let find_index = get_first_location_of_string(some_string.clone(), param.param_name.clone());
        if find_index >= 0 {
            return_data = remove_substring_at_pos(some_string.clone(), find_index, find_index + param.param_name.clone().len() as i32 - 1);
            return_data = insert_substring_at_pos(return_data.clone(), param.param_content.clone(), find_index);
        }
    }
    
    return return_data;
}

pub fn get_first_location_of_string(main_data: String, substring: String) -> i32 {
    // We're looking for attribute_name="foo". Specifically:
    // Check that the next n chars == attribute_name=" and then, 
    // Capture chars in a string until a " appears.
    
    if substring == "" {
        return -1;
    }
    
    let mut indexer = 0;
    let string_to_match = substring;
    let mut chars_matched = 0;
    let mut found = false;
    
    for car in main_data.chars() {
        if chars_matched == string_to_match.len() {
            indexer -= string_to_match.len() as i32;
            found = true;
            break;
        } else if car == string_to_match.chars().nth(chars_matched).unwrap() {
            chars_matched += 1;
        } else {
            chars_matched = 0;
        }
      
        indexer += 1;
    }
    
    if !found {
        indexer = -1;
    }
    
    return indexer;
}

pub fn insert_substring_at_pos(some_string: String, substring: String, start_pos: i32 ) -> String {
    if start_pos >= some_string.len() as i32 {
        return some_string;
    }
    
    let mut indexer = start_pos;
    let mut return_string = some_string.clone();
    for car in substring.chars() {
        return_string.insert(indexer as usize, car);
        indexer += 1;
    }
    
    return return_string;
}

pub fn remove_substring_at_pos(some_string: String, start_pos: i32, end_pos: i32) -> String {
    if start_pos >= some_string.len() as i32 {
        return some_string;
    }
    
    let mut new_string = some_string;
    
    for _ in start_pos..end_pos + 1 {
        if new_string.len() > 0 {
            new_string.remove(start_pos as usize);
        }
    }
    
    return new_string;
}

pub fn get_total_tag_length(main_data: String, start_pos: i32) -> i32 {
    let value = get_entire_tag(main_data.clone(), start_pos).len() as i32;
    return value;
}

pub fn get_entire_tag(main_data: String, start_pos: i32) -> String {
    let mut indexer = 0;
    let mut return_data = String::new();
    let mut flag = true;
    for car in main_data.chars() {
      
        if indexer >= start_pos && flag {
            if car == '>' {
                return_data.push_str(&*car.to_string());
                flag = false;
            } else {
                return_data.push_str(&*car.to_string());
            }
        }
      
        indexer += 1;
    }
    
    return return_data;
}

pub fn get_new_data(replacement_id: String, build_loc: String) -> String {
    // build_loc is a file that we need to read so that we can get the locations of our .ohtml files.
    let build_file_contents = get_file_contents(&*build_loc);
    let files: Vec<String> = get_substrings_from_delims(build_file_contents, '[', ']');
    let mut some_valid_data;
    let mut final_return_data = "ERROR".to_string();
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

pub fn does_replacement_exist(file: String, replacement_id: String) -> ValidData {
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

pub fn get_replacement_data(file_contents: String, start_pos: i32) -> String {
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

pub fn get_substrings_from_delims(some_string: String, start_delim: char, end_delim: char) -> Vec<String> {
    let mut substrings: Vec<String> = vec![];
    
    let mut currently_matching = false;
    let mut temp_data = String::new();
    
    for car in some_string.chars() {
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

pub fn get_tag_name(main_data: String, start_index: i32) -> String {
    // Starting at the start_index, collect characters until we hit the ' '.
    
    let mut name = String::new();
    let mut counter = 0;
    
    for car in main_data.chars() {
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

pub fn get_replacement_id(main_data: String, start_index: i32) -> String {
    return get_attribute(main_data.clone(), "object".to_string(), start_index);
}

// start_index should be the position of the opening '<' character.
pub fn get_params(main_data: String, start_index: i32) -> ParamContainer {
    let mut params = ParamContainer{children: vec![ParamChild{param_name: "tag".to_string(), param_content: get_tag_name(main_data.clone(), start_index)}]};
    let param_string = get_attribute(main_data.clone(), "params".to_string(), start_index);
    let single_params = get_substrings_from_delims(param_string, '[', ']');
    let mut indexer: u32 = 1;
    
    for each in single_params {
        let name: String = "@p".to_string() + &*indexer.to_string() + "@";
        params.children.push(ParamChild{param_name: name, param_content: each });
        indexer += 1;
    }
    return params;
}

pub fn get_attribute(some_string: String, attribute_name: String, start_index: i32) -> String {
    // We're looking for attribute_name="foo". Specifically:
    // Check that the next n chars == attribute_name=" and then, 
    // Capture chars in a string until a " appears.
    let mut indexer = 0;
    let string_to_match = attribute_name.clone() + "=\"";
    let mut chars_matched = 0;
    let mut label = String::new();
    
    for car in some_string.chars() {
        if indexer > start_index {
            if chars_matched == string_to_match.len() {
                if car == '"'{
                    break;
                }
                label.push_str(&*car.to_string());
            } else if car == string_to_match.chars().nth(chars_matched).unwrap() {
                chars_matched += 1;
            } else {
                chars_matched = 0;
            }
        }
      
        indexer += 1;
    }
    
    return label;
}

pub fn get_file_contents(p: &str) -> String {
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
