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

use std::fs::File;
use std::path::Path;

use chrono::prelude::*;
use ini::Ini;
use serde_json;
use console::style;


#[derive(Serialize, Deserialize, Clone)]
pub struct Ice {
    description: String,
    message: String,
    emails: Vec<String>,
    active: bool,
    send_date: Option<DateTime<Local>>
}

impl Ice {
    /// Create a new ICE mail
    ///
    /// # Arguments
    ///
    /// * `description` - Short description for the mail
    /// * `message` - Mail contents
    pub fn new(description: String, message: String) -> Ice {
        Ice {
            description: description,
            message: message,
            emails: Vec::new(),
            active: false,
            send_date: None
        }
    }

    /// Get the short description of the ICE mail
    pub fn get_description(&self) -> String {
        self.description.clone()
    }

    /// Update the short description of the ICE mail
    ///
    /// # Arguments
    ///
    /// * `description` - New description to use
    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }

    /// Get the date in yyyy-mm-dd format
    pub fn get_date(&self) -> String {
        match self.send_date {
            Some(v) => v.format("%F %R").to_string(),
            None => "Unknown".to_string()
        }
    }

    /// Update the date of the ICE mail
    ///
    /// # Arguments
    ///
    /// * `new_date` - New date to use (or None if the mail is disabled)
    pub fn set_date(&mut self, new_date: Option<DateTime<Local>>) {
        self.send_date = new_date;
    }

    /// Get the mail recipients
    pub fn get_emails(&self) -> &Vec<String> {
        &self.emails
    }

    /// Update the recipients of the ICE mail
    ///
    /// # Arguments
    ///
    /// * `emails` - New recipients
    pub fn set_emails(&mut self, emails: &Vec<String>) {
        self.emails.clear();
        self.emails.extend_from_slice(emails);
    }

    /// Get the mail content
    pub fn get_message(&self) -> String {
        self.message.clone()
    }

    /// Update the short description of the ICE mail
    ///
    /// # Arguments
    ///
    /// * `message` - New message to use
    pub fn set_message(&mut self, message: String) {
        self.message = message;
    }

    /// Get the status of the ICE mail
    pub fn get_status(&self) -> bool {
        self.active
    }

    /// Update the status of the ICE mail
    ///
    /// # Arguments
    ///
    /// * `status` - New status for the ICE mail
    pub fn set_status(&mut self, status: bool) {
        self.active = status;
    }

    /// Get a short line specifying the status of the ICE and the scheduled
    /// date if active
    pub fn get_status_line(&self) -> String {
        format!(
            "{} ~> {} {}",
            self.description,
            if self.active {style("Active").green()} else {style("Inactive").red()},
            if self.active {format!("({})", self.get_date())} else {"".to_string()}
        )
    }
}

/// Obtain a list of ICE mails from the JSON file
///
/// # Arguments
///
/// * `conf` - Application configuration
pub fn get_ices(conf: &Ini) -> Result<Vec<Ice>, &'static str> {
    let json_section = conf.section(Some("json".to_owned())).unwrap();
    let json_path = Path::new(json_section.get("path").unwrap());

    if !json_path.exists() {
        return Err("JSON file does not exist");
    }

    let file = File::open(json_path).unwrap();
    let ices: Vec<Ice> = serde_json::from_reader(file).unwrap();

    Ok(ices)
}

/// Write a list of ICE mails into the JSON file
///
/// # Arguments
///
/// * `conf` - Application configuration
/// * `ices` - List of ICE mails
pub fn write_ices(conf: &Ini, ices: &Vec<Ice>) -> serde_json::Result<()> {
    let json_section = conf.section(Some("json".to_owned())).unwrap();
    let json_path = Path::new(json_section.get("path").unwrap());

    let file = File::create(json_path).unwrap();

    serde_json::to_writer(file, &ices)
}
