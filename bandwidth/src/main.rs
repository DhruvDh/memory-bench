use rand::prelude::*;
use rayon;
use packed_simd::{u8x32};
use std::time::Instant;
use std::thread;

#[inline(always)]
fn do_read_write (len: usize) -> f32 {
    let mut rng = rand::thread_rng();

    // creates a vector of 999 (32 lane unsigned 8 bit) integers
    let mut random_ints: Vec<u8x32> = (0..len).map(|_| {
        let num: Vec<u8> = (0..32).map(|_| rng.gen()).collect();
        u8x32::from_slice_unaligned(&num)
    }).collect();

    let rotate_by = {
        let num: Vec<u8> = (0..32).map(|_| rng.gen()).collect();
        u8x32::from_slice_unaligned(&num)
    };

    let now = Instant::now();
    
    for _ in 0..100_000 {
        for nums in random_ints.iter_mut() {
            *nums = nums.rotate_left(rotate_by);
        }
    }

    let time_taken = now.elapsed().as_secs_f32();
    dbg!(&random_ints);

    time_taken
}

fn main() {
    let num_threads = 4;
    let reps = 100;

    let time_taken: f32 = (0..reps).map(|_| {
        let mut threads = vec![];      

        for _ in 0..num_threads {
            threads.push(thread::spawn(|| do_read_write(256)))
        }

        let time_taken: f32 = threads.into_iter()
                                .map(|t| t.join().unwrap())
                                .sum();
        
        time_taken/(num_threads as f32)
    }).sum();


    println!("Avg. time for each thread is {:?} seconds.", time_taken/(reps as f32));
}
