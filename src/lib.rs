use std::error::Error;
use std::fs;


pub struct Config{
    pub query:String,
    pub file_path:String
}

impl Config{
    pub fn new(args:&[String])->Result<Config,&str>{
        if args.len()<3{
            return Err("not enough arguments passed");
        }

        let query=args[1].clone();
        let file_path=args[2].clone();

        Ok(Config{query,file_path})
    }
}


pub fn read_file(config:Config)->Result<(),Box<dyn Error>>{
    //let content=fs::read_to_string(config.file_path).expect("unable to read the file");
    let content=fs::read_to_string(config.file_path)?;


    println!("\n{}",content);

    return Ok(());
}
