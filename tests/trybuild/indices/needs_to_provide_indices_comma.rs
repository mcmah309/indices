use indices::indices;

fn main() {
    let mut data: [i32; 0] = [];
    let slice = data.as_mut_slice();
    let result = indices!(slice,);
}