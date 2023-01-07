use std::{fs, error::Error};
use std::io::Read;
use clap::Parser;
use bytebuffer::{ByteBuffer, Endian};
use log::{info, trace, warn, error};
use stderrlog::LogLevelNum;
use substring::Substring;

use crate::nk2_definitions::{Nk2Property, Nk2PropertyData, Nk2Row, PR_DISPLAY_NAME_W, PR_NICK_NAME_W, PR_EMAIL_ADDRESS_W, PR_SMTP_ADDRESS_W, PR_NICK_NAME_WEIGHT};
use crate::contact::Contact;
use crate::contact_writer::ContactWriter;

extern crate num;
#[macro_use]
extern crate num_derive;

pub mod errors;
pub mod nk2_definitions;
pub mod nk2_data_parser;
pub mod contact;
pub mod contact_writer;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to outlook autocomplete nk2/dat file
    #[arg(short, long)]
    file: String,
    #[arg(short, long)]
    output_file: String,
    #[arg(long, value_enum, default_value_t = ContactWriter::Csv)]
    output_format: ContactWriter,
    #[arg(short, long, action)]
    verbose: bool
}

fn read_property(buffer: &mut ByteBuffer) -> Result<Nk2Property, Box<dyn Error>> {
    let property_type = buffer.read_u16()?;
    let parsed_property_type = crate::nk2_definitions::parse_property_type(property_type)?;

    let property_tag: u32 = ((buffer.read_u16()? as u32) << 16) | (property_type as u32);
    let reserved_data = buffer.read_u32()?;

    let mut value_union = [0 as u8; 8];
    buffer.read_exact(&mut value_union)?;

    let mut property = Nk2Property {
        property_type: parsed_property_type,
        property_tag: property_tag,
        reserved_data: reserved_data,
        value_union: value_union,
        value: Nk2PropertyData::Empty
    };

    trace!("    Property Type: {:?}", parsed_property_type);
    trace!("    Property Tag: 0x{:02x}", property_tag);

    property.value = parsed_property_type.parse_data(&property, buffer)?;
    trace!("    Value: {}", property.value);

    Ok(property)
}

fn read_row(buffer: &mut ByteBuffer) -> Result<Nk2Row, Box<dyn Error>> {
    let mut row = Nk2Row {
        properties: Vec::new()
    };

    let properties_count = buffer.read_u32()?;

    for property_idx in 0..properties_count {
        trace!("  Property: {}", property_idx);

        let property = read_property(buffer)?;
        row.properties.push(property);
    }

    Ok(row)
}

fn read_all_rows(buffer: &mut ByteBuffer) -> Result<Vec<Nk2Row>, Box<dyn Error>> {
    let mut rows = Vec::new();

    let rows_count = buffer.read_u32()?;
    for _row_idx in 0..rows_count {
        let row = read_row(buffer)?;
        rows.push(row);
    }

    Ok(rows)
}

fn read_file(filepath: &str) -> Result<Option<Vec<Nk2Row>>, Box<dyn Error>> {
    let data = fs::read(filepath)?;

    let mut buffer = ByteBuffer::from_vec(data);
    buffer.set_endian(Endian::LittleEndian);

    buffer.read_bytes(12)?;  // Skip metadata

    match read_all_rows(&mut buffer) {
        Ok(result) => {
            info!("Successfully read {} rows from nk2 autocomplete file.", result.len());
            return Ok(Some(result));
        },
        Err(error) => {
            error!("Error while reading offset {}: {}", buffer.get_rpos(), error);
            return Ok(None);
        }
    }
}

fn fix_name(name: &str) -> String {
    let mut fixed_name = name.to_string();
    if fixed_name.starts_with("'") {
        fixed_name = fixed_name.substring(1, fixed_name.len()).to_string();
    }
    if fixed_name.ends_with("'") {
        fixed_name = fixed_name.substring(0, fixed_name.len() - 1).to_string();
    }

    return fixed_name;
}

fn parse_contact(row: &Nk2Row) -> Result<Contact, Box<dyn Error>> {
    let name_property = row.find_property_by_tag(PR_DISPLAY_NAME_W).ok_or(Box::new(errors::MissingPropertyTagError(PR_DISPLAY_NAME_W)))?;
    let recipient_property = row.find_property_by_tag(PR_EMAIL_ADDRESS_W).ok_or(Box::new(errors::MissingPropertyTagError(PR_EMAIL_ADDRESS_W)))?;

    let email_property = row.find_property_by_tag(PR_SMTP_ADDRESS_W).unwrap_or(recipient_property);

    let weight: Option<i32>;
    let weight_property = row.find_property_by_tag(PR_NICK_NAME_WEIGHT);
    if weight_property.is_some() {
        weight = Some(weight_property.unwrap().decode_value_as_long()?);
    } else {
        weight = None;
    }

    let contact = Contact {
        name: fix_name(&name_property.value.to_string()),
        email: email_property.value.to_string(),
        server_email: recipient_property.value.to_string(),
        weight: weight
    };
    Ok(contact)
}

fn parse_contacts(rows: &Vec<Nk2Row>) -> Result<Vec<Contact>, Box<dyn Error>> {
    let mut contacts = Vec::new();
    for (idx, row) in rows.iter().enumerate() {
        match parse_contact(&row) {
            Ok(contact) => contacts.push(contact),
            Err(error) => {
                let id_property = row.find_property_by_tag(PR_NICK_NAME_W);
                let name = id_property.map_or("?".to_string(), |x| x.value.to_string());

                warn!("Failed to parse contact {} ({}): {}", idx, name, error);
            }
        }
    }
    
    Ok(contacts)
}

fn get_min_log_level(verbose: bool) -> LogLevelNum {
    if verbose {
        return LogLevelNum::Trace;
    } else {
        return LogLevelNum::Info;
    }
}

fn main() {
    let args = Args::parse();

    stderrlog::new()
        .verbosity(get_min_log_level(args.verbose))
        .show_level(false)
        .module(module_path!())
        .init().unwrap();
    
    info!("Reading and parsing file {} ...", args.file);

    let rows_result = read_file(&args.file).unwrap();
    if rows_result.is_none() {
        return;
    }

    let rows = rows_result.unwrap();
    let contacts = parse_contacts(&rows).unwrap();

    for contact in &contacts {
        trace!("Contact | {} | {} | {}", contact.name, contact.email, contact.weight.map_or("?".to_string(), |x| x.to_string()));
    }

    info!("Write output file ...");
    args.output_format.write_to_file(&args.output_file, &contacts).unwrap();
    info!("Successfully created {}", args.output_file);
}
