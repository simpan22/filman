use crate::app_state::{self, AppState};
use std::process::Command as Cmd;

pub enum CommandName {
    MkDir,
}

impl TryFrom<&str> for CommandName {
    type Error = anyhow::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "mkdir" => Ok(CommandName::MkDir),
            _ => Err(anyhow::format_err!("Not a valid command")),
        }
    }
}

pub struct Command {
    name: CommandName,
    arguments: Vec<String>,
}

impl TryFrom<&str> for Command {
    type Error = anyhow::Error;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let spl: Vec<_> = s.split(' ').collect();

        Ok(Command {
            name: spl[0].try_into()?,
            arguments: spl[1..].iter().map(|x| x.to_string()).collect(),
        })
    }
}

pub fn execute_command(cmd: Command, app_state: &mut AppState) -> anyhow::Result<()> {
    match cmd.name {
        CommandName::MkDir => {
            // FIXME: mkdir should create a dir in the current directory not
            // in the start dir.
            Cmd::new("mkdir").args(cmd.arguments).status()?;
            app_state.refresh()?;
        }
    }

    Ok(())
}
