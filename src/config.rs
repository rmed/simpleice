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

//! Configuration file operations

use std::env::home_dir;
use std::path::PathBuf;
use ini::Ini;

/// Attempt to read a configuration file from the home directory
pub fn read_config() -> Result<Ini, &'static str> {
    let home = home_dir();
    if home.is_none() {
        return Err("Cannot find home directory");
    }

    let mut conf_path = PathBuf::new();
    conf_path.push(home.unwrap());
    conf_path.push(".simpleice");

    if !conf_path.exists() {
        return Err("Cannot find configuration file");
    }

    // Load config
    match Ini::load_from_file(conf_path.to_str().unwrap()) {
        Ok(v) => Ok(v),
        Err(_) => Err("Failed to load configuration file")
    }
}


/// Write a basic configuration file
pub fn write_empty_config() -> Result<(), &'static str> {
    let home = home_dir();
    if home.is_none() {
        return Err("Cannot find home directory");
    }

    let mut conf_path = PathBuf::new();
    conf_path.push(home.unwrap());
    conf_path.push(".simpleice");

    if conf_path.exists() {
        return Err("Configuration file already exists");
    }

    // Create config
    let mut conf = Ini::new();
    conf.with_section(Some("mail".to_owned()))
        .set("address", "")
        .set("password", "")
        .set("server", "")
        .set("port", "");
    conf.with_section(Some("json".to_owned()))
        .set("path", "");

    match conf.write_to_file(conf_path.to_str().unwrap()) {
        Ok(_) => Ok(()),
        Err(e) => Err("Failed to write configuration file"),
    }
}
