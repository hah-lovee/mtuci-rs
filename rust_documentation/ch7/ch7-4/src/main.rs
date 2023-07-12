use rand::Rng;
use std::{cmp::Ordering, io::{self, Write}};
// use std::io::{self, Write}
use std::collections::*;


fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
}