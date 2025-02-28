// TODO: implement a multi-threaded version of the `sum` function
//  using `spawn` and `join`.
//  Given a vector of integers, split the vector into two halves and
//  sum each half in a separate thread.

// Caveat: We can't test *how* the function is implemented,
// we can only verify that it produces the correct result.
// You _could_ pass this test by just returning `v.iter().sum()`,
// but that would defeat the purpose of the exercise.
//
// Hint: you won't be able to get the spawned threads to _borrow_
// slices of the vector directly. You'll need to allocate new
// vectors for each half of the original vector. We'll see why
// this is necessary in the next exercise.
use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    // Handle edge cases
    if v.is_empty() {
        return 0;
    }
    if v.len() == 1 {
        return v[0];
    }

    // Split the vector into two halves
    let mid = v.len() / 2;
    let left_half = v[0..mid].to_vec();
    let right_half = v[mid..].to_vec();

    // Spawn a thread to sum the left half
    let left_handle = thread::spawn(move || left_half.iter().sum::<i32>());

    // Sum the right half in the current thread
    let right_sum = right_half.iter().sum::<i32>();

    // Wait for the left thread to complete and get its result
    let left_sum = left_handle.join().unwrap();

    // Return the sum of both halves
    left_sum + right_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}
