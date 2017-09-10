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

/// Application commands

use std::error::Error;

use console::{Term, style};
use dialoguer::{Confirmation, Editor, Input, Select};
use ini::Ini;

use parser;
use parser::Ice;

/// Create a new ICE mail
///
/// This function asks the user for a short description and the contents of the
/// mail.
///
/// # Arguments
///
/// * `term` - Terminal abstraction
/// * `conf` - Application configuration
pub fn create_ice(term: &Term, conf: &Ini) {
    term.write_line("Creating a new ICE mail");
    term.write_line("You need to provide a short description and message to send\n");

    // Ask for description
    let description = Input::new("Please specify a short description").interact().unwrap();

    // Ask for message
    term.write_line("Opening your default editor to write the message...");
    let message = Editor::new().edit("Please write your message").unwrap();

    if message.is_none() {
        // Need a message
        term.write_line("You need to specify a message. Aborting...");
        return;
    }

    // Create new ICE
    let new_ice = Ice::new(description, message.unwrap());

    let mut ices = match parser::get_ices(&conf) {
        Ok(v) => v,
        // File may not exist yet, will be created later
        Err(_) => Vec::new()
    };
    ices.push(new_ice);

    match parser::write_ices(&conf, &ices) {
        Ok(_) => term.write_line("New ICE mail created"),
        Err(e) => term.write_line(format!("Error: {}" ,e.description()).as_str())
    };
}

/// Show a list of ICE mails and select one to edit
///
/// # Arguments
///
/// * `term` - Terminal abstraction
/// * `conf` - Application configuration
pub fn edit_ice(term: &Term, conf: &Ini) {
    let mut ices = match parser::get_ices(&conf) {
        Ok(v) => v,
        Err(e) => {
            term.write_line(format!("Error: {}", e).as_str());
            return;
        }
    };

    if ices.is_empty() {
        term.write_line("No ICE mails to show");
        return;
    }

    // Select an ICE to edit
    let mut selection = Select::new();
    for ice in &ices {
        selection.item(ice.get_description().as_str());
    }

    let selected = selection.default(0).interact().unwrap();
    let mut edited = ices[selected].clone();

    term.write_line(format!("Editing '{}'\n", edited.get_description()).as_str());

    // Description
    if Confirmation::new("Do you want to edit the short description?").interact().unwrap() {
        let new_description = Input::new("Please specify a short description")
            .default(edited.get_description().as_str())
            .interact().unwrap();

        edited.set_description(new_description);
    }

    // Message
    if Confirmation::new("Do you want to edit the message?").interact().unwrap() {
        let new_message = Editor::new().edit(edited.get_message().as_str()).unwrap();

        if new_message.is_none() {
            term.write_line("No message provided, using the original one");
        } else {
            edited.set_message(new_message.unwrap());
        }
    }

    // Addresses
    if Confirmation::new("Do you want to edit the recipients?").interact().unwrap() {
        let new_emails = Input::new("Please specify recipients (comma-separated)")
            .default(edited.get_emails().join(",").as_str())
            .interact().unwrap();

        let mut email_list = Vec::new();
        for email in new_emails.split(",") {
            email_list.push(email.trim().to_string());
        }

        edited.set_emails(&email_list);
    }

    // Save edited ICE
    ices[selected] = edited;
    match parser::write_ices(&conf, &ices) {
        Ok(_) => term.write_line("ICE mail updated"),
        Err(e) => term.write_line(format!("Error: {}" ,e.description()).as_str())
    };
}

/// List ICE mails present in the JSON file
///
/// The output also shows whether an ICE is enabled and the date when it is
/// scheduled to be sent.
///
/// # Arguments
///
/// * `term` - Terminal abstraction
/// * `conf` - Application configuration
pub fn list_ices(term: &Term, conf: &Ini) {
    let ices = match parser::get_ices(&conf) {
        Ok(v) => v,
        Err(e) => {
            term.write_line(format!("Error: {}", e).as_str());
            return;
        }
    };

    if ices.is_empty() {
        term.write_line("No ICE mails to show");
        return;
    }

    for ice in ices {
        term.write_line(ice.get_status_line().as_str());
    }
}

/// Show a list of ICE mails and select one to remove
///
/// # Arguments
///
/// * `term` - Terminal abstraction
/// * `conf` - Application configuration
pub fn remove_ice(term: &Term, conf: &Ini) {
    let mut ices = match parser::get_ices(&conf) {
        Ok(v) => v,
        Err(e) => {
            term.write_line(format!("Error: {}", e).as_str());
            return;
        }
    };

    if ices.is_empty() {
        term.write_line("No ICE mails to show");
        return;
    }

    // Select an ICE to remove
    let mut selection = Select::new();
    for ice in &ices {
        selection.item(ice.get_description().as_str());
    }

    let selected = selection.default(0).interact().unwrap();

    // Ask for confirmation
    if !Confirmation::new(format!(
        "Do you want to remove '{}'?", ices[selected].get_description()
    ).as_str()).interact().unwrap() {
        term.write_line("Operation cancelled");
        return;
    }

    // Remove ICE
    let removed = ices.remove(selected);
    match parser::write_ices(&conf, &ices) {
        Ok(_) => {
            term.write_line(
                format!("ICE mail '{}' removed", removed.get_description())
                .as_str()
            )
        },
        Err(e) => term.write_line(format!("Error: {}" ,e.description()).as_str())
    };
}
