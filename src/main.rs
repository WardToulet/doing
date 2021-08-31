mod config;
mod current;
mod doing;
mod record;
mod store;

use structopt::StructOpt;

use crate::doing::Error;

#[derive(StructOpt)]
enum Command {
    Now { item: String, tag: Option<String>, },

    Done,

    What,
}

fn main() {
    let command = Command::from_args();
    let doing = doing::Doing::new();

    match command {
        Command::Now { tag, item } => {
            match doing.now(&item, tag) {
                Ok(current) => println!("You are now tracking `{}`", current),
                Err(Error::AlreadyTracking(current)) => {
                    println!("Already tracking `{}`.\nTo stop tracking use `doing done`", current)
                }
                Err(_) => {}
            }
        }
        Command::Done => {
            match doing.done() {
                Ok(record) => println!("{}", record),
                Err(_) => println!("You are not tracking anything currently,\nto start tracking ues `doing now <what you are doing>`"),
            }
            
        },
        Command::What => {
            match doing.what() {
                Some(current) => println!("You are {}.", current),
                None => println!("Not tracking anything."),
            }
        },
    }
}
