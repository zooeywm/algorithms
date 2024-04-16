/// Given a integer array nums and a integer k ，please calculate and return the count of sub
/// arrays whose sum equals to k
/// Note. The sub arrays are non-empty sequences in the origin array.

// Resolve with double loop.
fn double_loop(origin: &[i32], k: u32) -> i32 {
    // When x, nums of sub_arr.
    let mut n = 0;
    // Each sum of each x
    let mut tmp;

    let mut x_end_index;

    // Min sub_arr length
    for x in 1..origin.len() + 1 {
        // All possible x len
        for i in 0..origin.len() {
            tmp = 0;
            // Current first element index to the x length
            x_end_index = i + x;
            if x_end_index <= origin.len() {
                (i..x_end_index).for_each(|j| {
                    tmp += origin[j];
                    if tmp == k as i32 && j == x_end_index - 1 {
                        n += 1;
                    }
                });
            }
        }
    }
    n
}

fn init() -> (Vec<i32>, u32) {
    (vec![1, 1, 1, 2, 3, 4, 5], 4)
}

#[test]
fn answer1() {
    let (nums, k) = init();
    let r = double_loop(&nums, k);
    assert_eq!(2, r);
}
