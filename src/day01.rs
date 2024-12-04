use std::fs::File;
use std::io::{self, BufRead};
use std::collections::{BinaryHeap, HashMap};


#[allow(dead_code)]
pub fn part_one() {
    let filepath = "./inputs/day01.txt";
    let lines = io::BufReader::new(File::open(filepath).unwrap()).lines();
    
    let mut heap_a = BinaryHeap::<i32>::new();
    let mut heap_b = BinaryHeap::<i32>::new();
    
    for line in lines.flatten() {
        let numbers = line.split("   ").collect::<Vec<&str>>();
        heap_a.push(numbers[0].parse().unwrap());
        heap_b.push(numbers[1].parse().unwrap());
    }
    
    let mut sum = 0;
    for _ in 0..1000 {
        sum += (heap_a.pop().unwrap() - heap_b.pop().unwrap()).abs();
    }
    
    println!("sum: {}", sum);
}


pub fn part_two() {
    let filepath = "./inputs/day01.txt";
    let lines = io::BufReader::new(File::open(filepath).unwrap()).lines();
    
    let mut similarities = HashMap::<i32, Vec<i32>>::new();
    
    for line in lines.flatten() {
        let numbers = line.split("   ").collect::<Vec<&str>>();

        let number_a = similarities.entry(numbers[0].parse().unwrap()).or_insert(vec![0, 0]);
        number_a[0] += 1;
        let number_b = similarities.entry(numbers[1].parse().unwrap()).or_insert(vec![0, 0]);
        number_b[1] += 1;
    }
    
    let mut similarity = 0;
    for (number, occurences) in similarities.into_iter() {
        let a = occurences[0];
        let b = occurences[1];
        similarity += number * a * b;
    }

    println!("similarity: {}", similarity);
}