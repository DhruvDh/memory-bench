#![feature(mem_take)]

use rand::prelude::*;
use rayon;
use packed_simd::{u8x32};
use std::time::Instant;
use std::thread;

const NUM_LOOPS: usize = 100_000;
const NUM_THREADS: usize = 8;
const NUM_OF_256BIT_VECTORS: usize = 1023 * 8;

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
    
    for _ in 0..NUM_LOOPS {
        for nums in random_ints.iter_mut() {
            *nums = nums.rotate_left(rotate_by);
            // *nums = std::mem::take(nums);
        }
    }

    let time_taken = now.elapsed().as_secs_f32();
    // dbg!(&random_ints);

    time_taken
}

fn main() {
    let time_taken: f32 = (1..21).map(|i| {
        let mut threads = vec![];      

        for _ in 0..NUM_THREADS {
            let size = NUM_OF_256BIT_VECTORS * i;
            threads.push(thread::spawn(move || do_read_write(size.clone())))
        }

        let time_taken: f32 = threads.into_iter()
                                .map(|t| t.join().unwrap())
                                .sum();
        
        println!("Size is {:?} kB.", (NUM_OF_256BIT_VECTORS * i * 32 / 1024));
        println!("{:?} seconds.", time_taken/(NUM_THREADS as f32));
        
        let memory_used = NUM_OF_256BIT_VECTORS * 32;
        let bandwidth = ((memory_used * NUM_LOOPS * NUM_THREADS) as f32)/time_taken;
        let bandwidth = bandwidth / (1024 * 1024 * 1024) as f32;
        println!("Bandwidth is {:?} GBps.", bandwidth);
        println!("/n/n");
        time_taken/(NUM_THREADS as f32)
    }).sum();

    dbg!(&time_taken);
}
