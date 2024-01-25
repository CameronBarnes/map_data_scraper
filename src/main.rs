use std::process::Command;

use once_cell::sync::Lazy;

mod types;
mod parsing;

static IS_WINDOWS: bool = cfg!(windows);
static HAS_RSYNC: Lazy<bool> = Lazy::new(check_for_rsync);

#[must_use]
pub fn check_for_rsync() -> bool {
    let result = Command::new("which").arg("rsync").output();

    if let Ok(output) = result {
        output.status.success()
    } else {
        false
    }
}

fn main() {
    println!("Hello, world!");
}
