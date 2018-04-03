extern crate num;

use self::num::Num;

use std::cmp::PartialOrd;
use std::clone::Clone;
/// Bubble sort
#[allow(unused_assignments, dead_code)]
pub fn bubble_sort<T: Num + PartialOrd + Clone>(arr: &mut Vec<T>) {
	let mut swaping = true;
	let len = arr.len();
	swaping = !(len == 0);
	while swaping {
		swaping = false;
	    for i in 0 .. len - 1 {
	        if arr[i] > arr[i+1] {
	        	let first = arr[i].clone();
	        	let second = arr[i+1].clone();
	        	arr[i] = second;
	        	arr[i+1] = first;
	        	swaping = true;
	        }
	    }
	}
}
/// Insertion sort
#[allow(unused_assignments, dead_code)]
pub fn insertion_sort<T: Num + PartialOrd + Clone>(arr: &mut Vec<T>) {
	 // Указатель в каком месте массива мы сейчас находимся
	let mut pos = 0;
	let len = arr.len();
	// Проход по массиву по указателю 
	for i in 1 .. len {
	   let insert_val = arr[i].clone();
	   pos = i;
	   while pos > 0 && arr[pos-1] > insert_val {
	       arr[pos] = arr[pos-1].clone();
	       pos -= 1;
	   }
	   
	   arr[pos] = insert_val;
	}   
}
