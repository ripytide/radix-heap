#[macro_use]
extern crate criterion;

extern crate rand;
extern crate radix_heap;

use std::collections::BinaryHeap;
use criterion::{Criterion, Bencher, black_box};
use radix_heap::{RadixHeapMap, Radix};
use rand::{thread_rng, Rng, Rand};

fn extend_radix<T: Copy + Ord + Radix + Rand>(b: &mut Bencher) {
    let data: Vec<T> = thread_rng().gen_iter().take(10000).collect();
    let mut heap = RadixHeapMap::new();
    
    b.iter(|| {
        heap.extend(data.iter().map(|&k| (k,())));
        
        while let Some(a) = heap.pop() {
            black_box(a);
        }
        
        heap.clear();
    });
}

fn extend_binary<T: Copy + Ord + Radix + Rand>(b: &mut Bencher) {
    let data: Vec<T> = thread_rng().gen_iter().take(10000).collect();
    let mut heap = BinaryHeap::<T>::new();
    
    b.iter(|| {
        heap.extend(data.iter());
        
        while let Some(a) = heap.pop() {
            black_box(a);
        }
        
        heap.clear();
    });
}

fn pushpop_radix(b: &mut Bencher) {
    let mut heap = RadixHeapMap::<i32, ()>::new();
    
    b.iter(|| {
        heap.push(0, ());
        
        for _ in 0..10000 {
            let (n,_) = heap.pop().unwrap();
            
            for i in 0..4 {
                heap.push(n - i, ());
            }
        }
        
        heap.clear();
    });
}

fn pushpop_binary(b: &mut Bencher) {
    let mut heap = BinaryHeap::<i32>::new();
    
    b.iter(|| {
        heap.push(0);
        
        for _ in 0..10000 {
            let n = heap.pop().unwrap();
            
            for i in 0..4 {
                heap.push(n - i);
            }
        }
        
        heap.clear();
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("extend_radix 8", extend_radix::<u8>);
    c.bench_function("extend_radix 16", extend_radix::<u16>);
    c.bench_function("extend_radix 32", extend_radix::<u32>);
    c.bench_function("extend_binary 8", extend_binary::<u8>);
    c.bench_function("extend_binary 16", extend_binary::<u16>);
    c.bench_function("extend_binary 32", extend_binary::<u32>);
    c.bench_function("pushpop_radix", pushpop_radix);
    c.bench_function("pushpop_binary", pushpop_binary);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);