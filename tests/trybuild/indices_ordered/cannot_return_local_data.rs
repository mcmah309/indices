use indices::indices_ordered;

fn main() {
    one();
    two();
    three();
    four();
    five();
}

fn one() -> &'static mut i32 {
    let mut data = [5, 4, 3, 2, 1];
    let slice = data.as_mut_slice();
    let (four,) = indices_ordered!(slice, 4);
    four
}

fn two() -> (&'static mut i32, &'static mut i32) {
    let mut data = [5, 4, 3, 2, 1];
    let slice = data.as_mut_slice();
    let (one, four) = indices_ordered!(slice, 1, 4);
    (one, four)
}

fn three() -> &'static mut i32 {
    let mut data = [5, 4, 3, 2, 1];
    let slice = data.as_mut_slice();
    let (one, two, four) = indices_ordered!(slice, 1, 2, 4);
    one
}

fn four() -> (&'static mut i32, &'static mut i32, &'static mut i32, &'static mut i32) {
    let mut data = [5, 4, 3, 2, 1];
    let slice = data.as_mut_slice();
    let (one, two, three, four) = indices_ordered!(slice, 1, 2, 3, 4);
    (one, two, three, four)
}

fn five() -> (&'static mut i32, &'static mut i32, &'static mut i32, &'static mut i32, &'static mut i32) {
    let mut data = [5, 4, 3, 2, 1];
    let slice = data.as_mut_slice();
    let (zero, one, two, three, four) = indices_ordered!(slice, 0, 1, 2, 3, 4);
    (zero, one, two, three, four)
}
