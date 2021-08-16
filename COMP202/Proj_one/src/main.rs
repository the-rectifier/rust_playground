use std::io;
use std::env;
use std::process;
use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut list: Vec<String> = Vec::new();
    let mut filename = String::new();

    if args.len() == 1 { 
        print!("Enter File to Process: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut filename)
            .expect("Failed to Read Line");
    } else if args.len() == 2 {
        filename = args[1].clone();
    } else {
        help();
        process::exit(1);
    }
    println!("Proccessing File: {}", filename);

    read_file(&mut list, &filename);

    handle_cli(&mut list, &filename);
}

fn handle_cli(vec: &mut Vec<String>, filename: &String) {
    let mut index = 0;
    let mut newline_toggle = true;
    loop {
        let mut choice = String::new();

        print!("\nCMD> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut choice).expect("Couln't Read User Choice");
        choice = String::from(choice.trim());
        if choice.len() > 1 {
            continue;
        } else {
            choice = String::from(&choice[..1]);
        }
        
        match choice.as_str() {
            "^" => index = 0,
            "$" => index = vec.len() - 1,
            "-" => if index > 0 { index -= 1 },
            "+" => if index <= vec.len()-2 { index += 1},
            "a" => add_after(vec, index),
            "t" => add_before(vec, index),
            "d" => delete_current(vec, index),
            "l" => print_all(vec.to_vec(), newline_toggle),
            "n" => newline_toggle = !newline_toggle,
            "p" => print_current(vec.to_vec(), newline_toggle, index),
            "q" => quit(),
            "w" => write(vec.to_vec(), &filename),
            "x" => {write(vec.to_vec(), &filename);quit()},
            "=" => println!("{}", index),
            // "#" => stats(),
            // "c" => create_index_file(),
            // "v" => print_index(),
            _ => continue,
        }
    }
}

fn delete_current(vec: &mut Vec<String>, index: usize) {
    vec.remove(index);
}

fn add_before(vec: &mut Vec<String>, index: usize){
    let mut user_in = String::new();
    print!("Enter Text to be inserted: ");
    io::stdout().flush().unwrap();
    
    io::stdin().read_line(&mut user_in).expect("Couldn't Read Line");
    user_in = user_in.replace("\n", "");
    vec.insert(index, user_in);
}

fn add_after(vec: &mut Vec<String>, index: usize){
    let mut user_in = String::new();
    print!("Enter Text to be inserted: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut user_in).expect("Couldn't Read Line");
    user_in = user_in.replace("\n", "");
    if index == vec.len() - 1 {
        vec.push(user_in)
    } else {
        vec.insert(index + 1, user_in);
    }
}

fn quit() {
    process::exit(0);
}

fn write(vec: Vec<String>, filename: &String) {
        let mut file = File::create(filename).unwrap();
    for (_, line) in vec.iter().enumerate() {
        file.write_all(line.as_bytes()).unwrap();
        file.write_all(b"\n").unwrap();
    }
}

fn print_current(vec: Vec<String>, nl: bool, index: usize) {
    if nl {
        println!("{}. {}", index+1, &vec[index]);
    } else {
        println!("{}", &vec[index]);
    }
}

fn print_all(vec: Vec<String>, nl: bool) {
    for (index, line) in vec.iter().enumerate() {
        if nl {
            println!("{}. {}", index+1, line);
        } else {
            println!("{}",line);
        }
    }
}

fn read_file(vec: &mut Vec<String>, filename: &String) {
    let fd = File::open(filename).expect("Couldn't Open File");
    let reader = BufReader::new(fd);

    for (_, line) in reader.lines().enumerate() {
        let line: String = String::from(line.unwrap()).replace("\n", "");
        if line.len() > 80 {
            let mut temp = line.clone();
            while temp.len() > 80 {
                vec.push(String::from(&temp[..80]));
                temp = String::from(&temp[80..]);
            }
            if temp.len() == 0{
                continue;
            }
            vec.push(temp);
        } else {
            if line.len() == 0 {
                continue;
            }
            vec.push(line);
        }
    }
   
}

fn help() {
    println!("Usage: ./proj_one [filename]");
}
