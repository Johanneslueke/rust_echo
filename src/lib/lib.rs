extern crate clap;
use clap::{ArgMatches};


pub fn escape(input : &str) -> String{

    if ! input.contains("\\") {
        return String::from(input);
    }

    let mut escaped = String::from("");
    for (_index,ch)  in input.char_indices(){
        if ! (ch == '\\'){
            escaped.push(ch);
            continue;
        }
        
    }
    escaped
}

pub fn echo (matches : ArgMatches) {

 let input = matches.values_of("INPUT");
    match input {
        Some(x) => {
            let input = &x.fold(String::from(""),
                               |prev,cur| {
                                   String::from(prev) + &String::from(cur)
                                });
            match matches {
                _ if matches.is_present("E") => println!("{}",input),
                _ if matches.is_present("E")
                  && matches.is_present("n") => print!("{}",input),
                _ if matches.is_present("n") => print!("{}",escape(input)),
                _ => println!("{}",escape(input))
            };
        }
        ,
        None => println!(""),
    };

  
}