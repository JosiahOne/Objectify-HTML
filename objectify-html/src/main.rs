use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    let mut compile_option = "";
    let mut build_option = "";
    
    let mut index_counter = 0;
    for i in args {
        if i == "-c" && index_counter + 1 <= args.len() {
            compile_option = args[index_counter + 1];
        } else if i == "-b" && index_counter + 1 <= args.len() {
            build_option = args[index_counter + 1];
        }
        
        index_counter++;
    }
    
    let main_file = get_main_file(compile_option);
    
    
}

fn get_main_file(p: String) {
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
    }

    // `file` goes out of scope, and the "hello.txt" file gets closed

}
