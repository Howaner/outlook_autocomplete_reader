use std::error::Error;
use std::fmt;
use crate::errors;

/// This property must be first in each recipient row. Functionally serves as a key identifier for the recipient row.
pub const PR_NICK_NAME_W: u32 = 0x6001001F;
/// The address book entry identifier for the recipient.
pub const PR_ENTRYID: u32 = 0x0FFF0102;
/// The recipient’s display name.
pub const PR_DISPLAY_NAME_W: u32 = 0x3001001F;
/// PR_EMAIL_ADDRESS_W
pub const PR_EMAIL_ADDRESS_W: u32 = 0x3003001F;
/// The recipient’s address type (e.g. SMTP or EX).
pub const PR_ADDRTYPE_W: u32 = 0x3002001F;
/// The recipient’s SMTP address.
pub const PR_SMTP_ADDRESS_W: u32 = 0x39FE001F;
/// The display string that shows up in the autocomplete list.
pub const PR_DROPDOWN_DISPLAY_NAME_W: u32 = 0x6003001F;
/// The weight of this autocomplete entry. The weight is used to determine in what order autocomplete entries show up when matching the autocomplete list.
pub const PR_NICK_NAME_WEIGHT: u32 = 0x60040003;


pub struct Nk2Property {
    pub property_type: Nk2PropertyType,
    pub property_tag: u32,
    pub reserved_data: u32,
    pub value_union: [u8;8],
    pub value: Nk2PropertyData
}

pub struct Nk2Row {
    pub properties: Vec<Nk2Property>
}

impl Nk2Row {
    pub fn find_property_by_tag(&self, tag: u32) -> Option<&Nk2Property> {
        self.properties.iter().filter(|device| device.property_tag == tag).next()
    }
}

const MV_FLAG: u16 = 0x1000;

#[derive(Copy, Clone, FromPrimitive, Debug)]
#[repr(u16)]
pub enum Nk2PropertyType {
	PtUnspecified = 0x0000,
    PtNull        = 0x0001,
    PtI2          = 0x0002,
    PtI4          = 0x0003,
    PtFloat       = 0x0004,
    PtDouble      = 0x0005,
    PtCurrency    = 0x0006,
    PtAppTime     = 0x0007,
    PtError       = 0x000A,
    PtBoolean     = 0x000B,
    PtObject      = 0x000D,
    PtI8          = 0x0014,
    PtString8     = 0x001E,
    PtUnicode     = 0x001F,  // Same as PT_TSTRING
    PtSysTime     = 0x0040,
    PtClsid       = 0x0048,
    PtSvreid      = 0x00FB,
    PtSRestrict   = 0x00FD,
    PtActions     = 0x00FE,
    PtBinary      = 0x0102,
	PtMvBinary    = MV_FLAG | 0x0102,
	PtMvString8   = MV_FLAG | 0x001E,
	PtMvUnicode   = MV_FLAG | 0x001F
}

pub enum Nk2PropertyData {
    Empty,
    Text(String),
    Bytes(Vec<u8>),
    BytesList(Vec<Vec<u8>>),
    TextList(Vec<String>)
}

impl fmt::Display for Nk2PropertyData {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Nk2PropertyData::Empty => write!(f, "empty"),
			Nk2PropertyData::Text(text) => write!(f, "{}", text),
			Nk2PropertyData::Bytes(bytes) => write!(f, "{:?}", bytes),
			Nk2PropertyData::BytesList(list) => write!(f, "BList: {:?}", list),
			Nk2PropertyData::TextList(list) => write!(f, "TList: {:?}", list)
		}
	}
}

pub fn parse_property_type(property_type: u16) -> Result<Nk2PropertyType, Box<dyn Error>> {
    match num::FromPrimitive::from_u16(property_type) {
        Some(parsed_type) => Ok(parsed_type),
        None => Err(Box::new(errors::InvalidPropertyTypeError(property_type)))
    }
}
