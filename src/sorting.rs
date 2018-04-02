extern crate num;

use self::num::Num;

use std::cmp::PartialOrd;
use std::clone::Clone;
/// Bubble sorting
#[allow(unused_assignments)]
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