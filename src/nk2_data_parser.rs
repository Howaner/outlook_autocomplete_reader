use std::error::Error;
use bytebuffer::{ByteBuffer, Endian};
use crate::nk2_definitions::{Nk2Property, Nk2PropertyType, Nk2PropertyData};
use crate::errors::TooMuchDataError;

const MAX_STRING_LENGTH: usize = 10_000;
const MAX_BYTE_ARRAY_LENGTH: usize = 2_000_000;
const MAX_ARRAY_ELEMENTS: usize = 500_000;

fn check_max_array_len(name: &str, len: usize, max_len: usize) -> Result<(), Box<dyn Error>> {
	if len > max_len {
		return Err(Box::new(TooMuchDataError(name.to_string(), len, max_len)));
	} else {
		return Ok(());
	}
}

fn parse_ansi_string(buffer: &mut ByteBuffer) -> Result<String, Box<dyn Error>> {
	let bytes_count = buffer.read_u32()? as usize;
	check_max_array_len("ANSI String", bytes_count, MAX_STRING_LENGTH)?;
	let mut bytes = buffer.read_bytes(bytes_count)?;

	if let Some(last_byte) = bytes.last() {
		if last_byte == &('\0' as u8) {
			bytes.remove(bytes.len() - 1);
		}
	}

	let str = encoding_rs::WINDOWS_1252.decode(&bytes);
	Ok(str.0.into_owned())
}

fn parse_unicode_string(buffer: &mut ByteBuffer) -> Result<String, Box<dyn Error>> {
	let bytes_count = buffer.read_u32()? as usize;
	check_max_array_len("Unicode String", bytes_count, MAX_STRING_LENGTH)?;
	let mut bytes = buffer.read_bytes(bytes_count)?;

	if let Some(last_byte) = bytes.last() {
		if last_byte == &('\0' as u8) {
			bytes.remove(bytes.len() - 1);
			bytes.remove(bytes.len() - 1);
		}
	}

	let str = encoding_rs::UTF_16LE.decode(&bytes);
	Ok(str.0.into_owned())
}

fn parse_binary(buffer: &mut ByteBuffer) -> Result<Vec<u8>, Box<dyn Error>> {
	let bytes_count = buffer.read_u32()? as usize;
	check_max_array_len("Byte array", bytes_count, MAX_BYTE_ARRAY_LENGTH)?;
	Ok(buffer.read_bytes(bytes_count)?)
}

fn parse_binary_arrays(buffer: &mut ByteBuffer) -> Result<Vec<Vec<u8>>, Box<dyn Error>> {
	let mut arrays: Vec<Vec<u8>> = Vec::new();

	let arrays_count = buffer.read_u32()? as usize;
	check_max_array_len("Byte arrays", arrays_count, MAX_ARRAY_ELEMENTS)?;

	for _ in 0..arrays_count {
		let data = parse_binary(buffer)?;
		arrays.push(data);
	}

	Ok(arrays)
}

fn parse_ansi_string_arrays(buffer: &mut ByteBuffer) -> Result<Vec<String>, Box<dyn Error>> {
	let mut strings: Vec<String> = Vec::new();

	let count = buffer.read_u32()? as usize;
	check_max_array_len("ANSI String arrays", count, MAX_ARRAY_ELEMENTS)?;

	for _ in 0..count {
		strings.push(parse_ansi_string(buffer)?);
	}

	Ok(strings)
}

fn parse_unicode_string_arrays(buffer: &mut ByteBuffer) -> Result<Vec<String>, Box<dyn Error>> {
	let mut strings: Vec<String> = Vec::new();

	let count = buffer.read_u32()? as usize;
	check_max_array_len("Unicode String arrays", count, MAX_ARRAY_ELEMENTS)?;

	for _ in 0..count {
		strings.push(parse_unicode_string(buffer)?);
	}

	Ok(strings)
}

impl Nk2PropertyType {
    pub fn parse_data(&self, _property: &Nk2Property, buffer: &mut ByteBuffer) -> Result<Nk2PropertyData, Box<dyn Error>> {
        Ok(match self {
            Nk2PropertyType::PtString8 => Nk2PropertyData::Text(parse_ansi_string(buffer)?),
			Nk2PropertyType::PtUnicode => Nk2PropertyData::Text(parse_unicode_string(buffer)?),
			Nk2PropertyType::PtClsid => Nk2PropertyData::Bytes(buffer.read_bytes(16)?),
			Nk2PropertyType::PtBinary => Nk2PropertyData::Bytes(parse_binary(buffer)?),
			Nk2PropertyType::PtMvBinary => Nk2PropertyData::BytesList(parse_binary_arrays(buffer)?),
			Nk2PropertyType::PtMvString8 => Nk2PropertyData::TextList(parse_ansi_string_arrays(buffer)?),
			Nk2PropertyType::PtMvUnicode => Nk2PropertyData::TextList(parse_unicode_string_arrays(buffer)?),
			_ => Nk2PropertyData::Empty
        })
    }
}

impl Nk2Property {
    pub fn decode_value_as_long(self: &Nk2Property) -> Result<i32, Box<dyn Error>> {
		let mut buffer = ByteBuffer::from_bytes(&self.value_union);
    	buffer.set_endian(Endian::LittleEndian);

		Ok(buffer.read_i32()?)
	}
}
