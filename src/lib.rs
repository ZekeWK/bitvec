#![allow(dead_code)]
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
        self.int |= 1 << i;
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
pub struct BitVec {
    storage : Vec<Bits32>,
    next_bit : u8
}

impl BitVec {
    #[inline]
    pub fn new() -> Self {
        BitVec{storage : Vec::new(), next_bit : 32}
    }

    #[inline]
    pub fn with_capacity(capacity : usize) -> Self {
        BitVec{storage : Vec::with_capacity((capacity + 31) >> 5), next_bit : 32}
    }

    #[inline]
    pub fn zeroes(amount : usize) -> Self {
        BitVec{storage : vec![Bits32::zeroes(); (amount + 31) >> 5], next_bit : ((amount + 31 & 31)) as u8 + 1}
    }

    #[inline]
    pub fn ones(amount : usize) -> Self {
        BitVec{storage : vec![Bits32::ones(); (amount + 31) >> 5], next_bit : ((amount + 31) & 31) as u8 + 1}
    }

    #[inline]
    pub fn len(&self) -> usize {
        ((self.storage.len()) << 5) + (self.next_bit as usize) -32
    }

    #[inline]
    pub fn get(&self, i : usize) -> Option<bool> {
        if i >= self.len() {return None} 

        Some(self.storage[i >> 5].get((i & 31) as u8))
    }

    #[inline]
    pub fn set(&mut self, i : usize, val : bool) -> Result<(), Error>{
        if i >= self.len() {return Err(IndexError)} 

        self.storage[i >> 5].set((i & 31) as u8, val);
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
pub enum Error {
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
            assert!(len == 2*i);
            bit_vec.push(true);
            
            let len = bit_vec.len();
            assert!(len == 2*i+1);
            bit_vec.push(false);
        }

        assert!(bit_vec.get(98).unwrap());
        assert!(!bit_vec.get(97).unwrap());
        
        bit_vec.set(98, false).unwrap();
        assert!(!bit_vec.get(98).unwrap());
        bit_vec.set(97, true).unwrap();
        assert!(bit_vec.get(97).unwrap());
    }

    #[test]
    fn zeroes_ones() {
        for i in 0..100 {
            assert_eq!(BitVec::zeroes(i).len(), i);
            assert_eq!(BitVec::ones(i).len(), i);
        }
    }
}
