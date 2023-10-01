#[derive(Debug)]
enum Storage {
    HDD { size: u32, rpm: u32 },
    SSD(u32),
}

fn main() {
    let s = Storage::HDD {
        size: 2048,
        rpm: 7200,
    };
    println!("{:?}", s);
    println!("{:#?}", s);
}
