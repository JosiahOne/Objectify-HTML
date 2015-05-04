use std::env;
use std::path::Path;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

struct ValidData {
    exists: bool,
    data: String,
}

struct ParamContainer {
    children: Vec<ParamChild>,
}

struct ParamChild {
    param_name: String,
    param_content: String,
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

    // If a compile_option was not supplied, we can't continue.
    if compile_option == "" {
        println!("No compile (-c) option was supplied! Aborting.");
    } else {        
        if build_option == "" {
            build_option = ".build".to_string();
        }

        let main_data = get_file_contents(&*compile_option); // :String

        let data_to_write = inline_replace_html_file(main_data, build_option);

        println!("{}", data_to_write);
    }
}

fn inline_replace_html_file(main_data: String, build_loc: String) -> String {
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

fn insert_parameters(some_string: String, params: ParamContainer) -> String {
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

fn get_first_location_of_string(main_data: String, substring: String) -> i32 {
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

fn insert_substring_at_pos(some_string: String, substring: String, start_pos: i32 ) -> String {
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

fn remove_substring_at_pos(some_string: String, start_pos: i32, end_pos: i32) -> String {
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

fn get_total_tag_length(main_data: String, start_pos: i32) -> i32 {
    let value = get_entire_tag(main_data.clone(), start_pos).len() as i32;
    return value;
}

fn get_entire_tag(main_data: String, start_pos: i32) -> String {
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

fn get_new_data(replacement_id: String, build_loc: String) -> String {
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

fn get_substrings_from_delims(some_string: String, start_delim: char, end_delim: char) -> Vec<String> {
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

fn get_tag_name(main_data: String, start_index: i32) -> String {
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

fn get_replacement_id(main_data: String, start_index: i32) -> String {
    return get_attribute(main_data.clone(), "object".to_string(), start_index);
}

// start_index should be the position of the opening '<' character.
fn get_params(main_data: String, start_index: i32) -> ParamContainer {
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

fn get_attribute(some_string: String, attribute_name: String, start_index: i32) -> String {
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

///////////////////////////////////////
/////////////// TESTS ////////////////
//////////////////////////////////////

#[test]
fn it_works() {
}

#[test]
fn test_get_attribute() {
    let attribute = get_attribute("<div id=\"Test\">".to_string(), "id".to_string(), 0);
    
    if attribute != "Test" {
        assert!(false);
    }
}

#[test]
fn test_get_tag_name() {
    let tag_name = get_tag_name("<div id=\"Test\">".to_string(), 0);
    
    if tag_name != "div" {
        assert!(false);
    }
}

#[test]
fn test_get_substrings_from_delims() {
    let substrings = get_substrings_from_delims("[foo][bar]".to_string(), '[', ']');
    
    if substrings[0] != "foo".to_string() || substrings[1] != "bar".to_string() {
        assert!(false);
    }
    
    if get_substrings_from_delims("[foo][bar]".to_string(), 'X', 'Z').len() != 0 {
        assert!(false);
    }
}

#[test]
fn test_get_params() {
    let params = get_params("<div id=\"thing\" params=\"[foo][bar]\">".to_string(), 0);
    
    if params.children[1].param_name != "@p1@".to_string() || params.children[1].param_content != "foo".to_string() {
        assert!(false);
    }
    
    if params.children[2].param_name != "@p2@".to_string() || params.children[2].param_content != "bar".to_string() {
        assert!(false);
    }
}

#[test]
fn test_get_first_location_of_string() {
    let loc = get_first_location_of_string("The quick brown fox jumped.".to_string(), "brown".to_string());
    
    if loc != 10 {
        assert!(false);
    }
    
    let loc2 = get_first_location_of_string("The quick brown fox jumped.".to_string(), "Hello".to_string());
    let loc3 = get_first_location_of_string("".to_string(), "Thing".to_string());
    
    if loc2 != -1 && loc3 != -1 {
        assert!(false);
    }
}

#[test]
fn test_insert_parameters() {
    let string_to_test = "<div class=\"@p1@\"/>".to_string();
    let params = ParamContainer{children: vec![ParamChild{param_name: "@p1@".to_string(), param_content: "foo".to_string()}]};
    
    let return_stuff = insert_parameters(string_to_test, params);
    println!("Return value = {}", return_stuff);
    
    if return_stuff != "<div class=\"foo\"/>" {
        assert!(false);
    }
}

#[test]
fn test_get_total_tag_length() {
    let string_to_test = "<div foo=\"bar\">".to_string();
    let return_number = get_total_tag_length(string_to_test, 0);
    
    if return_number != 15 {
        assert!(false);
    }
}

#[test]
fn test_get_replacement_id() {
    let string_to_test = "<div object=\"foo\"/>".to_string();
    let return_data = get_replacement_id(string_to_test.clone(), 0);
    let return_data_2 = get_replacement_id(string_to_test.clone(), 1);
    
    if return_data != "foo".to_string() && return_data_2 != "foo".to_string() {
        assert!(false);
    }
}

#[test]
fn test_get_file_contents() {
    let file = "test.html".to_string();
    let proper_response = "<html>\n  <body>\n    <include object=\"test\"/>\n  </body>\n</html>\n".to_string();
    let return_data = get_file_contents(&*file);
    
    if return_data != proper_response {
        assert!(false);
    }
}

#[test]
#[should_panic]
fn test_get_non_existing_file_contents() {
    let return_data = get_file_contents("a-non-existing-file.txt");
}

#[test]
fn test_remove_substring_at_pos() {
    let string_to_test = "Hello world. Testing 1 2 3.".to_string();
    let expected_result = "Hell. Testing 1 2 3.".to_string();
    let result = remove_substring_at_pos(string_to_test, 4, 10);
    
    if result != expected_result {
        assert!(false);
    }
}
