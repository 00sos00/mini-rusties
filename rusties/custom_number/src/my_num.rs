#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MyNum {
    pub data: Vec<u32>,
}

impl MyNum {
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![0; size],
        }
    }

    pub fn set_nth_bit(&mut self, n: usize) {
        assert!((((n - 1) as f32 / 8.0).ceil() as usize) < self.data.len());

        let index = (n - 1) / 8;
        let bit_to_change = (n - 1) % 8;

        let byte = self.data.get_mut(index).unwrap();

        *byte |= 1 << bit_to_change;
    }

    pub fn and(&self, rhs: &Self) -> Self {
        assert_eq!(self.data.len(), rhs.data.len());
        
        let data = self
            .data
            .iter()
            .zip(rhs.data.iter())
            .map(|(x1, x2)| x1 & x2)
            .collect();

        Self { data }
    }

    pub fn or(&self, rhs: &Self) -> Self {
        assert_eq!(self.data.len(), rhs.data.len());

        let data = self
            .data
            .iter()
            .zip(rhs.data.iter())
            .map(|(x1, x2)| x1 | x2)
            .collect();

        Self { data }
    }

    pub fn xor(&self, rhs: &Self) -> Self {
        assert_eq!(self.data.len(), rhs.data.len());

        let data = self
            .data
            .iter()
            .zip(rhs.data.iter())
            .map(|(x1, x2)| x1 ^ x2)
            .collect();

        Self { data }
    }
}
