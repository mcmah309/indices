use indices::try_indices;

fn main() {
    x();
}

fn x() -> &'static mut i32 {
    let mut data = [5, 4, 3, 2, 1];
    let slice = data.as_mut_slice();
    let (one, _two, _three) = try_indices!(slice, 1, 2, 3).unwrap();
    return one
}