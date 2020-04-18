use core::ops::RangeInclusive;

const CHAR_SETS: [RangeInclusive<u8>; 3] = [
    RangeInclusive::new(48, 57),
    RangeInclusive::new(65, 90),
    RangeInclusive::new(97, 122),
];

const FF: [usize; 2] = [6, 6];
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

pub struct CharacterClass128 {
    bits: u128,
}

impl Default for CharacterClass128 {
    fn default() -> Self {
        CharacterClass128 { bits: 0 }
    }
}

impl CharacterClass128 {
    fn to_shift(char: char) -> u8 {
        char as u8
    }

    fn to_char(shift: u8) -> char {
        match shift {
            0..=127 => shift as char,
            _ => unreachable!(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.bits == 0
    }

    pub fn to_array(&self) -> Vec<char> {
        (0..=127)
            .into_iter()
            .filter(|shift| (self.bits & ((1 as u128) << shift)) != 0)
            .map(Self::to_char)
            .collect()
    }

    pub fn add(&mut self, char: char) {
        self.bits ^= (1 as u128) << Self::to_shift(char);
    }

    pub fn add_slice(&mut self, chars: &[char]) {
        chars.iter().for_each(|&char| self.add(char));
    }

    pub fn negate(&mut self) {
        self.bits = !self.bits;
    }
}

pub fn to_character_class<T: AsRef<[char]>>(set: T) -> String {
    let mut bits: u128 = 0;
    for &char in set.as_ref() {
        bits ^= (1 as u128) << char as u8;
    }
    let mut string: String = Default::default();
    /*for shift in (0 as u8)..(127 as u8) {
        if bits & ((1 as u128) << shift) != 0 {
            string.push(shift as char);
        }
    }*/
    for &char in set.as_ref() {
        string.push(char);
    }
    //48..=57 => (char as u8) - 48,
    //65..=90 => (char as u8) - 65 + 10,
    //97..=122 => (char as u8) - 97 + 36,
    string
}
