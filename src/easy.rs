use std::{
    fmt::Debug,
    io::{Read, Write},
};

use crate::compress::Compressor;

const VERSION: u8 = 1;
const ARCHIVE_TYPE_FILE: u8 = 0;
const ARCHIVE_TYPE_FOLDER: u8 = 1;

#[derive(Debug, Copy, Clone)]
pub struct ArchiveHeader {
    pub version: u8,
    pub archive_type: u8,
}

impl ArchiveHeader {
    pub fn new(version: u8, archive_type: u8) -> Self {
        ArchiveHeader {
            version,
            archive_type,
        }
    }

    pub fn from_bytes<R: Read>(mut b: R) -> (Self, R) {
        let first_byte: u8 = {
            let mut buf = [0u8; 1];
            b.read_exact(&mut buf).unwrap();
            buf[0]
        };

        (
            ArchiveHeader {
                version: first_byte >> 2,
                archive_type: first_byte & 0b11,
            },
            b,
        )
    }

    pub fn bytes(&self) -> Vec<u8> {
        let mut header_bytes = Vec::with_capacity(1);
        header_bytes.push((self.version << 2) | self.archive_type);
        header_bytes
    }
}

#[derive(Debug)]
pub struct Easy {
    pub header: Option<ArchiveHeader>,
    compressor: Compressor,
}

impl Easy {
    pub fn new(compressor: Compressor) -> Self {
        Easy {
            header: None,
            compressor,
        }
    }

    pub fn compress<R: Read, W: Write>(&mut self, b: R, mut w: W) {
        let h = &self
            .header
            .unwrap_or(ArchiveHeader::new(VERSION, ARCHIVE_TYPE_FILE));
        w.write_all(&h.bytes()).unwrap();

        self.compressor.compress(b, &mut w);
    }

    pub fn decompress<R: Read + Debug, W: Write>(&mut self, b: R, mut w: W) {
        let (h, left) = ArchiveHeader::from_bytes(b);
        self.header = Some(h);

        self.compressor.decompress(left, &mut w);
    }
}

#[cfg(test)]
mod archive_header_tests {
    use std::io::Cursor;

    use crate::easy::ArchiveHeader;

    #[test]
    fn test_from_bytes() {
        let b = [0b00001001, 0b10101010];
        let b = Cursor::new(b);
        let (ah, left) = ArchiveHeader::from_bytes(b);
        assert_eq!(ah.version, 0b10);
        assert_eq!(ah.archive_type, 0b1);
        assert_eq!(left.position(), 1);
    }
}
