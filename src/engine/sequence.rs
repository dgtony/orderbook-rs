
// Rotating index sequence
pub struct TradeSequence {
    min_id: u64,
    max_id: u64,
    current_idx: u64,
}


impl TradeSequence {
    pub fn next_id(&mut self) -> u64 {
        let next_id = self.current_idx;

        // update index
        if (next_id + 1) <= self.max_id {
            self.current_idx += 1;
        } else {
            self.current_idx = self.min_id;
        }

        next_id
    }
}


pub fn new_sequence_gen(min: u64, max: u64) -> TradeSequence {
    TradeSequence {
        min_id: min,
        max_id: max,
        current_idx: min,
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seq_from_zero() {
        let mut seq_gen = new_sequence_gen(0, 2);

        assert_eq!(seq_gen.current_idx, 0);
        assert_eq!(seq_gen.next_id(), 0);
        assert_eq!(seq_gen.next_id(), 1);
        assert_eq!(seq_gen.next_id(), 2);
        assert_eq!(seq_gen.next_id(), 0);
    }

    #[test]
    fn seq_from_positive() {
        let mut seq_gen = new_sequence_gen(1, 2);

        assert_eq!(seq_gen.current_idx, 1);
        assert_eq!(seq_gen.next_id(), 1);
        assert_eq!(seq_gen.next_id(), 2);
        assert_eq!(seq_gen.next_id(), 1);
    }
}
