#[macro_use]
extern crate clap;
extern crate yaml_rust;
use std::env;

use clap::App;

fn main()  ->   std::io::Result<()> {

    let yaml_file = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml_file).get_matches();

    let package = matches.value_of("PACKAGE").unwrap();
    let ip = matches.value_of("IP").unwrap();

    let ros2_dir = env::var("ROS2_DIR").unwrap_or(format!("/home/{}/ros2_ws/", env::var("USER").unwrap()));
    let package_dir = env::var("PACKAGE_DIR").unwrap_or(format!("/home/{}/ros2_ws/", env::var("USER").unwrap()));

    println!("values are: {}, {}, {}, {}", ros2_dir, package_dir, package, ip);
    Ok(())
}
