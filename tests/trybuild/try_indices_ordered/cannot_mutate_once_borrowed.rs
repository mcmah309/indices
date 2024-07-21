use indices::try_indices_ordered;

fn main() {
    let mut slice = [0, 1, 2, 3, 4];
    let (four, _one, _two) = try_indices_ordered!(&mut slice, 1, 2, 3).unwrap();
    slice[4] *= 2;
    *four *= 2;
    slice[4] *= 2;
    *four *= 2;
    println!("{four}");
}