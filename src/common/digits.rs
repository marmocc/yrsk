#[derive(Clone, Copy)]
pub struct Digits<const C: usize>([u8; C]);

impl<const C: usize> Digits<C> {
    #[inline]
    pub const fn as_bytes(self) -> [u8; C] {
        self.0
    }
}

impl Digits<3> {
    #[inline]
    pub const fn from_u8(value: u8) -> Self {
        Self([
            b'0' + value / 100,
            b'0' + value / 10 % 10,
            b'0' + value % 10,
        ])
    }

    #[inline]
    pub fn look_u8(value: u8) -> Self {
        static DIGITS3: [Digits<3>; 256] = Digits::<3>::table();
        DIGITS3[value as usize]
    }

    const fn table() -> [Self; 256] {
        let mut table = [Self::from_u8(0); 256];
        let mut i = 0;

        while i < 256 {
            table[i] = Self::from_u8(i as u8);
            i += 1;
        }

        table
    }
}

impl Digits<5> {
    #[inline]
    pub const fn from_u16(value: u16) -> Self {
        Self([
            b'0' + (value / 10000) as u8,
            b'0' + (value / 1000 % 10) as u8,
            b'0' + (value / 100 % 10) as u8,
            b'0' + (value / 10 % 10) as u8,
            b'0' + (value % 10) as u8,
        ])
    }
}
