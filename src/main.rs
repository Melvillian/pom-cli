extern crate lock;

use lock::{FailureReason, lock};

fn main() {
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