use std::ops::Index;


enum CacheKey {
    A,B,C,
}

#[derive(Debug)]
struct CacheValue {
    value: usize,
}

struct CacheMap<const N: usize> {
    cache: [CacheValue; N],
}

impl CacheValue {
    fn new(value: usize) -> Self { Self { value } }
}


impl <const N: usize> CacheMap<N> {
    fn from_slice(sl: [CacheValue; N]) -> Self {
        Self { cache: sl, }
    } 
}

impl <const N: usize> Index<CacheKey> for CacheMap<N> {
    type Output = CacheValue;

    fn index(&self, index: CacheKey) -> &Self::Output {
        match index {
            CacheKey::A => self.cache.index(0),
            CacheKey::B => self.cache.index(1),
            CacheKey::C => self.cache.index(2),
        }
    }
}


fn main() {
    let mut cache = CacheMap::from_slice([CacheValue::new(1),CacheValue::new(3),CacheValue::new(2),]);

    let a = &cache[CacheKey::A];

    println!("{a:?}");
}