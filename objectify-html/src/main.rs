use std::env;
use std::path::Path;
use std::error::Error;
use std::fs::File;
use std::io::Read;

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

    inline_replace_html_file(main_file);
}

fn inline_replace_html_file(main_file: String) {
    let mut index = 0;

    for car in main_file.chars() {
        if car == '<' {
            // Tag. Find tag name.
            if get_tag_name(main_file.clone(), index) == "include" {
                // Get the replacement name.
                
            }
        }
        
        index += 1;
    }
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
