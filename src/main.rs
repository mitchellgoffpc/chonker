use std::env;
use std::fs;
use std::process;
use std::collections::HashMap;

fn get_pair_frequencies(data: &[usize]) -> HashMap<[usize; 2], usize> {
    let mut pair_frequencies = HashMap::new();
    for (a, b) in data.iter().zip(data.iter().skip(1)) {
        *pair_frequencies.entry([*a, *b]).or_insert(0) += 1;
    }
    pair_frequencies
}

fn merge(data: &[usize], ids: [usize; 2], idx: usize) -> Vec<usize> {
    let mut merged_data = Vec::new();
    let mut i = 0;
    while i < data.len() {
        if i < data.len() - 1 && data[i] == ids[0] && data[i + 1] == ids[1] {
            merged_data.push(idx);
            i += 2;
        } else {
            merged_data.push(data[i]);
            i += 1;
        }
    }
    merged_data
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 1 {
        eprintln!("Usage: main <file.txt>");
        process::exit(1);
    }

    let file_path = &args[0];
    let contents = fs::read(file_path).unwrap_or_else(|err| {
        eprintln!("Error reading file '{}': {}", file_path, err);
        process::exit(1);
    });
    let original_data: Vec<usize> = contents.iter().map(|&x| x as usize).collect();
    let mut data = original_data.clone();
    let mut merges: HashMap<[usize; 2], usize> = HashMap::new();

    for i in 0..10 {
        let pair_frequencies = get_pair_frequencies(&data);
        let max_pair = pair_frequencies.iter().max_by_key(|&(_, count)| count).map(|(pair, _)| *pair).unwrap();
        data = merge(&data, max_pair, 256 + i);
        merges.insert(max_pair, 256 + i);
        println!("{:?} ({}) -> {}", max_pair, pair_frequencies[&max_pair], 256 + i);
    }

    println!("{:?}", original_data);
    println!("{:?}", data);
    println!("{:?}", merges);
}