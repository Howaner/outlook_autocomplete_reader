use std::collections::HashSet;
use std::fs::File;
use std::io::{LineWriter, Write};
use std::error::Error;

use clap::ValueEnum;
use log::warn;
use vcard::{Set, VCard};
use vcard::values::email_value::EmailValue;
use vcard::properties::*;

use crate::contact::Contact;

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum, Debug)]
pub enum ContactWriter {
	Vcard,
	Csv
}

impl ContactWriter {
	pub fn write_to_file(&self, file_path: &str, contacts: &Vec<Contact>) -> Result<(), Box<dyn Error>> {
		match self {
			ContactWriter::Vcard => write_vcard(file_path, contacts),
			ContactWriter::Csv => write_csv(file_path, contacts)
		}
	}
}

fn write_single_vcard_contact(contact: &Contact) -> Result<VCard, Box<dyn Error>> {
	let mut vcard = VCard::from_formatted_name_str(&contact.name)?;
		
	let email = Email::from_email_value(EmailValue::from_str(&contact.email)?);
	
	let mut emails = HashSet::new();
	emails.insert(email);
	vcard.emails = Some(Set::from_hash_set(emails)?);

	Ok(vcard)
}

fn write_vcard(file_path: &str, contacts: &Vec<Contact>) -> Result<(), Box<dyn Error>> {
	let file = File::create(file_path)?;
	let mut line_writer = LineWriter::new(file);

	for contact in contacts {
		match write_single_vcard_contact(contact) {
			Ok(contact) => {
				line_writer.write_all(contact.to_string().as_bytes())?;
			},
			Err(err) => {
				warn!("Failed to create vcard for contact {}: {}", contact.name, err);
			}
		}
	}

	line_writer.flush()?;
	Ok(())
}

fn write_csv(file_path: &str, contacts: &Vec<Contact>) -> Result<(), Box<dyn Error>> {
	let mut writer = csv::WriterBuilder::new()
		.double_quote(true)
		.terminator(csv::Terminator::CRLF)
		.from_path(file_path)?;
	writer.write_record(&["name", "email", "server_email", "weight"])?;

	for contact in contacts {
		let weight = contact.weight.map_or("?".to_string(), |x| x.to_string());
		writer.write_record(&[&contact.name, &contact.email, &contact.server_email, &weight])?;
	}

	writer.flush()?;
	Ok(())
}
