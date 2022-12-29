extern crate lock;

use lock::{FailureReason, lock};
use notify_rust::{Notification};
use std::{thread, time, env, ops::Index};
use clap::Parser;

const MINUTE: u64 = 1000 * 60;
/// A simple program to lock your screen and leave a desktop notification message <body> after some number of <minutes>
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Message body to appear in desktop notification
   #[arg(short)]
   body: Option<String>,

   /// Number of minutes to wait before locking
   #[arg(short)]
   minutes: Option<u64>,
}
// A small CLI tool for programmatically locking my laptop and alerting me with
// a message. I intend to use this for implementing the Pomodoro method, but on
// my own terms.
fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 2 {
        panic!("must provide BODY and MINUTES arguments")
    }

    let message = args.index(0);
    let minutes = args.index(1).parse::<u64>().unwrap();

    // wait MINUTES before locking
    let wait_time = time::Duration::from_millis(minutes * MINUTE);
    thread::sleep(wait_time);

    lock_screen();

    // @dev when testing on my M1 mac my notification would immediately
    // disappear if it happened at the same time as lock_screen(); so
    // here we wait an arbitrary & short amount of time (500 ms), which
    // solves the problem for me :)
    let small_delay = time::Duration::from_millis(500);
    thread::sleep(small_delay);


    notify(message)

    // let args = Args::parse();
    
    // let body = args.body.as_deref();
    // let minutes = args.minutes.as_ref();

    // match (body, minutes) {
    //     (Some(b), Some(m)) => {

    //         // @dev when testing on my M1 mac my notification would immediately
    //         // disappear if it happened at the same time as lock_screen(); so
    //         // here we wait an arbitrary & short amount of time (500 ms), which
    //         // solves the problem for me :)
    //         let ten_millis = time::Duration::from_millis(m * 1000);
    //         thread::sleep(ten_millis);

    //         lock_screen();


    //         notify(b)
    //     },
    //     _ => panic!("must provide BODY and MINUTES arguments"),

    // }
}

fn lock_screen() {
    let result = lock();

    match result {
        Err(FailureReason::CannotExecute) => {
            panic!("Lock command execution failure")
        },
        Err(FailureReason::LinuxCommandNotFound) => {
            // There is no xdg-screensaver, gnome-screensaver or dm-tool for linux,
            // do something else.
            panic!("There is no xdg-screensaver, gnome-screensaver or dm-tool for linux, do something else")
        }
        Ok(()) => (),
    }
}

fn notify(summary: &str) {
    Notification::new()
    .summary(summary)
    .timeout(0) // this however is
    .show().unwrap();
}