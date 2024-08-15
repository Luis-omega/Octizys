mod arguments_parser;
mod common_configuration;
mod run;

use arguments_parser::ParsedConfiguration::*;
use octizys_compiler::compiler::compile;
use octizys_documentation::documentation::generate_documentation;
use octizys_error::report_if_error;
use octizys_formatter::formatter::format_them;
use run::run::run;

fn main() -> () {
    match arguments_parser::get_raw_arguments() {
        Ok(raw_arguments) => {
            match arguments_parser::parse_arguments(raw_arguments) {
                Ok(parsed_arguments) => match parsed_arguments {
                    Compile(config) => report_if_error(compile(config)),
                    Run(config) => report_if_error(run(config)),
                    Documentation(config) => {
                        report_if_error(generate_documentation(config))
                    }
                    // format name causes a problem?
                    Format(config) => report_if_error(format_them(config)),
                },
                Err(x) => {
                    print!("Error happened!");
                    print!("{}", x)
                }
            }
        }
        Err(x) => {
            print!("Error happened!");
            println!("{}", x)
        }
    }
}
