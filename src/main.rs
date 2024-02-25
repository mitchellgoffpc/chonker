use std::env;
use std::fs;
use std::process;
use std::collections::HashMap;

// Helper functions

fn concat<T: Clone>(vec1: &[T], vec2: &[T]) -> Vec<T> {
    vec1.iter().cloned().chain(vec2.iter().cloned()).collect()
}

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


// API functions

fn train(bytes: &[u8], n: usize) -> Vec<(usize, usize, usize)> {
    let mut data: Vec<usize> = bytes.iter().map(|&x| x as usize).collect();
    let mut merges = Vec::new();

    for i in 0..n {
        let pair_frequencies = get_pair_frequencies(&data);
        let max_pair = pair_frequencies.iter().max_by_key(|&(_, count)| count).map(|(pair, _)| *pair).unwrap();
        data = merge(&data, max_pair, 256 + i);
        merges.push((256 + i, max_pair[0], max_pair[1]));
        // println!("{:?} ({}) -> {}", max_pair, pair_frequencies[&max_pair], 256 + i);
    }

    merges
}

fn encode(bytes: &[u8], merges: &Vec<(usize, usize, usize)>) -> Vec<usize> {
    let mut data: Vec<usize> = bytes.iter().map(|&x| x as usize).collect();
    for (index, a, b) in merges.iter() {
        data =  merge(&data, [*a, *b], *index);
    }
    data
}

fn decode(tokens: &[usize], vocab: &HashMap<usize, Vec<u8>>) -> Vec<u8> {
    tokens.iter().flat_map(|&value| vocab[&value].clone()).collect()
}


// Main

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 1 {
        eprintln!("Usage: main <file.txt>");
        process::exit(1);
    }

    let file_path = &args[0];
    let bytes = fs::read(file_path).unwrap_or_else(|err| {
        eprintln!("Error reading file '{}': {}", file_path, err);
        process::exit(1);
    });

    let merges = train(&bytes, 1000);
    println!("Done training");

    let encoded = encode(&bytes, &merges);

    let mut vocab: HashMap<usize, Vec<u8>> = (0..256).map(|i| (i, vec![i as u8])).collect();
    for (index, a, b) in merges.iter() {
        vocab.insert(*index, concat(&vocab[a], &vocab[b]));
    }

    let decoded = decode(&encoded, &vocab);
    if bytes != decoded {
        eprintln!("Error: Decoded bytes are different from original bytes");
        process::exit(1);
    }

    println!("{:?} vs {:?}", bytes.len(), encoded.len());
}
