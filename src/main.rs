mod commands;
use dev_term_io::CommandIo;
use std::env::{current_dir};
use std::io::{stdout, stdin, Write};
use colored::Colorize;
use std::process::Command;
fn main() -> std::io::Result<()> {

    /*
    cmdMap.RegisterCommand("os", commands.Os{})
    cmdMap.RegisterCommand("cd", commands.Cd{})
    cmdMap.RegisterCommand("download", commands.Download{})
    cmdMap.RegisterCommand("echo", commands.Echo{})
    cmdMap.RegisterCommand("ls", commands.Ls{FileColors: cfg.FileColors})
    cmdMap.RegisterCommand("exec", commands.Exec{})
    */

    println!("{}", "         / \\---------------------------,
         \\_,|                          |
            |    Welcome to Dev Term   |
            |  ,-------------------------
            \\_/________________________/ ".green());
    loop {
        let wd = current_dir()?;
        print!("[{}] ➜ ", wd.display().to_string().as_str().blue());

        stdout().flush()?;
        let mut input = String::new();
        stdin().read_line(&mut input)?;

        let mut args = input.trim().split(" ");
        let command = crate::commands::Command::read(&mut args);
        if let Ok(inner) = command {
                inner.execute()?;
        } else if let Err(e) = command {
            let args = input.trim().split(" ");
            let mut win_cmd = Command::new("cmd");
            win_cmd.arg("/C").args(args);
            win_cmd.status()?;
            //check error type as to determine wether we should closest_match
        }
        /*let name = args[0];
        match command {
            None => {
                let closest = command_map.closest_match(name);
                if closest == None {
                    let mut win_cmd = Command::new("cmd");
                    win_cmd.arg("/C").args(args);
                    win_cmd.status()?;
                } else {
                    println!("Unknown command {}, did you mean: {}?", name, closest.unwrap());
                }
            }
            _ => {
                // command.unwrap().execute(); todo
            }
        }*/

    }
}