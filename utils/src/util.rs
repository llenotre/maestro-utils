//! This module implements utility functions.

use std::thread;
use std::time::Duration;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

/// Returns the current timestamp since the Unix epoch.
pub fn get_timestamp() -> Duration {
    SystemTime::now()
		.duration_since(UNIX_EPOCH)
		.expect("System clock panic!")
}

/// Executes the closure `f`.
/// If the closure returns Ok, the function returns directly. If it return an error, the function
/// ensures the execution takes at least the given duration `d`.
pub fn exec_wait<T, F: FnOnce() -> T>(d: Duration, f: F) -> T {
	let start = get_timestamp();

	let result = f();

	// Waiting until the given amount of time is spent
	while get_timestamp() < start + d {
		thread::sleep(Duration::from_millis(1));
	}

	result
}
