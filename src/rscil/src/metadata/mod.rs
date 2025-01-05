mod headers;
mod streams;
mod image;
mod parser;
mod kind;
mod rows;
mod index;
mod flags;
mod cil;
mod decode_context;

use std::{fs::File, io::{Cursor, Read, Seek, SeekFrom}};
use byteorder::{LittleEndian, ReadBytesExt};

pub use kind::TableKind;
pub use headers::*;
pub use streams::{Streams, HeapSizes};
pub use image::PeImage;
pub use parser::PeParser;
pub use rows::*;
pub use index::*;
pub use flags::*;
pub use cil::*;
pub(crate) use decode_context::TableDecodeContext;

pub(crate) type Buffer = Cursor<Vec<u8>>;
pub type Table = Vec<Row>;
