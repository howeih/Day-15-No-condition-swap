use std::mem;
fn swap(mut x: i32, mut y: i32) -> (i32, i32) {
    let (u, v) = (x, y);
    let s = (u - v) >> (mem::size_of::<i32>() * 8 - 1);
    x = v * (1 + s) - u * s;
    y = u * (1 + s) - v * s;
    (x, y)
}

fn main() {
    assert_eq!(swap(3, 15), swap(15, 3));
    assert_eq!(swap(-13, 5), swap(5, -13));
}
