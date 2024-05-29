use std::fs;
use std::process;
use std::collections::HashMap;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Args {
    #[structopt(name = "command")]
    command: String,
    #[structopt(name = "input")]
    input: String,
    #[structopt(short, long)]
    vocab: Option<String>,
}


// Helper functions

fn read(file_path: &str) -> Vec<u8> {
    fs::read(file_path).unwrap_or_else(|err| {
        eprintln!("Error reading file '{}': {}", file_path, err);
        process::exit(1);
    })
}

fn read_merges(file_path_opt: &Option<String>) -> Vec<(usize, usize, usize)> {
    let file_path = file_path_opt.as_ref().unwrap_or_else(|| {
        eprintln!("Vocab file not provided");
        process::exit(1);
    });
    let file = fs::read_to_string(file_path).unwrap_or_else(|err| {
        eprintln!("Error reading file '{}': {}", file_path, err);
        process::exit(1);
    });

    file.lines().map(|line| {
        let mut parts = line.split_whitespace();
        let index = parts.next().unwrap().parse().unwrap();
        let a = parts.next().unwrap().parse().unwrap();
        let b = parts.next().unwrap().parse().unwrap();
        (index, a, b)
    }).collect()
}

fn concat<T: Clone>(vec1: &[T], vec2: &[T]) -> Vec<T> {
    vec1.iter().cloned().chain(vec2.iter().cloned()).collect()
}

fn get_pair_frequencies(data: &[usize]) -> HashMap<(usize, usize), usize> {
    let mut pair_frequencies: HashMap<(usize, usize), usize> = HashMap::new();
    for i in 0..data.len() - 1 {
        *pair_frequencies.entry((data[i], data[i + 1])).or_insert(0) += 1;
    }
    pair_frequencies
}

fn merge(data: &[usize], ids: (usize, usize), idx: usize) -> Vec<usize> {
    let mut merged_data = Vec::new();
    let mut i = 0;
    while i < data.len() {
        if i < data.len() - 1 && data[i] == ids.0 && data[i + 1] == ids.1 {
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
        merges.push((256 + i, max_pair.0, max_pair.1));
    }

    merges
}

fn encode(bytes: &[u8], merges: &Vec<(usize, usize, usize)>) -> Vec<usize> {
    let mut data: Vec<usize> = bytes.iter().map(|&x| x as usize).collect();
    for (index, a, b) in merges.iter() {
        data =  merge(&data, (*a, *b), *index);
    }
    data
}

fn decode(tokens: &[usize], merges: &Vec<(usize, usize, usize)>) -> Vec<u8> {
    let mut vocab: HashMap<usize, Vec<u8>> = (0..256).map(|i| (i, vec![i as u8])).collect();
    for (index, a, b) in merges.iter() {
        vocab.insert(*index, concat(&vocab[a], &vocab[b]));
    }
    tokens.iter().flat_map(|&value| vocab[&value].clone()).collect()
}


// Main

fn main() {
    let args = Args::from_args();

    match args.command.as_str() {
        "train" => {
            let bytes = read(&args.input);
            let merges = train(&bytes, 1000);
            for merge in &merges {
                println!("{} {} {}", merge.0, merge.1, merge.2);
            }

            // self test
            let encoded = encode(&bytes, &merges);
            let decoded = decode(&encoded, &merges);
            assert_eq!(bytes, decoded);
        },
        "encode" => {
            let bytes = read(&args.input);
            let merges = read_merges(&args.vocab);
            let encoded = encode(&bytes, &merges);
            for token in encoded {
                println!("{}", token);
            }
        },
        "decode" => {
            let tokens: Vec<usize> = fs::read_to_string(&args.input).unwrap().lines().map(|line| line.parse().unwrap()).collect();
            let merges = read_merges(&args.vocab);
            let decoded = decode(&tokens, &merges);
            println!("{}", String::from_utf8_lossy(&decoded));
        },
        _ => {
            eprintln!("Invalid command '{}'", args.command);
            process::exit(1);
        }
    }
}
