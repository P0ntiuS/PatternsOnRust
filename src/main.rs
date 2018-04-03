extern crate rand;

use rand::{Rng, thread_rng};
mod sorting;

fn main() {
    let mut arr: Vec<i32> = Vec::new();
	fill_arr_random_i32_by_range(&mut arr, 100, -50, 50);
	println!("Before sort: {:?} \n", arr);
	sorting::insertion_sort(&mut arr);
    println!("After sort: {:?}", arr);
}

fn fill_arr_random_i32_by_range(arr: &mut Vec<i32>, len_arr: i32, min: i32, max:i32) {
	let mut rng = thread_rng();

	for _ in 0 .. len_arr {
	    arr.push(rng.gen_range(min, max));
	}
}
