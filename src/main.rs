use unique_iterator_rs::IteratorExt;

fn main() {
    let numbers = vec![1, 2, 3, 4, 4, 6, 2];
    let unique_numbers: Vec<_> = numbers.into_iter().unique().collect();
    println!("{:?}", unique_numbers);
}
