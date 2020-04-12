pub struct CharacterClass {
    bits: u64,
}

impl Default for CharacterClass {
    fn default() -> Self {
        CharacterClass { bits: 0 }
    }
}

impl CharacterClass {
    fn to_shift(char: char) -> u8 {
        match char as u8 {
            48..=57 => (char as u8) - 48,
            65..=90 => (char as u8) - 65 + 10,
            97..=122 => (char as u8) - 97 + 36,
            _ => panic!("Character '{}' is not alphanumeric. CharacterClass can only be used with alphanumeric characters.", char),
        }
    }

    fn to_char(shift: u8) -> char {
        match shift {
            0..=9 => (shift + 48) as char,
            10..=35 => (shift + 65 - 10) as char,
            36..=61 => (shift + 97 - 36) as char,
            _ => unreachable!(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.bits == 0
    }

    pub fn to_array(&self) -> Vec<char> {
        (0..=61)
            .into_iter()
            .filter(|shift| (self.bits & ((1 as u64) << shift)) != 0)
            .map(Self::to_char)
            .collect()
    }

    pub fn add(&mut self, char: char) {
        self.bits ^= (1 as u64) << Self::to_shift(char);
    }

    pub fn add_slice(&mut self, chars: &[char]) {
        chars.iter().for_each(|&char| self.add(char));
    }

    pub fn negate(&mut self) {
        self.bits = !self.bits;
    }
}
