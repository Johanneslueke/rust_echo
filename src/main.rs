extern crate clap;
extern crate echo;
use clap::{Arg, App};

use echo::echo;




fn main() {
    let matches = App::new(   "Echo"  ) 
            .arg(
                Arg::with_name("INPUT")
                    .multiple(true)
                    .help("Input stream used for echoing back to the shell")
            ).get_matches();



   

   echo(matches);
}


/*
 let mut cmd : CommandOptions = CommandOptions::new( env::args().collect());
   let args = &cmd.args[1..];

   for arg in args{
       if CommandOptions::is_parameter(arg){
           if arg == "-E"
            {
                cmd.disable  = true;
                cmd.enable = false;
            }
            else if arg == "-e" {
                cmd.disable  = false;
                cmd.enable = true;
            }
            else if arg == "-n"{
                cmd.ignore_trailing_newline = true;
            }
            else
            {
                panic!("Unkown argument {}",arg);
            }
       }
       else{

       }
   }
*/
