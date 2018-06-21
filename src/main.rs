#[macro_use]
extern crate clap;
use std::env;
use std::path::Path;
use std::process::*;

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
    // terrible hack to ensure that we build all packages if none given
    let package_bool  = if package == "#" { "" } else { "--only-package="};
    let username = matches.value_of("USERNAME").unwrap();
    let ip = matches.value_of("IP").unwrap();
    let level: u8 = matches.value_of("LEVEL").unwrap().parse::<u8>().unwrap();

    let ros2_dir = env::var("ROS2_DIR").unwrap_or(format!("/home/{}/ros2_ws/", env::var("USER").unwrap()));
    let package_dir = env::var("PACKAGE_DIR").unwrap_or(format!("/home/{}/ros2_ws/", env::var("USER").unwrap()));

    //println!("values are: {}, {}, {}, {}, {}", ros2_dir, package_dir, package, ip, level);


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
                    package_dir, username, ip, package_dir
                    )
                );

            println!("stdout:\n{}", String::from_utf8_lossy(&output.stdout));
            println!("Package Synced!\n");
        }

        if level == 3  {
            println!("Running package on SoC...\nPlease choose your executable:\n\n");
            let output = run_bash(
                &format!(
                    "ssh {}@{} '(find {}/build/{} -maxdepth 1-type f ! -name \"*.*\" -executable)'"
                    , username, ip, package_dir, package
                    )
                );

            println!("stdout:\n{}", String::from_utf8_lossy(&output.stdout));
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
    use super::*;

    #[test]
    fn main_binary_works() {
        assert_cli::Assert::main_binary()
            .with_env(assert_cli::Environment::inherit().insert("ROS2_DIR", "/home/amar/github/julia_ros_ws"))
            .with_env(assert_cli::Environment::inherit().insert("PACKAGE_DIR", "/home/amar/github/julia_code"))
            .stderr()
            .is("")
            .unwrap();
    }
}
