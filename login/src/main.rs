//! `login` prompts a username/password to authenticate on a new session.

#![feature(never_type)]

use std::env;
use std::error::Error;
use std::path::Path;
use std::process::exit;
use std::process::Command;
use std::time::Duration;
use utils::prompt::prompt;
use utils::user;
use utils::user::User;
use utils::util;

/// Switches to the given user after login is successful.
///
/// Arguments:
/// - `logname` is the name of the user used to login.
/// - `user` is the user to switch to.
fn switch_user(logname: &str, user: &User) -> Result<!, Box<dyn Error>> {
    let User {
        uid,
        gid,
        home,
        interpreter,
        ..
    } = user;

    // Changing user
    user::set(*uid, *gid)?;

    // TODO Execute without fork
    // Running the user's program
    let status = Command::new(interpreter)
        .current_dir(home)
        .env("HOME", home)
        .env("LOGNAME", logname)
        .status()
        .map_err(|_| format!("login: Failed to run shell `{interpreter}`"))?;

    // Exiting with the shell's status
    exit(status.code().unwrap());
}

fn main() {
    let _args = env::args(); // TODO Parse and use

    loop {
        println!();

        // Get user prompt
        let user_prompt = format!("{} login: ", util::get_hostname());

        // Prompt login and password
        let login = prompt(Some(&user_prompt), false).unwrap_or_else(|| exit(1));
        let pass = prompt(None, true).unwrap_or_else(|| exit(1));

        // Read users lists
        let passwd = user::read_passwd(Path::new(user::PASSWD_PATH)).unwrap_or_else(|e| {
            eprintln!("Cannot read passwd file: {e}");
            exit(1);
        });
        let shadow = user::read_shadow(&Path::new(user::SHADOW_PATH)).ok();

        // Get user from prompted login
        let user_entry = passwd.into_iter().find(|e| e.login_name == login);

        let interval = Duration::from_millis(1000);
        util::exec_wait(interval, || {
            if let Some(user_entry) = user_entry {
                // Checking password against user entry
                let correct = user_entry.check_password(&pass).unwrap_or_else(|| {
                    if let Some(shadow) = shadow {
                        shadow
                            .into_iter()
                            .filter(|e| e.login_name == login)
                            .map(|e| e.check_password(&pass))
                            .next()
                            .unwrap_or(false)
                    } else {
                        false
                    }
                });

                if correct {
                    switch_user(&login, &user_entry).unwrap_or_else(|e| {
                        eprintln!("login: {e}");
                        exit(1);
                    });
                }
            }
        });

        eprintln!("Login incorrect");
    }
}
