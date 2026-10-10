use std::collections::BTreeMap;

const INTEGER_START: u8 = b'i';
const LIST_START: u8 = b'l';
const DICTIONARY_START: u8 = b'd';
const BYTES_START: std::ops::RangeInclusive<u8> = b'0'..=b'9';
const VALUE_END: u8 = b'e';

enum BencodeValue {
    Integer(i64),
    StringBytes(Vec<u8>),
    List(Vec<BencodeValue>),
    Dictionary(BTreeMap<Vec<u8>, BencodeValue>),
}

#[derive(Debug)]
enum ParseError {
    EmptyInput,
    UnexpectedEnd { position: usize },
    InvalidInput(String),
}

impl BencodeValue {
    fn parse_at(input: &[u8], start: usize) -> Result<(BencodeValue, usize), ParseError> {
        match input.get(start) {
            Some(INTEGER_START) => Self::parse_int(input, start),
            Some(LIST_START) => Self::parse_list(input, start),
            Some(DICTIONARY_START) => Self::parse_dictionary(input, start),
            Some(byte) if BYTES_START.contains(byte) => Self::parse_string(input, start),
            None => Err(ParseError::UnexpectedEnd { position: start }),
            Some(byte) => Err(ParseError::InvalidInput(format!(
                "invalid bencode prefix byte {byte} at position {start}"
            ))),
        }
    }

    fn parse_int(input: &[u8], start: usize) -> Result<(BencodeValue, usize), ParseError> {
        let digits_start = start + 1;
        let plausible_end = input[digits_start..]
            .iter()
            .position(|&byte| byte == VALUE_END)
            .ok_or(ParseError::InvalidInput("integer"));

        let end = digits_start + plausible_end;
        let digits = &input[digits_start..end];

        let text = std::str::from_utf8(digits).map_err(|_| ParseError::InvalidInput("integer"))?;
        let value = text
            .parse::<i64>()
            .map_err(|_| ParseError::InvalidInput("integer"))?;
        if text == "-0"
            || (text.starts_with('0') && text.len() > 1)
            || (text.starts_with("-0") && text.len() > 2)
        {
            return Err(ParseError::InvalidInteger);
        }

        Ok((BencodeValue::Integer(value), end + 1))
    }

    fn parse_list(input: &[u8], start: usize) -> Result<(BencodeValue, usize), ParseError> {
        let mut values = Vec::new();
        let mut position = start + 1; // Skip the `l` prefix.

        loop {
            match input.get(position) {
                Some(&VALUE_END) => {
                    return Ok((BencodeValue::List(values), position + 1));
                }
                Some(_) => {
                    let (value, next_position) = Self::parse_at(input, position)?;
                    values.push(value);
                    position = next_position;
                }
                None => return Err(ParseError::UnexpectedEnd { position }),
            }
        }
    }

    fn parse_dictionary(input: &[u8], _start: usize) -> Result<(BencodeValue, usize), ParseError> {
        todo!()
    }

    fn parse_string(input: &[u8], _start: usize) -> Result<(BencodeValue, usize), ParseError> {
        todo!()
    }
}

pub fn parse(input: &[u8]) -> Result<(BencodeValue, usize), ParseError> {
    BencodeValue::parse_at(input, 0)
}
