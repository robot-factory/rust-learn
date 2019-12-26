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

    let a: [i32;5] = [1,2,3,4,5];
    println!("{}",a[1]);
    let _a = [5;5];
    let _index = 10;
    // println!("{}",a[index]);
    let b = get_num();
    println!("{}", b);

    let a = 10;
    if a> 5 {
        println!("a > 5");
    } else if a>3 {
        println!("a > 3");
    }

    let b = if a > 5 {
        "a > 5"
    } else {
        "a <=5"
    };
    println!("{}", b);

    let mut a = 0;
    loop {
        a = a + 1;
        println!("now is in loop {}", a);
        if a >100 {
            break;
        }
    };
    println!("{}",a);
    let mut b = 0;
    let loop_num = loop {
        b = b + 1;
        if b *20 %13 == 5 {
            break b * 16 %23;
        }
    };
    println!("{}",loop_num);

    while b <1000 {
        b = b * 10;
        println!("{}", b);
    }

    for i in [1,2,4].iter() {
        println!("{}", i);
    }

    for i in 1..4 {
        println!("{}",i);
    }
}

fn get_num() -> u32 {
    let x = 4;
    x+1
}
