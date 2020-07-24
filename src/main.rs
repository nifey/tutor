extern crate clap;
use clap::{App, Arg, SubCommand};
use std::fs::File;

mod parse;

fn main() {
    let matches = App::new("Tutor")
        .version("0.1")
        .about("Command line tutorials")
        .author("Abdun Nihaal")
        .subcommand(
            SubCommand::with_name("list")
                .alias("ls")
                .about("Lists the sections and excercises")
                .arg(
                    Arg::with_name("section")
                        .help("section that you want to list")
                        .index(1),
                ),
        )
        .subcommand(
            SubCommand::with_name("check")
                .alias("c")
                .about("Checks if the solution is correct")
                .arg(
                    Arg::with_name("file")
                        .help("The solution file to check")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            SubCommand::with_name("lesson")
                .alias("l")
                .about("Prints the current lesson")
                .arg(
                    Arg::with_name("lesson_index")
                        .help("The lesson whose text is to be printed")
                        .index(1),
                ),
        )
        .subcommand(
            SubCommand::with_name("task")
                .alias("t")
                .about("Prints the current task")
                .arg(
                    Arg::with_name("lesson_index")
                        .help("The lesson whose task is to be printed")
                        .index(1),
                ),
        )
        .subcommand(
            SubCommand::with_name("hint")
                .alias("h")
                .about("Prints a hint if provided in the lesson")
                .arg(
                    Arg::with_name("next")
                        .help("Opens the next hidden hint")
                        .index(1),
                ),
        )
        .get_matches();

    if let Ok(info_file) = File::open("tutorinfo.toml") {
    } else {
        println!("Run tutor from a tutorial directory containing tutorinfo.toml file");
        return;
    }

    if let Some(cmd_matches) = matches.subcommand_matches("list") {
        let section_str = cmd_matches.value_of("section").unwrap_or("");
    }

    if let Some(cmd_matches) = matches.subcommand_matches("check") {
        let file = cmd_matches.value_of("file").unwrap();
        let current_file = File::open("lessons/section1/lesson1").unwrap();
        if let Ok(shell_script) = parse::get_section(&current_file, "check") {
            if parse::check(shell_script, file) {
                println!("Correct");
                print!(
                    "{}",
                    parse::get_section(&current_file, "solution")
                        .unwrap_or("No solution text for this lesson".to_string())
                );
            } else {
                println!("Incorrect. Try again");
            }
        }
    }

    if let Some(cmd_matches) = matches.subcommand_matches("lesson") {
        let current_file = File::open("lessons/section1/lesson1").unwrap();
        let lesson_index = cmd_matches.value_of("lesson_index").unwrap_or("");
        print!(
            "{}",
            parse::get_section(&current_file, "lesson")
                .unwrap_or("No help text for this lesson".to_string())
        );
    }

    if let Some(cmd_matches) = matches.subcommand_matches("task") {
        let current_file = File::open("lessons/section1/lesson1").unwrap();
        let lesson_index = cmd_matches.value_of("lesson_index").unwrap_or("");
        print!(
            "{}",
            parse::get_section(&current_file, "task")
                .unwrap_or("No task for this lesson".to_string())
        );
    }

    if let Some(cmd_matches) = matches.subcommand_matches("hint") {
        let current_file = File::open("lessons/section1/lesson1").unwrap();
        let next = cmd_matches.value_of("section").is_some();
        print!(
            "{}",
            parse::get_section(&current_file, "hint")
                .unwrap_or("No hint for this lesson".to_string())
        );
    }
}
