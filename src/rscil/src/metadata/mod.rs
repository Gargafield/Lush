mod headers;
mod streams;
mod image;
mod parser;
mod kind;
mod rows;
mod index;
mod flags;
mod cil;
mod decode;

use std::io::{Cursor, Read, Seek, SeekFrom};
use byteorder::{LittleEndian, ReadBytesExt};

pub use kind::TableKind;
pub use headers::*;
pub use streams::Streams;
pub use image::PeImage;
pub use parser::PeParser;
pub use rows::*;
pub use index::*;
pub use flags::*;
pub use cil::*;
pub use decode::*;

pub type Table = Vec<Row>;
