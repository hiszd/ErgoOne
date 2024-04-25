// Can't return str without a ref to a lifetime somewhere else,
// so instead return a buffer of the maximum i32 length
pub struct ItoaResult {
    // i32 represents values from -2147483648 to 2147483647
    buf: [u8; 11],
    len: usize,
}

impl ItoaResult {
    pub fn new() -> Self {
        Self { buf: [0u8; 11], len: 0 }
    }

    fn push(&mut self, c: u8) {
        self.len += 1;
        self.buf[11usize.checked_sub(self.len).unwrap()] = c;
    }
    
    pub fn as_str(&self) -> &str {
        core::str::from_utf8(&self.buf[11usize.checked_sub(self.len).unwrap()..]).unwrap()
    }
}

pub fn itoa(mut value: i16) -> ItoaResult {
    let mut result = ItoaResult::new();

    let neg = value < 0;

    loop {
        result.push(b'0' + u8::try_from((value % 10).abs()).unwrap());
        value /= 10;
        // check at end of loop, so 0 prints to "0"
        if value == 0 {
            break;
        }
    }
    
    if neg {
        result.push(b'-');
    }
    
    result
}
