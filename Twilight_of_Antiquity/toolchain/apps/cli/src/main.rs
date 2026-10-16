use saecvar_workspace::SourcePaths;
use std::{env, path::Path, process::ExitCode};
mod syntax_command;

fn main() -> ExitCode {
    let args: Vec<_> = env::args_os().skip(1).collect();
    if args.len() == 2 && args[0] == "parse" {
        return syntax_command::run(Path::new(&args[1]));
    }
    if args.len() != 3 || args[0] != "inspect-paths" {
        eprintln!("usage: saecvar-cli inspect-paths <game-data-root> <mod-root>");
        eprintln!("       saecvar-cli parse <utf8-text-script>");
        return ExitCode::from(2);
    }
    match SourcePaths::inspect(Path::new(&args[1]), Path::new(&args[2])) {
        Ok(paths) => {
            println!("game_read_only={}", paths.game_read_only.display());
            println!("mod_authored={}", paths.mod_authored.display());
            println!("engine_validation=NOT_RUN (path inspection only)");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("path inspection failed: {error}");
            ExitCode::FAILURE
        }
    }
}
