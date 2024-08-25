use std::env;
use std::process;

use minigrep::*;

fn main() {
    //let args: Vec<String> = env::args().collect();

    /*let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {query}");
    println!("In file {file_path}");

    let content=fs::read_to_string(file_path).expect("should be able to read the file");

     println!("With text:\n{content}");*/
    let args: Vec<String> = env::args().collect();

    let config = minigrep::Config::new(&args).unwrap_or_else(|err| {
        println!("{}",err);
        process::exit(1);
    });

    println!("{}",config.query);
    println!("{}",config.file_path);
    

    /*match config {
        Ok(my_data) => {
println!("{}", my_data.query);
            println!("{}", my_data.file_path);
        },
        Err(err) => {
            println!("{}", err);
            process::exit(1);
        }    
    }*/
    if let Err(err)=read_file(config){
        println!("{}",err);
        process::exit(1);
    }
    
}



/*fn parse_config(args:&[String])->Config{
    let query=args[1].clone();
    let file_path=args[2].clone();

    Config{query,file_path}
}*/


