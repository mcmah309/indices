use indices::try_indices_ordered;

fn main() {
    let mut data = [5,4,3,2,1];
    let slice = data.as_mut_slice();
    let result = try_indices_ordered!(slice, -1, 1);
}