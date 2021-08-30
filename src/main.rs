mod record_store;
mod record;

use crate::record::{Current, Record};
use crate::record_store::RecordStore;

use structopt::StructOpt;

#[derive(StructOpt)]
enum Command {
    Now {
        item: String,
    },

    Done,

    What,
}

fn main() {

    match Command::from_args() {
        Command::Now { item } => { Record::start(&item).save().unwrap(); }
        Command::Done => { 
            match Current::open() {
                Some(current) => {
                    let record = current.stop();
                    Current::clear();
                    
                    println!("Stop tracking {:#?}", record);

                    let mut record_store = record_store::CsvRecordStore::open();
                    record_store.push(record);
                    record_store.save();
                }
                None => {
                    println!("You are currently not tracking anything, to start tracking type `working on <your item>`");
                },
            }
        },
        Command::What => {
            match Current::open() {
                Some(current) => {
                    println!("Working on {}, for {} seconds", current.item(), chrono::Local::now().timestamp() - current.starting_timestamp());
                }, 
                None => todo!(),
            }
        },
    }
}
