fn main() {
    let x = 5;
    println!("Hello, world! x is {}", x);
    let mut x = 6;
    println!("x is {}", x);
    x = 7;
    println!("x is {}", x);

    let space = "     ";
    let sl = space.len();
    println!("{}", sl);
    let num: u32 = "42".parse().expect("no a num");
    println!("{:?}", num);

    let a:f32 = 10.0 + 10.0;
    println!("{}", a);

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let _five_hundred = x.0;

    let _six_point_four = x.1;

    let one = x.2;

    println!("{} {} {}", _five_hundred, _six_point_four, one);
}
