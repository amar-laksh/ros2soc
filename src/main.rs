#[macro_use]
extern crate ros2soc;
extern crate clap;

use clap::*;
use ros2soc::Ros2soc;

fn main() -> std::io::Result<()> {
    let yaml_file = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml_file).get_matches();
    let conf = Ros2soc::new(matches);
    Ros2soc::cross_compile(&conf.unwrap());

    /*     println!("values are:\n\nros2_dir:{}\npackage_dir:{}\npackage:{}\nip:{}\nlevel:{}\n" */
    // , ros2_dir, package_dir, package, ip, level);
    /*  */

    Ok(())
}

#[cfg(test)]
mod tests {
    extern crate assert_cli;

    #[test]
    fn main_binary_works() {
        assert_cli::Assert::main_binary()
            .with_env(
                assert_cli::Environment::inherit()
                    .insert("ROS2_DIR", "/home/amar/github/julia_ros_ws"),
            )
            .with_env(
                assert_cli::Environment::inherit()
                    .insert("PACKAGE_DIR", "/home/amar/github/julia_code"),
            )
            .with_args(&["-l", "1"])
            .stderr()
            .is("")
            .unwrap();
    }
}
