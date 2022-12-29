extern crate lock;

use lock::{FailureReason, lock};
use notify_rust::{Notification};
use std::{thread, time};

// A small CLI tool for programmatically locking my laptop and alerting me with
// a message. I intend to use this for implementing the Pomodoro method, but on
// my own terms.
fn main() {
    lock_screen();

    // @dev when testing on my M1 mac my notification would immediately
    // disappear if it happened at the same time as lock_screen(); so
    // here we wait an arbitrary & short amount of time (500 ms), which
    // solves the problem for me :)
    let ten_millis = time::Duration::from_millis(500);
    thread::sleep(ten_millis);


    notify("floop", "bloop")
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

fn notify(summary: &str, body: &str) {
    Notification::new()
    .summary(summary)
    .body(body)
    .timeout(0) // this however is
    .show();
}