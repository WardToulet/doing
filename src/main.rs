mod record_store;
mod record;

use crate::record::{Current, Record};
use crate::record_store::RecordStore;

use structopt::StructOpt;

#[derive(StructOpt)]
enum Command {
    On {
        item: String,
    },

    Done,

    What,
}

fn main() {

    match Command::from_args() {
        Command::On { item } => { Record::start(&item).save().unwrap(); }
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
        Command::What => todo!(),
    }
}
