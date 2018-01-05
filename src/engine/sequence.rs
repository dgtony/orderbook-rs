
// Rotating index sequence
pub struct TradeSequence {
    min_id: u64,
    max_id: u64,
    last_idx: u64,
}


impl TradeSequence {
    pub fn next_id(&mut self) -> u64 {
        let new_idx = if self.last_idx < self.max_id {
            self.last_idx + 1
        } else {
            self.min_id
        };

        self.last_idx = new_idx;
        new_idx
    }
}


pub fn new_sequence_gen(min: u64, max: u64) -> TradeSequence {
    TradeSequence {
        min_id: min,
        max_id: max,
        last_idx: min,
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seq_from_zero() {
        let mut seq_gen = new_sequence_gen(0, 2);

        assert_eq!(seq_gen.last_idx, 0);
        assert_eq!(seq_gen.next_id(), 1);
        assert_eq!(seq_gen.next_id(), 2);
        assert_eq!(seq_gen.next_id(), 0);
    }

    #[test]
    fn seq_from_positive() {
        let mut seq_gen = new_sequence_gen(1, 2);

        assert_eq!(seq_gen.last_idx, 1);
        assert_eq!(seq_gen.next_id(), 2);
        assert_eq!(seq_gen.next_id(), 1);
        assert_eq!(seq_gen.next_id(), 2);
    }
}
