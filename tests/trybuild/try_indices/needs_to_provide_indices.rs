use indices::try_indices;

fn main() {
    let mut data: [i32; 0] = [];
    let slice = data.as_mut_slice();
    let result = try_indices!(slice);
}