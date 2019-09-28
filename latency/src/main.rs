#![feature(core_intrinsics, asm)]

use std::fmt;
use std::fs::File;
use std::io::Write;
use std::thread;
use std::time::Instant;

const NUM_LOOPS_1: usize = 100_000;
const NUM_LOOPS_2: usize = 10_000;
const NUM_LOOPS_3: usize = 1_000;
const NUM_LOOPS_4: usize = 100;

const START_SIZE: usize = 1;
const UP_TO: usize = 26;
const NUM_THREADS: usize = 4;

struct SIZE {
    n: usize
}

impl fmt::Display for SIZE {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let size = self.n * 8;

        if (64 - size.leading_zeros()) <= 11 {
            write!(f, "{} B", size)
        }
        else if (64 - size.leading_zeros()) <= 21 {
            write!(f, "{} KB", size / 1024)
        }
        else if (64 - size.leading_zeros()) <= 31 {
            write!(f, "{} MB", size / (1024 * 1024))
        }
        else if (64 - size.leading_zeros()) <= 41{
            write!(f, "{} GB", size / (1024 * 1024 * 1024))
        }
        else {
            write!(f, "{}", size)
        }
    }
}


fn do_read(size: usize) -> f32 {
    let mut nums = vec![];

    (0..size).for_each(|_| nums.push(1u64));
    nums.shrink_to_fit();

    let mut sum = 0u64;
    let mut num_loops = 0;

    let time_taken = unsafe {
        std::intrinsics::prefetch_read_data(&nums, 3);
        if size <= (4096 * 4) {
            num_loops = NUM_LOOPS_1;
            let now = Instant::now();
            for _ in 0..NUM_LOOPS_1 {
                for n in &nums {
                    sum = sum + *n;
                }
            }
            now.elapsed().as_secs_f32()
        } else if size <= (4096 * 4 * 4) {
            num_loops = NUM_LOOPS_2;
            let now = Instant::now();
            for _ in 0..NUM_LOOPS_2 {
                for n in &nums {
                    sum = sum + *n;
                }
            }
            now.elapsed().as_secs_f32()
        } else if size <= (4096 * 4 * 4 * 4) {
            num_loops = NUM_LOOPS_3;
            let now = Instant::now();
            for _ in 0..NUM_LOOPS_3 {
                for n in &nums {
                    sum = sum & *n;
                }
            }
            now.elapsed().as_secs_f32()
        } else {
            num_loops = NUM_LOOPS_4;
            let now = Instant::now();
            for _ in 0..NUM_LOOPS_4 {
                for n in &nums {
                    sum = sum + *n;
                }
            }
            now.elapsed().as_secs_f32()
        }
    };

    dbg!(&sum);

    let bandwidth = (time_taken * 10e9) / (size * num_loops) as f32;
    bandwidth
}

fn main() {
    for i in 0..UP_TO {
        let multiplier = 2f32.powi(i as i32);
        let size = (START_SIZE as f32 * multiplier) as usize;

        let mut threads = vec![];
        for _ in 0..NUM_THREADS {
            threads.push(thread::spawn(move || do_read(size.clone())))
        }

        let bandwidths: f32 = threads.into_iter().map(|t| t.join().unwrap()).sum();
        println!("{}:\t{:?} nanoseconds", SIZE {n: size}, bandwidths / NUM_THREADS as f32);
    }
}
