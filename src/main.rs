// MIT License
//
// Copyright (c) 2017 Rafael Medina García <rafamedgar@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

/// Application launcher

extern crate chrono;
#[macro_use]
extern crate clap;
extern crate console;
extern crate dialoguer;
extern crate ini;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use clap::{Arg, App, SubCommand};
use console::Term;
use ini::Ini;

use parser::Ice;

mod commands;
mod config;
mod parser;


fn main() {
    let matches = App::new("simpleice")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Schedule emails in case of emergency")
        .arg(Arg::with_name("command")
            .possible_values(&[
                "create-config",
                "check",
                "daemon",
                "list",
                "new",
            ])
            .hide_possible_values(true)
            .required(true)
            .takes_value(true)
            .long_help("\
                Available commands:\n\n\
                create-config       Create empty configuration file\n\
                check               Check if there are scheduled emails to send\n\
                daemon              Run in daemon mode\n\
                list                List existing ICE mails\n\
                new                 Create new ICE mail"))
        .get_matches();

    let term = Term::stdout();
    let command = matches.value_of("command").unwrap();

    // Special case, create empty config
    if command == "create-config" {
        match config::write_empty_config() {
            Ok(_) => term.write_line("Empty config file created in ~/.simpleice"),
            Err(e) => term.write_line(e)
        };

        return;
    }

    // Check config
    let conf = match config::read_config() {
        Ok(v) => {v},
        Err(e) => {
            term.write_line(e);
            term.write_line(
                "You can create an empty configuration file it using the `create-config` command"
            );
            return;
        }
    };

    // Check command to run
    match command {
        "check" => (),
        "daemon" => (),
        "list" => commands::list_ices(&term, &conf),
        "new" => commands::create_ice(&term, &conf),
        _ => ()
    }
}
