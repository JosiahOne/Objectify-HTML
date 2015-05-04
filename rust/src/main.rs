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

///////////////////////////////////////
/////////////// TESTS ////////////////
//////////////////////////////////////

#[cfg(test)]
mod test {
    use objectify_html::*;

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
        get_file_contents("a-non-existing-file.txt");
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
}