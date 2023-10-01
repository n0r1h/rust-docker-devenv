#[derive(Debug)]
enum List<T> {
    Node { data: T, next: Box<List<T>> },
    Nil,
}
fn main() {
    let n1 = List::<u32>::Nil;
    let n2 = List::<u32>::Node {
        data: 10,
        next: Box::<List<u32>>::new(n1),
    };
    let n3 = List::Node {
        data: 40,
        next: Box::new(n2),
    };
    println!("{:?}", n3);
}
