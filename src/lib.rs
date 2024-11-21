pub const ILLEGAL_ENCODE_VAL: u8 = 255;
pub static ENCODE_TABLE: [u8; 256] = {
    let mut t = [0u8; 256];
    let mut i = 0;
    while i < t.len() {
        t[i] = match i as u8 {
            b'0'..=b'9' => i as u8 - b'0',
            b'a'..=b'z' => 10 + (i as u8 - b'a'),
            _ => ILLEGAL_ENCODE_VAL, // shouldn't ever be read
        };
        i += 1;
    }
    t
};

pub static DECODE_TABLE: [u8; 256] = {
    let mut t = [0u8; 256];
    let mut i = 0;
    while i < t.len() {
        t[i] = match i as u8 {
            0..10 => i as u8 + b'0',
            10..36 => i as u8 - 10 + b'a',
            _ => 0,
        };
        i += 1;
    }
    t
};

/// input should be no more than 20 bytes, and should only contain [a-z0-9].
pub fn pack(input: &[u8]) -> [u32; 4] {
    assert!(input.len() <= 20, "input should be no more than 20 bytes");
    let mut packed = [0u32; 4];
    for (chunk5, out_word) in input.chunks(5).zip(packed.iter_mut()) {
        for (idx, &byte) in chunk5.iter().enumerate() {
            let mapped_byte = ENCODE_TABLE[byte as usize];
            debug_assert_ne!(ILLEGAL_ENCODE_VAL, mapped_byte, "got bad character");
            *out_word |= (mapped_byte as u32) << (idx * 6);
        }
    }
    packed
}

/// out should be at least 20 bytes.
/// The original text will be written into the buffer, padded by null bytes until 20.
pub fn unpack(input: &[u32; 4], out: &mut [u8]) {
    assert!(out.len() >= 20, "output slice should be at least 20 bytes long");
    for (word_idx, &word) in input.iter().enumerate() {
        for i in 0..5 {
            out[word_idx * 5 + i] = DECODE_TABLE[(word as usize >> (i * 6)) & 0b11_1111];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    static TEST_CASES: &[&str] = &[
        "a",
        "ab",
        "abc",
        "a0bc1d2",
        "012345678912345678",
        "0123456789123456789",
        "01234567891234567890",
    ];

    #[test]
    fn round_trip() {
        for case in TEST_CASES {
            let mut out = [0u8; 20];
            unpack(&pack(case.as_bytes()), &mut out);
            assert_eq!(case.as_bytes(), &out[..case.len()], "case: {case}");
        }
    }
}
