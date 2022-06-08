use aes::*;
use rand::prelude::*;
use std::io;
use std::io::Write;
use std::time::Instant;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    let mut rng = rand::thread_rng();

    let mut times_str = String::new();
    print!("How many times do you want to run?\n> ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut times_str).unwrap();
    let times = times_str.trim().parse::<usize>().unwrap();

    let mut texts: Vec<[u8; 16]> = Vec::with_capacity(times);
    let mut keys: Vec<[u8; 16]> = Vec::with_capacity(times);

    for _ in 0..times {
        texts.push(rng.gen());
        keys.push(rng.gen());
    }

    let start = Instant::now();
    for i in 0..times {
        let _ = aes_16(&texts[i], &keys[i]);
    }
    let elapsed = start.elapsed();
    println!("aes_16: {} ms", elapsed.as_millis());
}
