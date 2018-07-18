extern crate clap;
use std::env;
use std::io::stdin;
use std::path::Path;
use std::process::*;
use std::result::Result;

pub struct Ros2soc {
    pub package: String,
    pub destination: String,
    pub ip: String,
    pub username: String,
    pub ros2_dir: String,
    pub package_dir: String,
    pub level: u8,
    pub package_prefix: String,
}

fn _run_bash(command: &str) -> std::process::Output {
    let output = Command::new("/usr/bin/bash")
        .args(&["-c", command])
        .output()
        .expect("failed to execeute process");

    if !output.status.success() {
        println!("stderr:\n{}", String::from_utf8_lossy(&output.stderr));
        exit(256)
    }
    return output;
}

fn _choose_exectuable(output: &std::process::Output) -> usize {
    let n_execs = String::from_utf8_lossy(&output.stdout).lines().count();
    let mut _text = String::new();
    stdin().read_line(&mut _text).expect("failed to read line");
    let _input = match _text.trim().parse::<usize>() {
        Ok(_input) => {
            if _input > n_execs {
                println!("Sorry you entered the wrong number for the executable");
                exit(256);
            }
            _input
        }
        Err(e) => {
            println!("please input a number ({})", e);
            exit(256);
        }
    };
    return _input;
}

impl Ros2soc {
    pub fn new(matches: clap::ArgMatches) -> Result<Ros2soc, &'static str> {
        let package: String = matches.values_of("PACKAGE").unwrap().collect();
        let destination = matches.values_of("DEST").unwrap().collect();

        // terrible hack to ensure that we build all packages if none given
        let package_prefix = if package == "#" {
            String::from("")
        } else {
            String::from("--only-package=")
        };
        let username = matches.values_of("USERNAME").unwrap().collect();
        let ip = matches.values_of("IP").unwrap().collect();
        let level: u8 = matches.value_of("LEVEL").unwrap().parse::<u8>().unwrap();

        let ros2_dir =
            env::var("ROS2_DIR").unwrap_or(format!("/home/{}/ros2_ws/", env::var("USER").unwrap()));
        let package_dir = env::var("PACKAGE_DIR")
            .unwrap_or(format!("/home/{}/ros2_ws/", env::var("USER").unwrap()));
        Ok(Ros2soc {
            package,
            destination,
            ip,
            username,
            ros2_dir,
            package_dir,
            level,
            package_prefix,
        })
    }

    pub fn cross_compile(conf: &Ros2soc) {
        if Path::new(&conf.ros2_dir).exists() && Path::new(&conf.package_dir).exists() {
            println!("\nBuilding your package...\n");
            let output = _run_bash(&format!(
                ". {}/install/setup.bash && cd {} && ament build {}{}",
                conf.ros2_dir, conf.package_dir, conf.package_prefix, conf.package
            ));

            println!("stdout:\n{}", String::from_utf8_lossy(&output.stdout));
            println!("Package Built!\n");
        }
    }

    pub fn sync_package(conf: &Ros2soc) {
        println!("Syncing package with SoC...\n");
        let output = _run_bash(&format!(
            "rsync -avz --del {}/ {}@{}:{}/",
            conf.package_dir, conf.username, conf.ip, conf.destination
        ));

        println!("stdout:\n{}", String::from_utf8_lossy(&output.stdout));
        println!("Package Synced!\n");
    }

    pub fn run_package(conf: &Ros2soc) {
        println!("Running package on SoC...\nPlease choose your executable:\n\n");
        let output = _run_bash(&format!(
            "ssh {}@{} '(find {}/build/{} -maxdepth 1 -type f ! -name \"*.*\" -executable)'",
            conf.username, conf.ip, conf.destination, conf.package
        ));
        let _input = _choose_exectuable(&output);
        println!("Running executable {}  on SoC...\n", _input);
        let output = _run_bash(&format!(
            "ssh {}@{} '{}'",
            conf.username,
            conf.ip,
            String::from_utf8_lossy(&output.stdout)
                .lines()
                .nth(_input as usize)
                .unwrap()
        ));
        println!("output:\n{:?}\n", &output.stdout);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
