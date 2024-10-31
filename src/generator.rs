const CHARACTERS: [char; 96] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '-', '_', '=', '+',
    '[', ']', '{', '}', '|', ';', ':', '\'', ',', '.', '<', '>', '?', '/', '\\', '"', '`', '~',
    ' ', '\n',
];

pub(crate) struct CombinationGenerator {
    length: usize,
    indices: Vec<usize>,
    finished: bool,
}

impl CombinationGenerator {
    pub fn new(length: usize) -> Self {
        Self {
            length,
            indices: vec![0; length],
            finished: false,
        }
    }
}

impl Iterator for CombinationGenerator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        let combination: String = self.indices.iter().map(|&i| CHARACTERS[i]).collect();

        let mut i: usize = self.length - 1;
        loop {
            self.indices[i] += 1;
            if self.indices[i] < CHARACTERS.len() || i == 0 {
                break;
            }
            self.indices[i] = 0;
            i = i.saturating_sub(1);
        }

        if self.indices[0] == CHARACTERS.len() {
            self.finished = true;
        }

        Some(combination)
    }
}
