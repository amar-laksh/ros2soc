#[macro_use]
extern crate clap;
use std::env;
use std::path::Path;
use std::process::*;
use std::io;

use clap::App;

fn run_bash(command: &str) -> std::process::Output {
    let output = Command::new("/usr/bin/bash")
        .args(&["-c", command
        ]).output().expect("failed to execeute process");

    if !output.status.success() {
        println!("stderr:\n{}", String::from_utf8_lossy(&output.stderr));
        exit(256)
    }
    return output
}

fn main() -> std::io::Result<()> {

    let yaml_file = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml_file).get_matches();

    let package = matches.value_of("PACKAGE").unwrap();
    let destination = matches.value_of("DEST").unwrap_or("/home/pi/ros2_package");
    // terrible hack to ensure that we build all packages if none given
    let package_bool  = if package == "#" { "" } else { "--only-package="};
    let username = matches.value_of("USERNAME").unwrap();
    let ip = matches.value_of("IP").unwrap();
    let level: u8 = matches.value_of("LEVEL").unwrap().parse::<u8>().unwrap();

    let ros2_dir = env::var("ROS2_DIR").unwrap_or(format!("/home/{}/ros2_ws/", env::var("USER").unwrap()));
    let package_dir = env::var("PACKAGE_DIR").unwrap_or(format!("/home/{}/ros2_ws/", env::var("USER").unwrap()));

/*     println!("values are:\n\nros2_dir:{}\npackage_dir:{}\npackage:{}\nip:{}\nlevel:{}\n" */
             // , ros2_dir, package_dir, package, ip, level);
/*  */

    if Path::new(&ros2_dir).exists() && Path::new(&package_dir).exists() {
        // Check if package is fine and Start with the cross-compiling
        if level >= 1 {
            println!("\nBuilding your package...\n");
            let output = run_bash(
                &format!(
                    ". {}/install/setup.bash && cd {} && ament build {}{}"
                    , ros2_dir, package_dir, package_bool, package
                    )
                );

            println!("stdout:\n{}", String::from_utf8_lossy(&output.stdout));
            println!("Package Built!\n");
        }

        if level >= 2 {
            println!("Syncing package with SoC...\n");
            let output = run_bash(
                &format!(
                    "rsync -avz --del {}/ {}@{}:{}/",
                    package_dir, username, ip, destination
                    )
                );

            println!("stdout:\n{}", String::from_utf8_lossy(&output.stdout));
            println!("Package Synced!\n");
        }

        if level == 3  {
            println!("Running package on SoC...\nPlease choose your executable:\n\n");
            let output = run_bash(
                &format!(
                    "ssh {}@{} '(find {}/build/{} -maxdepth 1 -type f ! -name \"*.*\" -executable)'"
                    , username, ip, destination, package
                    )
                );
            let mut n_execs: u32 = 0;
            for i in String::from_utf8_lossy(&output.stdout).lines() {
                n_execs += 1;
                println!("{}:{}\n", n_execs, i);
            }
            let mut _text = String::new();
            io::stdin().read_line(&mut _text).expect("failed to read line");
            let _input = match _text.trim().parse::<u32>() {
                Ok(_input) => {
                    if _input > n_execs {
                        println!("Sorry you entered the wrong number for the executable");
                        exit(256);
                    }
                    _input
                },
                Err(e) => {
                    println!("please input a number ({})", e);
                    exit(256);
                }
            };
            
            println!("Running executable {}  on SoC...\n", _input);
            let output = run_bash(
                &format!(
                    "ssh {}@{} '{}'"
                    , username, ip, String::from_utf8_lossy(&output.stdout).lines().nth(_input as usize).unwrap()
                    )
                );
            println!("output:\n{:?}\n", &output.stdout);
        }

        else if level < 1 || level > 3 {
            println!("Sorry you entered a wrong level. :(");
            exit(256);
        }

    } else {
        // Download ROS2 and cross-compile it

    }
    Ok(())
}


#[cfg(test)]
mod tests {
    extern crate assert_cli;

    #[test]
    fn main_binary_works() {
        assert_cli::Assert::main_binary()
            .with_env(assert_cli::Environment::inherit().insert("ROS2_DIR", "/home/amar/github/julia_ros_ws"))
            .with_env(assert_cli::Environment::inherit().insert("PACKAGE_DIR", "/home/amar/github/julia_code"))
            .with_args(&["-l","1"])
            .stderr()
            .is("")
            .unwrap();
    }
}
