#![feature(core_intrinsics, asm)]

use std::time::Instant;
use packed_simd::{u64x4};
use std::thread;
use std::fmt;
use std::fs::File;
use std::io::Write;

const NUM_LOOPS_1: usize = 100_000;
const NUM_LOOPS_2: usize = 10_000;
const NUM_LOOPS_3: usize = 1_000;
const NUM_LOOPS_4: usize = 100;

const START_SIZE: usize = 2;
const UP_TO: usize = 23;
const NUM_THREADS: usize = 32;

struct SIZE {
    n: usize
}

impl fmt::Display for SIZE {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let size = self.n * 4 * 64 / 8;

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
    nums.push(u64x4::new(1, 2, 3, 4));

    nums = nums.iter()
               .cycle()
               .take(size)
               .map(|p| *p)
               .collect();
    
    let mut sum = u64x4::new(0, 0, 0, 0);
    let mut num_loops = 0;

    let time_taken = unsafe {
        std::intrinsics::prefetch_read_data(&nums, 3);
        
        if size <= (4096 * 4) {
            num_loops = NUM_LOOPS_1;
            let now  = Instant::now();
            for _ in 0..NUM_LOOPS_1 {
                for n in &nums {
                    sum = sum & *n;
                }
            }
            now.elapsed().as_secs_f32()
        }
        else if size <= (4096 * 4 * 4) {
            num_loops = NUM_LOOPS_2;
            let now  = Instant::now();
            for _ in 0..NUM_LOOPS_2 {
                for n in &nums {
                    sum = sum & *n;
                }
            }
            now.elapsed().as_secs_f32()
        }
        else if size <= (4096 * 4 * 4 * 4) {
            num_loops = NUM_LOOPS_3;
            let now  = Instant::now();
            for _ in 0..NUM_LOOPS_3 {
                for n in &nums {
                    sum = sum & *n;
                }
            }
            now.elapsed().as_secs_f32()
        }
        else {
            num_loops = NUM_LOOPS_4;
            let now  = Instant::now();
            for _ in 0..NUM_LOOPS_4 {
                for n in &nums {
                    sum = sum & *n;
                }
            }
            now.elapsed().as_secs_f32()
        }
    };

    dbg!(&sum);

    let bandwidth = (size * num_loops * 8 * 8) as f32/ (time_taken * 10e9);
    bandwidth
}


fn do_read_write(size: usize) -> f32 {
    let size = size / 2;

    let mut nums_A = vec![];
    nums_A.push(0u64);

    nums_A = nums_A.iter()
               .cycle()
               .take(size * 4)
               .map(|p| *p)
               .collect();
    
    let mut nums_B = vec![];
    nums_B.push(u64x4::new(1, 2, 3, 4));

    nums_B = nums_B.iter()
               .cycle()
               .take(size)
               .map(|p| *p)
               .collect();
    
    let mut num_loops = 0;

    let time_taken = unsafe {
        if size <= (4096 * 4) {
            num_loops = NUM_LOOPS_1;
            let now  = Instant::now();

            for _ in 0..NUM_LOOPS_1 {
                for (slice, n) in nums_A.chunks_exact_mut(4).zip(&nums_B) {
                    n.write_to_slice_aligned_unchecked(slice);
                }
            }

            now.elapsed().as_secs_f32()
        }
        else if size <= (4096 * 4 * 4) {
            num_loops = NUM_LOOPS_2;
            let now  = Instant::now();

            for _ in 0..NUM_LOOPS_1 {
                for (slice, n) in nums_A.chunks_exact_mut(4).zip(&nums_B) {
                    n.write_to_slice_aligned_unchecked(slice);
                }
            }

            now.elapsed().as_secs_f32()
        }
        else if size <= (4096 * 4 * 4 * 4) {
            num_loops = NUM_LOOPS_3;
            let now  = Instant::now();

            for _ in 0..NUM_LOOPS_1 {
                for (slice, n) in nums_A.chunks_exact_mut(4).zip(&nums_B) {
                    n.write_to_slice_aligned_unchecked(slice);
                }
            }

            now.elapsed().as_secs_f32()
        }
        else {
            num_loops = NUM_LOOPS_4;
            let now  = Instant::now();

            for _ in 0..NUM_LOOPS_1 {
                for (slice, n) in nums_A.chunks_exact_mut(4).zip(&nums_B) {
                    n.write_to_slice_aligned_unchecked(slice);
                }
            }

            now.elapsed().as_secs_f32()
        }
    };

    dbg!(&nums_A);
    dbg!(&nums_B);

    let bandwidth = (size * num_loops * 8 * 8) as f32/ (time_taken * 10e9);
    bandwidth
}


fn do_write(size: usize) -> f32 {
    let mut nums = vec![];
    nums.push(0u64);

    nums = nums.iter()
               .cycle()
               .take(size * 4)
               .map(|p| *p)
               .collect();
    
    let mut num_loops = 0;

    let time_taken = unsafe {
        std::intrinsics::prefetch_write_data(&nums, 3);
        let n = u64x4::new(1, 2, 3, 4);

        if size <= (4096 * 4) {
            num_loops = NUM_LOOPS_1;
            let now  = Instant::now();
            for _ in 0..NUM_LOOPS_1 {
                for slice in nums.chunks_exact_mut(4) {
                    n.write_to_slice_aligned_unchecked(slice);
                }
            }
            now.elapsed().as_secs_f32()
        }
        else if size <= (4096 * 4 * 4) {
            num_loops = NUM_LOOPS_2;
            let now  = Instant::now();
            for _ in 0..NUM_LOOPS_2 {
                for slice in nums.chunks_exact_mut(4) {
                    n.write_to_slice_aligned_unchecked(slice);
                }
            }
            now.elapsed().as_secs_f32()
        }
        else if size <= (4096 * 4 * 4 * 4) {
            num_loops = NUM_LOOPS_3;
            let now  = Instant::now();
            for _ in 0..NUM_LOOPS_3 {
                for slice in nums.chunks_exact_mut(4) {
                    n.write_to_slice_aligned_unchecked(slice);
                }
            }
            now.elapsed().as_secs_f32()
        }
        else {
            num_loops = NUM_LOOPS_4;
            let now  = Instant::now();
            for _ in 0..NUM_LOOPS_4 {
                for slice in nums.chunks_exact_mut(4) {
                    n.write_to_slice_aligned_unchecked(slice);
                }
            }
            now.elapsed().as_secs_f32()
        }
    };

    dbg!(&nums);

    let bandwidth = (size * num_loops * 8 * 8) as f32/ (time_taken * 10e9);
    bandwidth
}


fn main() {

    let mut reads = vec![];
    let mut writes = vec![];
    let mut read_writes = vec![];

    if std::env::args().len() > 1 {
        let i: i32 = std::env::args().nth(1).unwrap().parse::<i32>().unwrap();
        let multiplier = 2f32.powi(i);
        let size = (START_SIZE as f32 * multiplier) as usize;


        // ------------------- READ --------------------------
        let mut threads = vec![];
        for _ in 0..NUM_THREADS {
            threads.push(thread::spawn(move || do_read(size.clone())))
        }

        let bandwidths: f32 = threads.into_iter()
                                .map(|t| t.join().unwrap())
                                .sum();
        
        let mem = SIZE { n: size };
        let total_mem = SIZE { n: (size * NUM_THREADS) };

        println!("READ\t{}\t------\t{} GBps.", mem, bandwidths);

    
        // ------------------- WRITE --------------------------
        let mut threads = vec![];
        for _ in 0..NUM_THREADS {
            threads.push(thread::spawn(move || do_write(size.clone())))
        }

        let bandwidths: f32 = threads.into_iter()
                                .map(|t| t.join().unwrap())
                                .sum();
        
        let mem = SIZE { n: size };
        let total_mem = SIZE { n: (size * NUM_THREADS) };

        println!("WROTE\t{}\t------\t{} GBps.", mem, bandwidths);   
    }
    else {
        // ------------------- READ --------------------------
        for i in 0..UP_TO {
            let multiplier = 2f32.powi(i as i32);
            let size = (START_SIZE as f32 * multiplier) as usize;

            let mut threads = vec![];
            for _ in 0..NUM_THREADS {
                threads.push(thread::spawn(move || do_read(size.clone())))
            }

            let bandwidths: f32 = threads.into_iter()
                                    .map(|t| t.join().unwrap())
                                    .sum();
            
            let mem = SIZE { n: size };
            let total_mem = SIZE { n: (size * NUM_THREADS) };

            println!("READ\t{}\t------\t{} GBps.", mem, bandwidths);
            reads.push((total_mem, bandwidths));
        }
        println!("\n");
        // ------------------- WRITE --------------------------
        for i in 0..UP_TO {
            let multiplier = 2f32.powi(i as i32);
            let size = (START_SIZE as f32 * multiplier) as usize;

            let mut threads = vec![];
            for _ in 0..NUM_THREADS {
                threads.push(thread::spawn(move || do_write(size.clone())))
            }

            let bandwidths: f32 = threads.into_iter()
                                    .map(|t| t.join().unwrap())
                                    .sum();
            
            let mem = SIZE { n: size };
            let total_mem = SIZE { n: (size * NUM_THREADS) };

            println!("WROTE\t{}\t------\t{} GBps.", mem, bandwidths);
            writes.push((total_mem, bandwidths));
        }
        println!("\n");

        // ------------------- READ/WRITE --------------------------
        for i in 0..UP_TO {
            let multiplier = 2f32.powi(i as i32);
            let size = (START_SIZE as f32 * multiplier) as usize;

            let mut threads = vec![];
            for _ in 0..NUM_THREADS {
                threads.push(thread::spawn(move || do_read_write(size.clone())))
            }

            let bandwidths: f32 = threads.into_iter()
                                    .map(|t| t.join().unwrap())
                                    .sum();
            
            let mem = SIZE { n: size };
            let total_mem = SIZE { n: (size * NUM_THREADS) };

            println!("R/WROTE\t{}\t------\t{} GBps.", mem, bandwidths);
            read_writes.push((total_mem, bandwidths));
        }
    }
    println!("\n");

    let mut file = File::create("results.tsv").unwrap();
    writeln!(&mut file, "SIZE\tR\tW\tRW").unwrap();

    for ((r, w), rw) in reads.iter().zip(writes.iter()).zip(read_writes.iter()) {
        println!("{}\tR:{}\tW:{}\tR/W:{}", r.0, r.1, w.1, rw.1);
        writeln!(&mut file, "{}\t{}\t{}\t{}", r.0, r.1, w.1, rw.1).unwrap();
    }    






}
