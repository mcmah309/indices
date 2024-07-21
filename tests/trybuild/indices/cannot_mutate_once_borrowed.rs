use indices::indices;

fn main() {
    one();
    two();
    three();
    four();
    five();
}

fn one() {
    let mut slice = [0, 1, 2, 3, 4];
    let (four,) = indices!(&mut slice, 4);
    slice[4] *= 2;
    *four *= 2;
    slice[4] *= 2;
    *four *= 2;
    println!("{four}");
}

fn two() {
    let mut slice = [0, 1, 2, 3, 4];
    let (four, one) = indices!(&mut slice, 4, 1);
    slice[4] *= 2;
    *four *= 2;
    slice[4] *= 2;
    *four *= 2;
    println!("{four}, {one}");
}

fn three() {
    let mut slice = [0, 1, 2, 3, 4];
    let (four, _one, _two) = indices!(&mut slice, 4, 1, 2);
    slice[4] *= 2;
    *four *= 2;
    slice[4] *= 2;
    *four *= 2;
    println!("{four}");
}

fn four() {
    let mut slice = [0, 1, 2, 3, 4];
    let (four, one, two, three) = indices!(&mut slice, 4, 1, 2, 3);
    slice[4] *= 2;
    *four *= 2;
    slice[4] *= 2;
    *four *= 2;
    println!("{four}, {one}, {two}, {three}");
}

fn five() {
    let mut slice = [0, 1, 2, 3, 4];
    let (four, one, two, three, zero) = indices!(&mut slice, 4, 1, 2, 3, 0);
    slice[4] *= 2;
    *four *= 2;
    slice[4] *= 2;
    *four *= 2;
    println!("{four}, {one}, {two}, {three}, {zero}");
}
