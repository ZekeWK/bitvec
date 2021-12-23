use Error::*;

#[derive(Debug, Copy, Clone)]
struct Bits32 {
    int : u32
}

impl Bits32 {
    #[inline]
    pub fn new() -> Self {
        Bits32 {int : 0}
    }
    
    #[inline]
    pub fn zeroes() -> Self {
        Bits32 {int : 0}
    }

    #[inline]
    pub fn ones() -> Self {
        Bits32 {int : !0}
    }

    #[inline]
    pub fn get(&self, i : u8) -> bool {
        debug_assert!(i < 32);
        (self.int & (1 << i)) != 0
    }

    #[inline]
    pub fn not(&mut self, i : u8) {
        debug_assert!(i < 32);
        self.int ^= 1 << i;
    }

    #[inline]
    pub fn set_on(&mut self, i : u8) {
        debug_assert!(i < 32);
        self.int ^= 1 << i;
    }

    #[inline]
    pub fn set_off(&mut self, i : u8) {
        debug_assert!(i < 32);
        self.int &= !(1 << i)
    }

    #[inline]
    pub fn set(&mut self, i : u8, val : bool) {
        if val {self.set_on(i)}
        else   {self.set_off(i)}
    }
}

#[derive(Debug)]
struct BitVec {
    storage : Vec<Bits32>,
    next_bit : u8
}

impl BitVec {
    #[inline]
    pub fn new() -> Self {
        BitVec{storage : Vec::new(), next_bit : 32}
    }

    #[inline]
    pub fn with_capacity(capacity : u128) -> Self {
        BitVec{storage : Vec::with_capacity((capacity as usize + 31) >> 5), next_bit : 32}
    }

    #[inline]
    pub fn zeroes(amount : u128) -> Self {
        BitVec{storage : vec![Bits32::zeroes(); ((amount + 31) >> 5) as usize], next_bit : ((amount + 31 & 31)) as u8 + 1}
    }

    #[inline]
    pub fn ones(amount : u128) -> Self {
        BitVec{storage : vec![Bits32::ones(); ((amount + 31) >> 5) as usize], next_bit : ((amount + 31) & 31) as u8 + 1}
    }

    #[inline]
    pub fn len(&self) -> u128 {
        (((self.storage.len()) as u128) << 5) + (self.next_bit as u128) -32
    }

    #[inline]
    pub fn get(&self, i : u128) -> Option<bool> {
        if i >= self.len() {return None} 

        Some(self.storage[(i >> 5) as usize].get((i & 31) as u8))
    }

    #[inline]
    pub fn set(&mut self, i : u128, val : bool) -> Result<(), Error>{
        if i >= self.len() {return Err(IndexError)} 

        self.storage[(i >> 5) as usize].set((i & 31) as u8, val);
        Ok(())    
    }

    #[inline]
    pub fn push(&mut self, val : bool) {
        if self.next_bit >= 32 {self.storage.push(Bits32::new()); self.next_bit = 0}

        self.storage.last_mut().unwrap().set( self.next_bit, val);
        self.next_bit += 1;
    }
}

#[derive(Debug, Clone, Copy)]
enum Error {
    IndexError
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bits_set_get_not() {
        let mut bits = Bits32::new();
        
        for i in (0..32u8).into_iter().step_by(2){
           bits.set(i, true);
        }

        assert!(bits.get(4));
        assert!(!bits.get(3));
        assert!(bits.get(0));
        assert!(!bits.get(31));

        bits.not(4);
        assert!(!bits.get(4))
    }

    #[test]
    fn bit_vec_set_get_not() {
        let mut bit_vec = BitVec::new();

        for i in 0..100 {
            let len = bit_vec.len();
            assert!(len == 2*i, format!("Length should be {}, was : {}", 2*i, len));
            bit_vec.push(true);
            
            let len = bit_vec.len();
            assert!(len == 2*i+1, format!("Length should be {}, was : {}", 2*i+1, len));
            bit_vec.push(false);
        }

        assert!(bit_vec.get(4).unwrap());
        assert!(!bit_vec.get(3).unwrap());
        
        bit_vec.set(4, false).unwrap();
        assert!(!bit_vec.get(4).unwrap());
    }

    #[test]
    fn zeroes_ones() {
        for i in 0..100 {
            assert_eq!(BitVec::zeroes(i).len(), i);
            assert_eq!(BitVec::ones(i).len(), i);
        }
    }
}