const BIT_ARRAY_SIZE: usize = 100;

pub struct BloomFilter {
    bit_array: Vec<bool>,
    functions : [fn(&str) -> usize; 3]
}

impl BloomFilter {
    pub fn new() -> BloomFilter {
        BloomFilter {
            bit_array: vec![false; BIT_ARRAY_SIZE],
            functions: [
                djb2_hash,
                fnv1a_hash,
                basic_hash
            ],
        }
    }

    pub fn new_with_values(values: Vec<&str>) -> BloomFilter {
        let mut filter = BloomFilter {
            bit_array: vec![false; BIT_ARRAY_SIZE],
            functions: [
                djb2_hash,
                fnv1a_hash,
                basic_hash
            ],
        };
        values.iter().for_each(|value| {
            filter.add(value);
        });
        filter
    }

    fn add(&mut self, value: &str) {
        self.functions.iter().for_each(|function| {
            let index = function(value);
            self.bit_array[index] = true;
        })
    }

    pub fn check_value(&self, value: &str) -> bool {
        self.functions.iter().all(|function| {
            let index = function(value);
            self.bit_array[index]
        })
    }
}


fn djb2_hash(value: &str) -> usize {
    let mut hash: usize = 5381;
    for byte in value.bytes() {
        hash = hash.wrapping_shl(5).wrapping_add(hash).wrapping_add(byte as usize);
    }
    hash % BIT_ARRAY_SIZE
}

fn fnv1a_hash(value: &str) -> usize {
    let mut hash: usize = 14695981039346656037;

    for byte in value.bytes() {
        hash ^= byte as usize;
        hash = hash.wrapping_mul(1099511628211);
    }
    hash % BIT_ARRAY_SIZE
}

fn basic_hash(value: &str) -> usize {
    let mut hash:usize = 0;
    for byte in value.bytes() {
        hash += byte as usize;
    }
    hash % BIT_ARRAY_SIZE
}