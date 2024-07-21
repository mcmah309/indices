use indices::try_indices;

fn main() {
    one();
    two();
    three();
    four();
    five();
}

fn one() {
    let mut slice = [0, 1, 2, 3, 4];
    let (four,) = try_indices!(&mut slice, 4).unwrap();
    slice[4] *= 2;
    *four *= 2;
    slice[4] *= 2;
    *four *= 2;
    println!("{four}");
}

fn two() {
    let mut slice = [0, 1, 2, 3, 4];
    let (four, one) = try_indices!(&mut slice, 4, 1).unwrap();
    slice[4] *= 2;
    *four *= 2;
    slice[4] *= 2;
    *four *= 2;
    println!("{four}, {one}");
}

fn three() {
    let mut slice = [0, 1, 2, 3, 4];
    let (four, _one, _two) = try_indices!(&mut slice, 4, 1, 2).unwrap();
    slice[4] *= 2;
    *four *= 2;
    slice[4] *= 2;
    *four *= 2;
    println!("{four}");
}

fn four() {
    let mut slice = [0, 1, 2, 3, 4];
    let (four, one, two, three) = try_indices!(&mut slice, 4, 1, 2, 3).unwrap();
    slice[4] *= 2;
    *four *= 2;
    slice[4] *= 2;
    *four *= 2;
    println!("{four}, {one}, {two}, {three}");
}

fn five() {
    let mut slice = [0, 1, 2, 3, 4];
    let (four, one, two, three, zero) = try_indices!(&mut slice, 4, 1, 2, 3, 0).unwrap();
    slice[4] *= 2;
    *four *= 2;
    slice[4] *= 2;
    *four *= 2;
    println!("{four}, {one}, {two}, {three}, {zero}");
}
