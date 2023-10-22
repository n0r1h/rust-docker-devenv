struct H2O {}
struct O2 {}
struct H2 {}

fn burn(_h2_1: H2, _h2_2: H2, _o2: O2) -> (H2O, H2O) {
    (H2O {}, H2O {})
}
fn main() {
    let h2_1 = H2 {};
    let h2_2 = H2 {};
    let o2 = O2 {};

    let (h2o_1, h2o_2) = burn(h2_1, h2_2, o2);

    // value used here after move (rustc E0382)
    // let (h2o_1, h2o_2) = burn(h2_1, h2_2, o2);

    let a: i32 = 10;
    let b: &i32 = &a;

    square(b);

    Foo { x: &a };

    #[derive(Debug)]
    struct XY {
        x: Vec<i32>,
        y: Vec<i32>,
    }

    let mut xy = XY {
        x: vec![1, 2, 3],
        y: Vec::new(),
    };
    let XY { x, y } = &mut xy;

    for elm in x {
        y.push(*elm * *elm)
    }
    println!("{:?}", xy);
}

fn square<'a>(x: &'a i32) -> i32 {
    x * x
}

struct Foo<'a> {
    x: &'a i32,
}
