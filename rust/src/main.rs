extern crate objectify_html;

use objectify_html::*;
use std::env;

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