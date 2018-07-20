extern crate clap;
extern crate ros2soc;

use clap::*;
use ros2soc::Ros2soc;

fn main() -> std::io::Result<()> {
    let yaml_file = load_yaml!("./cli.yml");
    let matches = App::from_yaml(yaml_file).get_matches();
    let mut ros2soc = Ros2soc::new(matches).unwrap();
    match &ros2soc.level {
        1 => ros2soc.cross_compile_package(),
        2 => {
            ros2soc.cross_compile_package();
            ros2soc.sync_package()
        }
        3 => {
            ros2soc.cross_compile_package();
            ros2soc.sync_package();
            ros2soc.run_package()
        }
        _ => {
            println!("wrong level entered!");
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    extern crate assert_cli;

    #[test]
    fn no_args_error_works() {
        assert_cli::Assert::main_binary().fails().unwrap();
    }

    #[test]
    fn level_arg_works() {
        assert_cli::Assert::main_binary()
            .with_args(&["1"])
            .succeeds()
            .unwrap();
    }
}
