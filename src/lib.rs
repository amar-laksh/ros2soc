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
    pub fn new(matches: clap::ArgMatches) -> Result<Self, &'static str> {
        let package: String = matches.values_of("PACKAGE").unwrap().collect();
        let destination = matches.values_of("DEST").unwrap().collect();

        // terrible hack to ensure that we build all packages if none given
        let package_prefix = if package == "#" {
            String::from("")
        } else {
            String::from("--packages-select ")
        };
        let username = matches.values_of("USERNAME").unwrap().collect();
        let ip = matches.values_of("IP").unwrap().collect();
        let level: u8 = matches.value_of("LEVEL").unwrap().parse::<u8>().unwrap_or(3);

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

    pub fn cross_compile_package(&mut self) {
        if Path::new(&self.ros2_dir).exists() && Path::new(&self.package_dir).exists() {
            println!("\nBuilding your package...\n");
            let output = _run_bash(&format!(
                ". {}/install/setup.bash && cd {} && colcon build {}{}",
                self.ros2_dir, self.package_dir, self.package_prefix, self.package
            ));

            println!("stdout:\n{}", String::from_utf8_lossy(&output.stdout));
            println!("Package Built!\n");
        }
    }

    pub fn sync_package(&mut self) {
        println!("Syncing package with SoC...\n");
        let output = _run_bash(&format!(
            "rsync -avz --del {}/ {}@{}:{}/",
            self.package_dir, self.username, self.ip, self.destination
        ));

        println!("stdout:\n{}", String::from_utf8_lossy(&output.stdout));
        println!("Package Synced!\n");
    }

    pub fn run_package(&mut self) {
        println!("Running package on SoC...\nPlease choose your executable:\n\n");
        let output = _run_bash(&format!(
            "ssh {}@{} '(find {}/build/{} -maxdepth 1 -type f ! -name \"*.*\" -executable)'",
            self.username, self.ip, self.destination, self.package
        ));
        let _input = _choose_exectuable(&output);
        println!("Running executable {}  on SoC...\n", _input);
        let output = _run_bash(&format!(
            "ssh {}@{} '{}'",
            self.username,
            self.ip,
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
    use super::*;

    #[test]
    fn _run_bash_works() {
        assert_eq!(_run_bash("echo 1").status.success(), true);
        assert_eq!(_run_bash("123").status.success(), false);
    }

}
