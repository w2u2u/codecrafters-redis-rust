use std::io::{BufRead, Cursor, Read};

use anyhow::Error;

// read -> buffer -> Frame -> Command -> Frame -> bytes -> write

#[derive(Debug, PartialEq, Eq)]
pub enum Frame {
    SimpleString(String),
    BulkString(String),
    Arrays(Vec<String>),
    Null,
    Unknown,
}

impl Frame {
    pub fn parse(buf: &[u8]) -> Result<Self, Error> {
        let mut cursor = Cursor::new(buf);

        match read_byte(&mut cursor) {
            Ok(b'+') => {
                let line = read_line(&mut cursor)?;

                Ok(Frame::SimpleString(line))
            }
            Ok(b'$') => {
                let bulk_string = read_bulk_string(&mut cursor)?;

                Ok(Frame::BulkString(bulk_string))
            }
            Ok(b'*') => {
                let count = read_line(&mut cursor)?.parse::<usize>().unwrap_or(0);

                let mut array = Vec::with_capacity(count);

                for _ in 0..count {
                    if let Ok(b'$') = read_byte(&mut cursor) {
                        let bulk_string = read_bulk_string(&mut cursor)?;
                        let _ = read_line(&mut cursor)?; // consume \r\n

                        array.push(bulk_string);
                    }
                }

                Ok(Frame::Arrays(array))
            }
            _ => Ok(Frame::Unknown),
        }
    }

    pub fn to_vec(&self) -> Vec<String> {
        let mut result = Vec::new();

        match self {
            Frame::SimpleString(s) => {
                result.push(s.clone());
            }
            Frame::BulkString(s) => {
                result.push(s.clone());
            }
            Frame::Arrays(a) => result = a.to_vec(),
            _ => {}
        }

        result
    }
}

fn read_bulk_string(cursor: &mut Cursor<&[u8]>) -> Result<String, Error> {
    let length = read_line(cursor)?.parse::<usize>().unwrap_or(0);

    if length == 0 {
        Ok(String::new())
    } else {
        let bulk_string = read_n_bytes(cursor, length)?;

        Ok(bulk_string)
    }
}

fn read_byte(cursor: &mut Cursor<&[u8]>) -> Result<u8, Error> {
    let mut buf = [0; 1];

    cursor.read_exact(&mut buf)?;

    Ok(buf[0])
}

fn read_n_bytes(cursor: &mut Cursor<&[u8]>, n: usize) -> Result<String, Error> {
    let mut buf = vec![0; n];

    cursor.read_exact(&mut buf)?;

    Ok(String::from_utf8_lossy(&buf).to_string())
}

fn read_line(cursor: &mut Cursor<&[u8]>) -> Result<String, Error> {
    let mut line = String::new();

    cursor.read_line(&mut line)?;

    Ok(line.trim_end_matches("\r\n").to_string())
}

#[cfg(test)]
mod test {
    use anyhow::Error;

    use super::Frame;

    #[test]
    fn test_parse_simple_string() -> Result<(), Error> {
        let raw_bulk = b"+PING\r\n";

        let frame = Frame::parse(raw_bulk)?;

        assert_eq!(Frame::SimpleString(String::from("PING")), frame);

        Ok(())
    }

    #[test]
    fn test_parse_bulk_string() -> Result<(), Error> {
        let raw_bulk = b"$4\r\nPING\r\n";

        let frame = Frame::parse(raw_bulk)?;

        assert_eq!(Frame::BulkString(String::from("PING")), frame);

        Ok(())
    }

    #[test]
    fn test_parse_arrays() -> Result<(), Error> {
        let raw_bulk = b"*2\r\n$3\r\nabc\r\n$3\r\nxyz\r\n";

        let frame = Frame::parse(raw_bulk)?;

        let expected = Frame::Arrays(vec![String::from("abc"), String::from("xyz")]);

        assert_eq!(expected, frame);

        Ok(())
    }
}
