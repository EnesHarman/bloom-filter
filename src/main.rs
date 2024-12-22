use bloom_filter::BloomFilter;

fn main() {
    let inputs = vec!["Lorem", "ipsum", "dolor", "sit", "amet", "minim", "enim"];
    let search_inputs = vec!["enes", "maximus", "java", "ipsum"];

    let filter = BloomFilter::new_with_values(inputs);
    for search_input in search_inputs {
        let result = filter.check_value(search_input);
        match result {
            true => println!("Key: {} is used in map", search_input),
            false => println!("Key: {} is NOT used in map", search_input),
        }
    }
}

