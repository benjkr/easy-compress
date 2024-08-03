use bytesize::*;
use std::io::{BufWriter, Read, Write};

const MAX_READ_BUFFER_SIZE: usize = 1 * MIB as usize;
const MAX_WRITE_BUFFER_SIZE: usize = 1 * MIB as usize;

#[derive(Debug)]
pub struct Compressor {}

impl Compressor {
    pub fn compress<R: Read, W: Write>(&self, mut buff: R, w: &mut W) {
        let mut tmp_buf = [0u8; MAX_READ_BUFFER_SIZE];
        let mut tmp_write_buf = BufWriter::with_capacity(MAX_WRITE_BUFFER_SIZE, w);

        let mut first_byte_buffer = [0; 1];
        buff.read_exact(&mut first_byte_buffer).unwrap();

        let mut last_byte = first_byte_buffer[0];

        let mut n: u8 = 1;
        let mut current_byte: u8;

        while let Ok(bytes_read) = buff.read(&mut tmp_buf) {
            if bytes_read == 0 {
                break;
            }

            for i in tmp_buf[..bytes_read].iter() {
                current_byte = *i;
                if current_byte == last_byte {
                    n += 1;
                } else {
                    tmp_write_buf.write(&[n, last_byte]).unwrap();
                    last_byte = current_byte;
                    n = 1;
                }
                if n == u8::MAX {
                    println!("Got a byte with value {}", n);
                    tmp_write_buf.write(&[n, last_byte]).unwrap();
                    n = 0;
                }
            }
        }
        if n > 0 {
            tmp_write_buf.write(&[n, last_byte]).unwrap();
        }

        tmp_write_buf.flush().unwrap();
    }

    pub fn decompress<R: Read, W: Write>(&self, mut buff: R, w: &mut W) {
        let mut tmp_read_buf = [0u8; MAX_READ_BUFFER_SIZE];
        let mut tmp_write_buf = BufWriter::with_capacity(MAX_WRITE_BUFFER_SIZE, w);

        // TODO: Check if this is possible. If so, remove the commented out code
        // let mut left_over: Option<u8> = None;

        while let Ok(bytes_read) = buff.read(&mut tmp_read_buf) {
            if bytes_read == 0 {
                break;
            }

            // TODO: Check if this is possible. If so, remove the commented out code
            // if let Some(left_over) = left_over {
            //     tmp_read_buf.push(left_over);
            //     tmp_read_buf.rotate_right(1);
            // }

            // left_over = if bytes_read % 2 == 0 {
            //     None
            // } else {
            //     println!("Popping left over byte");
            //     tmp_read_buf.pop()
            // };

            for c in tmp_read_buf[..bytes_read].chunks(2) {
                let n = c[0];
                let b = c[1];
                tmp_write_buf.write(&[b].repeat(n as usize)).unwrap();
            }
        }
        tmp_write_buf.flush().unwrap();
    }
}

#[cfg(test)]
mod compressor_tests {
    use super::Compressor;

    #[test]
    fn test_compressor() {
        let input: &[u8] = &[0xff, 0xff, 0xff, 0x01];
        let expected: &[u8] = &[3, 0xff, 1, 0x01];

        let mut out_buff = Vec::with_capacity(expected.len());
        Compressor {}.compress(input, &mut out_buff);
        assert_eq!(out_buff, expected);
    }

    #[test]
    fn test_compressor_over_255() {
        let input: &[u8] = &[0xffu8; 260];
        let expected: &[u8] = &[255, 0xff, 5, 0xff];

        let mut out_buff = Vec::with_capacity(expected.len());
        Compressor {}.compress(input, &mut out_buff);
        assert_eq!(out_buff, expected);
    }

    #[test]
    fn test_decompressor() {
        let input: &[u8] = &[3, 0xff, 1, 0x01];
        let expected: &[u8] = &[0xff, 0xff, 0xff, 0x01];

        let mut out_buff = Vec::with_capacity(expected.len());
        Compressor {}.decompress(input, &mut out_buff);
        assert_eq!(out_buff, expected);
    }
}
