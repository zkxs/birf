// there are 128 ascii characters (including NUL)
// the first 32 are non-printing (except TAB), so there's 31 we won't use
// the last one (DEL) is non-printing, so there's 1 we won't use
// 128 - 31 - 1 == 96
pub const FIRST_CHAR: u8 = b'\t';
pub const CHARS: [u8; 95] = [
    b' ',
    b'!',
    b'"',
    b'#',
    b'$',
    b'%',
    b'&',
    b'\'',
    b'(',
    b')',
    b'*',
    b'+',
    b',',
    b'-',
    b'.',
    b'/',
    b'0',
    b'1',
    b'2',
    b'3',
    b'4',
    b'5',
    b'6',
    b'7',
    b'8',
    b'9',
    b':',
    b';',
    b'<',
    b'=',
    b'>',
    b'?',
    b'@',
    b'A',
    b'B',
    b'C',
    b'D',
    b'E',
    b'F',
    b'G',
    b'H',
    b'I',
    b'J',
    b'K',
    b'L',
    b'M',
    b'N',
    b'O',
    b'P',
    b'Q',
    b'R',
    b'S',
    b'T',
    b'U',
    b'V',
    b'W',
    b'X',
    b'Y',
    b'Z',
    b'[',
    b'\\',
    b']',
    b'^',
    b'_',
    b'`',
    b'a',
    b'b',
    b'c',
    b'd',
    b'e',
    b'f',
    b'g',
    b'h',
    b'i',
    b'j',
    b'k',
    b'l',
    b'm',
    b'n',
    b'o',
    b'p',
    b'q',
    b'r',
    b's',
    b't',
    b'u',
    b'v',
    b'w',
    b'x',
    b'y',
    b'z',
    b'{',
    b'|',
    b'}',
    b'~',
];

pub fn increment_char(char: u8) -> Option<u8> {
    match char {
        b'\t' => Some(b' '),
        b' ' => Some(b'!'),
        b'!' => Some(b'"'),
        b'"' => Some(b'#'),
        b'#' => Some(b'$'),
        b'$' => Some(b'%'),
        b'%' => Some(b'&'),
        b'&' => Some(b'\''),
        b'\'' => Some(b'('),
        b'(' => Some(b')'),
        b')' => Some(b'*'),
        b'*' => Some(b'+'),
        b'+' => Some(b','),
        b',' => Some(b'-'),
        b'-' => Some(b'.'),
        b'.' => Some(b'/'),
        b'/' => Some(b'0'),
        b'0' => Some(b'1'),
        b'1' => Some(b'2'),
        b'2' => Some(b'3'),
        b'3' => Some(b'4'),
        b'4' => Some(b'5'),
        b'5' => Some(b'6'),
        b'6' => Some(b'7'),
        b'7' => Some(b'8'),
        b'8' => Some(b'9'),
        b'9' => Some(b':'),
        b':' => Some(b';'),
        b';' => Some(b'<'),
        b'<' => Some(b'='),
        b'=' => Some(b'>'),
        b'>' => Some(b'?'),
        b'?' => Some(b'@'),
        b'@' => Some(b'A'),
        b'A' => Some(b'B'),
        b'B' => Some(b'C'),
        b'C' => Some(b'D'),
        b'D' => Some(b'E'),
        b'E' => Some(b'F'),
        b'F' => Some(b'G'),
        b'G' => Some(b'H'),
        b'H' => Some(b'I'),
        b'I' => Some(b'J'),
        b'J' => Some(b'K'),
        b'K' => Some(b'L'),
        b'L' => Some(b'M'),
        b'M' => Some(b'N'),
        b'N' => Some(b'O'),
        b'O' => Some(b'P'),
        b'P' => Some(b'Q'),
        b'Q' => Some(b'R'),
        b'R' => Some(b'S'),
        b'S' => Some(b'T'),
        b'T' => Some(b'U'),
        b'U' => Some(b'V'),
        b'V' => Some(b'W'),
        b'W' => Some(b'X'),
        b'X' => Some(b'Y'),
        b'Y' => Some(b'Z'),
        b'Z' => Some(b'['),
        b'[' => Some(b'\\'),
        b'\\' => Some(b']'),
        b']' => Some(b'^'),
        b'^' => Some(b'_'),
        b'_' => Some(b'`'),
        b'`' => Some(b'a'),
        b'a' => Some(b'b'),
        b'b' => Some(b'c'),
        b'c' => Some(b'd'),
        b'd' => Some(b'e'),
        b'e' => Some(b'f'),
        b'f' => Some(b'g'),
        b'g' => Some(b'h'),
        b'h' => Some(b'i'),
        b'i' => Some(b'j'),
        b'j' => Some(b'k'),
        b'k' => Some(b'l'),
        b'l' => Some(b'm'),
        b'm' => Some(b'n'),
        b'n' => Some(b'o'),
        b'o' => Some(b'p'),
        b'p' => Some(b'q'),
        b'q' => Some(b'r'),
        b'r' => Some(b's'),
        b's' => Some(b't'),
        b't' => Some(b'u'),
        b'u' => Some(b'v'),
        b'v' => Some(b'w'),
        b'w' => Some(b'x'),
        b'x' => Some(b'y'),
        b'y' => Some(b'z'),
        b'z' => Some(b'{'),
        b'{' => Some(b'|'),
        b'|' => Some(b'}'),
        b'}' => Some(b'~'),
        b'~' => None,
        _ => unreachable!()
    }
}