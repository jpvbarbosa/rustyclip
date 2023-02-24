use std::io;
use std::fs;
use toml::Table;
use x11_clipboard::Clipboard;

struct Config{
    mem_capacity: usize
}

fn main() {
    
    let configpath = "Config.toml";
    
    let config = fs::read_to_string(configpath).expect("Failed to read config file");

    let config = config.parse::<Table>().expect("Failed to parse config file");

    let mem_capacity = match config["defaults"]["mem_capacity"].as_integer() {
        Some(v) => v,
        None => {
            println!("Failed to read mem_capacity. Using default of 10");
            10
        }
    };

    let config = Config { mem_capacity: mem_capacity as usize};

    let _clipboard = Vec::<String>::with_capacity(config.mem_capacity);

    poll()

    //main_process(&mut clipboard);
}

fn main_process(clipboard: &mut Vec::<String>)
{
    loop {
        let mut input = String::new();
        io::stdin()
        .read_line(&mut input)
        .expect("Failed to get user input");

        clipboard.push(input.trim().to_string()); // Temporary trim to remove \n from console input

        println!("Clipboard is {clipboard:?}")
    }
}

// From https://github.com/quininer/x11-clipboard/blob/master/examples/monitor_primary_selection.rs
fn poll() {
    let clipboard = Clipboard::new().unwrap();
    let mut last = String::new();

    println!("Waiting for selection...");

    loop {
        if let Ok(curr) = clipboard.load_wait(
            clipboard.getter.atoms.primary,    
            clipboard.getter.atoms.utf8_string,
            clipboard.getter.atoms.property
        ) {
            let curr = String::from_utf8_lossy(&curr);
            let curr = curr.trim_matches('\u{0}');
            if !curr.is_empty() && last != curr {
                last = curr.to_owned();
                println!("Contents of primary selection: `{last}`");
                println!("Waiting for selection...");
            }
        }
    }
}