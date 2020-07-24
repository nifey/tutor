extern crate clap;

mod index;
mod info;
mod stat;
mod util;

use clap::{App, Arg, SubCommand};
use info::Info;
use stat::Stat;

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
            SubCommand::with_name("instruction")
                .alias("i")
                .about("Gives introductary instructions about the tutorial"),
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

    let info: Info = info::read_tutorinfo();
    let mut stat: Stat;
    let read_option = stat::read_tutorstat();
    if read_option.is_none() {
        stat = stat::new(
            info.get_tutorial_version(),
            "1.1".to_string(),
            info.get_total_lessons(),
        );
        println!("{}", info.get_instructions());
    } else {
        stat = read_option.unwrap();
    }

    let current_file: String;
    let lesson_file_result = info.get_lesson_file(stat.get_current_lesson());
    if lesson_file_result.is_ok() {
        current_file = lesson_file_result.unwrap();
    } else {
        println!("{}", lesson_file_result.err().unwrap());
        return;
    }

    if let Some(cmd_matches) = matches.subcommand_matches("list") {
        let section_str = cmd_matches.value_of("section").unwrap_or("");
    }

    if let Some(cmd_matches) = matches.subcommand_matches("instruction") {
        println!("{}", info.get_instructions());
    }

    if let Some(cmd_matches) = matches.subcommand_matches("check") {
        let file = cmd_matches.value_of("file").unwrap();
        if let Ok(shell_script) = util::get_section(&current_file, "check") {
            if util::check(shell_script, file) {
                println!("Correct");
                print!(
                    "{}",
                    util::get_section(&current_file, "solution")
                        .unwrap_or("No solution text for this lesson".to_string())
                );
                //TODO Add to finished tasks in the stat structure
                if let Some(next_lesson) =
                    info.get_next_lesson(index::new_from_string(stat.get_current_lesson()).unwrap())
                {
                    stat.set_current_lesson(next_lesson);
                } else {
                    // TODO Check for unfinished tasks before congratulating
                    println!("Congratulations! you have completed the tutorial");
                }
            } else {
                println!("Incorrect. Try again");
            }
        }
    }

    if let Some(cmd_matches) = matches.subcommand_matches("lesson") {
        let lesson_index = cmd_matches.value_of("lesson_index").unwrap_or("");
        print!(
            "{}",
            util::get_section(&current_file, "lesson")
                .unwrap_or("No help text for this lesson".to_string())
        );
    }

    if let Some(cmd_matches) = matches.subcommand_matches("task") {
        let lesson_index = cmd_matches.value_of("lesson_index").unwrap_or("");
        print!(
            "{}",
            util::get_section(&current_file, "task")
                .unwrap_or("No task for this lesson".to_string())
        );
    }

    if let Some(cmd_matches) = matches.subcommand_matches("hint") {
        let next = cmd_matches.value_of("section").is_some();
        //TODO get the number of hints used from tutorstats
        //Also update when next is set
        let num_hints = 5;
        if let Ok(hints) = util::get_section(&current_file, "hint") {
            let (text, hints_read) =
                util::get_n_hints(hints, num_hints).unwrap_or(("".to_string(), 0));
            print!("{}", text);
            if hints_read < num_hints {
                println!("Only {} hints available for this task", hints_read);
            }
        } else {
            println!("No hints for this task");
        }
    }

    stat::write_tutorstat(stat);
}
