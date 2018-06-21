#[macro_use]
extern crate clap;
extern crate yaml_rust;
use std::env;
use std::path::Path;
use std::process::*;

use clap::App;

fn run_bash(command: &str) -> bool {
    let output = Command::new("/usr/bin/bash")
        .args(&["-c", command
        ]).output().expect("failed to execeute process");

    if !output.status.success() {
        println!("stderr:\n{}", String::from_utf8_lossy(&output.stderr));
        exit(256)
    }
    return output.status.success()
}

fn main() -> std::io::Result<()> {

    let yaml_file = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml_file).get_matches();

    let package = matches.value_of("PACKAGE").unwrap();
    let username = matches.value_of("USERNAME").unwrap();
    let ip = matches.value_of("IP").unwrap();

    let ros2_dir = env::var("ROS2_DIR").unwrap_or(format!("/home/{}/ros2_ws/", env::var("USER").unwrap()));
    let package_dir = env::var("PACKAGE_DIR").unwrap_or(format!("/home/{}/ros2_ws/", env::var("USER").unwrap()));

    //println!("values are: {}, {}, {}, {}", ros2_dir, package_dir, package, ip);
    println!("\nBuilding your package...\n");

    if Path::new(&ros2_dir).exists() {
        // Check if package is fine and Start with the cross-compiling

        run_bash(
            &format!(
                ". {}/install/setup.bash && cd {} && ament build --only-package={}"
                , ros2_dir, package_dir, package
                )
            );

        println!("Package Built!\n");
        //println!("stdout:\n{}", String::from_utf8_lossy(&output.stdout));

        println!("Syncing package with SoC...\n");
        run_bash(
            &format!(
                "rsync -avz --del {}/ {}@{}:{}/",
                package_dir, username, ip, package_dir
                )
            );

        println!("Package Synced!\n");
        //println!("stdout:\n{}", String::from_utf8_lossy(&output.stdout));


        println!("Running package with SoC...\n");

    } else {
        // Download ROS2 and cross-compile it

    }
    Ok(())
}
