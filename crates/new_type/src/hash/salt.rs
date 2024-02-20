use rand::Rng;

pub struct Salt(u128);

impl std::fmt::Display for Salt {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Salt(MASKED)")
    }
}

impl std::fmt::Debug for Salt {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Salt(MASKED)")
    }
}

impl From<Salt> for Vec<u8> {
    #[inline]
    fn from(salt: Salt) -> Self {
        salt.as_bytes().to_vec()
    }
}

impl Salt {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();

        Self::new_with_rng(&mut rng)
    }

    #[inline]
    pub fn new_with_rng<R: Rng>(rng: &mut R) -> Self {
        Self(rng.gen())
    }

    #[inline]
    pub const fn new_unchecked(salt: u128) -> Self {
        Self(salt)
    }

    #[inline]
    pub const fn as_bytes(&self) -> [u8; std::mem::size_of::<u128>()] {
        self.0.to_be_bytes()
    }

    #[inline]
    pub const fn as_u128(&self) -> u128 {
        self.0
    }
}
