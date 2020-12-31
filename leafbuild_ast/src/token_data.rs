//! Data about particular tokens, like the `NumVal` and `ParsedString` structures
use std::num::ParseIntError;
use std::str::FromStr;

/// A number value
#[derive(Copy, Clone, Debug, PartialOrd, Eq, PartialEq)]
pub enum NumVal {
    /// i32 number
    I32(i32),
    /// i64 number
    I64(i64),
    /// u32 number
    U32(u32),
    /// u64 number
    U64(u64),
}

#[derive(Copy, Clone)]
enum Tp {
    I32,
    I64,
    U32,
    U64,
}
impl Tp {
    const fn into_unsigned(self) -> Self {
        match self {
            Self::I32 => Self::U32,
            Self::I64 => Self::U64,
            x => x,
        }
    }

    const fn into_long(self) -> Self {
        match self {
            Self::I32 => Self::I64,
            Self::U32 => Self::U64,
            x => x,
        }
    }

    const fn zero(self) -> NumVal {
        match self {
            Self::I32 => NumVal::I32(0),
            Self::I64 => NumVal::I64(0),
            Self::U32 => NumVal::U32(0),
            Self::U64 => NumVal::U64(0),
        }
    }

    fn create_from_str(self, s: &str, radix: u32) -> Result<NumVal, ParseIntError> {
        match self {
            Self::I32 => i32::from_str_radix(s, radix).map(NumVal::I32),
            Self::U32 => u32::from_str_radix(s, radix).map(NumVal::U32),
            Self::I64 => i64::from_str_radix(s, radix).map(NumVal::I64),
            Self::U64 => u64::from_str_radix(s, radix).map(NumVal::U64),
        }
    }
}

impl FromStr for NumVal {
    type Err = ParseIntError;
    /// parse a number from a number literal string
    fn from_str(s: &str) -> Result<Self, ParseIntError> {
        let tp = s
            .chars()
            .rev()
            .take_while(|chr| Self::is_suffix(*chr))
            .fold(Tp::I32, |tp, chr| match chr {
                'u' | 'U' => tp.into_unsigned(),
                'l' | 'L' => tp.into_long(),
                _ => tp,
            });
        if s.starts_with("0x") {
            Self::parse_hex(s, tp)
        } else if s.starts_with("0b") {
            Self::parse_bin(s, tp)
        } else if s.starts_with('0') {
            Self::parse_oct(s, tp)
        } else {
            Self::parse_dec(s, tp)
        }
    }
}

impl NumVal {
    fn parse_hex(s: &str, tp: Tp) -> Result<Self, ParseIntError> {
        // s = "0x.."
        tp.create_from_str(
            s.trim_start_matches("0x").trim_end_matches(Self::is_suffix),
            16,
        )
    }

    fn parse_bin(s: &str, tp: Tp) -> Result<Self, ParseIntError> {
        // s = "0b..."
        tp.create_from_str(
            s.trim_start_matches("0b").trim_end_matches(Self::is_suffix),
            2,
        )
    }

    fn parse_oct(s: &str, tp: Tp) -> Result<Self, ParseIntError> {
        // s = "0..."
        let s = s.trim_start_matches('0').trim_end_matches(Self::is_suffix);
        if s.is_empty() {
            return Ok(tp.zero());
        }

        tp.create_from_str(s, 8)
    }

    fn parse_dec(s: &str, tp: Tp) -> Result<Self, ParseIntError> {
        // s = "..."
        tp.create_from_str(s.trim_end_matches(Self::is_suffix), 10)
    }

    const fn is_suffix(chr: char) -> bool {
        matches!(chr, 'u' | 'U' | 'l' | 'L')
    }
}

/// A parsed string, with escapes processed.
#[derive(PartialOrd, PartialEq, Ord, Eq, Debug, Clone)]
pub struct ParsedString(String);

/// Parses a single line string into a parsed string by applying escapes.
///
/// # Errors
/// On invalid escapes
pub fn parse_single_line_string(inp: &str) -> Result<ParsedString, <ParsedString as FromStr>::Err> {
    inp[1..inp.len() - 1].parse()
}

/// Creates a multiline string from the given string
/// Unlike [`parse_single_line_string`], this doesn't apply escapes
///
/// # Errors
/// Never
pub fn parse_multi_line_string(inp: &str) -> Result<ParsedString, <ParsedString as FromStr>::Err> {
    Ok(ParsedString(inp[3..inp.len() - 3].into()))
}

impl FromStr for ParsedString {
    type Err = snailquote::UnescapeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        snailquote::unescape(s).map(Self)
    }
}
