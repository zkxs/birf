// there are 128 ascii characters (including NUL)
// the first 32 are non-printing (except TAB), so there's 31 we won't use
// the last one (DEL) is non-printing, so there's 1 we won't use
// 128 - 31 - 1 == 96
pub const FIRST_CHAR: u8 = b'0';
pub const CHARS: [u8; 9] = [
    b'1',
    b'2',
    b'3',
    b'4',
    b'5',
    b'6',
    b'7',
    b'8',
    b'9',
];

pub fn increment_char(char: u8) -> Option<u8> {
    match char {
        b'0' => Some(b'1'),
        b'1' => Some(b'2'),
        b'2' => Some(b'3'),
        b'3' => Some(b'4'),
        b'4' => Some(b'5'),
        b'5' => Some(b'6'),
        b'6' => Some(b'7'),
        b'7' => Some(b'8'),
        b'8' => Some(b'9'),
        b'9' => None,
        _ => unreachable!()
    }
}
