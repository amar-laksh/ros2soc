#[macro_use]
extern crate clap;
extern crate yaml_rust;
use std::fs::File;
use std::io::prelude::*;
use std::env;

use clap::App;
use yaml_rust::YamlLoader;


fn main()  ->   std::io::Result<()> {
    let yaml_file = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml_file).get_matches();
    let config = matches.value_of("CONFIG").unwrap_or("./default.yml");

    let package = matches.value_of("PACKAGE").unwrap();
    let ip = matches.value_of("IP").unwrap();
    println!("values are: {}, {}, {}", config, package, ip);

    //getting config file
    let mut file = File::open(config)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let default_dir = format!("/home/{}/ros2_ws/", env::var("USER").unwrap());

    let conf = YamlLoader::load_from_str(&contents).unwrap();
    let conf = &conf[0];



    let ros2_dir  =conf["ros2_dir"].as_str().unwrap_or(&default_dir);
    let package_dir = conf["package_dir"].as_str().unwrap_or(&default_dir);
    println!("values: {}, {}",  ros2_dir, package_dir);
    Ok(())
}
