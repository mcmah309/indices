use indices::indices;

fn main() {
    let mut data = [5,4,3,2,1];
    let slice = data.as_mut_slice();
    let (_one, _two) = indices!(slice, -1, 1);
}