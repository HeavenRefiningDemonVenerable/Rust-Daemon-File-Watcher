use notify::{event::CreateKind, Event, RecursiveMode, Result, Watcher};
use core::time;
use std::{path::Path, sync::mpsc};
use notify::EventKind;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{DateTime, Utc};

// use notify::event::CreateKind;
// use rand::distr::{Alphanumeric, SampleString};

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel::<Result<Event>>();

    // Use recommended_watcher() to automatically select the best implementation
    // for your platform. The `EventHandler` passed to this constructor can be a
    // closure, a `std::sync::mpsc::Sender`, a `crossbeam_channel::Sender`, or
    // another type the trait is implemented for.
    let mut watcher = notify::recommended_watcher(tx)?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(Path::new("C:\\Users\\demio\\Downloads"), RecursiveMode::Recursive)?;
    // Block forever, printing out events as they come in
    for res in rx {
        match res {
            // Ok(event) => println!("event: {:?}", event),
            Ok(event) => message_simplyfier(Ok(event)),
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}

fn message_simplyfier(message: Result<Event>) -> (){
    let event = message.unwrap();
    let path = event.paths.get(0).unwrap().clone();
    match event.kind {
        EventKind::Remove(_) => println!("This was deleted: {}",path.display()),
        EventKind::Modify(_) => println!("File was modified"),
        EventKind::Create(_) => if fs::metadata(&path).expect("error with metadata, perhaps there is an error with the file path").is_dir() {
            let metadata = fs::metadata(&event.paths.get(0).unwrap()).expect("no metadata found, perhaps a problem with the path");
            if let Ok(time) = metadata.created(){ //use result here for error handling, actually use .expect
                match time.duration_since(SystemTime::UNIX_EPOCH) {
                    Ok(n) => {
                        let dt = DateTime::from_timestamp_secs(n.as_secs().try_into().unwrap());
                        println!("A folder: '{}' was created within: '{}' at time {:?} ", path.file_name().unwrap().display(), path.parent().unwrap().display(), dt.unwrap().to_string());
                    },
                    Err(_) => panic!("Great googly moogly, system time seems to be before the EPOCH"),
                    
                }
            }
            
        }else if fs::metadata(&path).expect("error with metadata, perhaps there is an error with the file path").is_file(){
            println!("A file: '{}' was created within: '{}' ", path.file_name().unwrap().display(), path.parent().unwrap().display() );
        },
        EventKind::Access(_) => println!("File was accessed"),
        _ => println!("Something else happened")
        
    }
    
}

