use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        return;
    }

    let cmd = args[1].as_str();

    match cmd {
        "list" => {
            // will add loading and printing todos
            println!("Listing todos...");
        }
        "add" => {
            // will add and save todos
            println!("Adding todo...");
        }
        _ => {
            // this is the default case
            println!("Unknown command: {cmd}");
            print_help();
        }
    }
}

fn print_help() {
    // keeping the help separate so the main logic stay clean
    println!("Commands:");
    println!("  add \"todo text\"");
    println!("  list");
}
