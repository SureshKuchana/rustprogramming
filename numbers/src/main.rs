// const cannot be mutated & explict type annotation must be declared unlike the let variable
// value of the const must be constant expression (computted at complie time)
// can declared in any scope
// don't occupy any space
const _MAX_PLAYERS: u8 = 10;

// static variables, explicit type annotation must be added
// can be marked as mut, but it is unsafe rust
// can declared in any scope
// do occupy space
static _RUST_NAME: &str = "Rust";

fn main() {
    let x = 1.1;
    let y: f64 = 2.2;
    let answer = multiply_both(x, y);

    // shadowing the c variable
    let _c: i32 = 10;
    let _c: i32 = 20;

    // outer scope
    let _d: i32 = 50;
    // inner scope
    {
        // inner scope variables can't be access outside of the inner scope
        let d: i32 = 40;
        print!("inner d is : {d}");
    }

    let vec: Vec<i32> = vec![];

    let mut b = vec;
    b.push(1);

    numbers::say_hello();
    println!("Hello, world!");
    println!(" x times y is = {}", x * y);
    println!("using the function to execute x * y = {}", answer);

    let x64 : f64 = 10.0 / 3.0;
    println!(" x64 {}", x64); // 3.3333333333333335
    let y32 : f32 = 10.0 / 3.0;
    println!(" y32 {}", y32); // 3.3333333
    println!(" 10/3 {}", 10 / 3); // 3.3333333

    // tuple is collection of two or more values
    let point: (i64, i64, i64) = (1,2,3);

    let (x1, y1, z1) = point; // destructure
    // let (x1, y1, _) = point; ==> if you don't want the z1
    // let (x1, _, _) = point; ==> if you don't want the y1 & z1
    // or
    // let p1 = point.0;
    // let p2 = point.1;
    // let p3 = point.2;
    println!(" x1, y1, z1, {}, {}, {}", x1, y1, z1);
    // get all the tuple
    println!(" get x , {} ", get_x_tup(point));
    println!(" get y , {} ", get_y_tup(point));
    println!(" get z , {} ", get_z_tup(point));

    // modify tuple value
    let point1: (i64, i64, i64) = (10, 20, 30);
    set_x_tup(point1, 100);
    println!(" get x , {} ", get_x_tup(point1));

    let t1: (i32, i32, i32) = (1, 2, 3);
    let _t2: (i32, f64, &str) = (5, 5.0, "5");

    let _s1 = t1.2;    

    // Unit (empty tuple)
    let _unit : () = ();

    let _println_return_val : () = println!("hi");

    // type aliasing
    type Age = u8;

    let _age1: Age = 57;

    // Structs
    struct Point {
        s1: i64, _s2: i64, _s3: i64
    }

    let point1 = Point {s1: 14, _s2: 15, _s3: 16};
    let Point { s1, .. } = point1;

    println!(" get s1 {} ", s1);

    // array 
    let years : [i32; 3] = [1999, 2000, 2001];
    let _a1: i32 = years[1];
    // iterate over array
    for year in years.iter() {
        println!(" Year {} ", year);
    }

    // if - else

    let test = 5;
    if test > 5 {
        println!(" bigger than 5")
    } else if test > 3 {
        println!(" bigger than 3")
    } else {
        println!(" smaller or equal to 3")
    }

    let _bar1 = if test > 5 { 1 } else { - 1};

    // loop
    // loop {
    //     print!(" loop forever");
    // }

    // loop
    // loop {
    //     print!(" loop break");
    //     break;
    // }

    let mut bar = 0;
    // while loop
    while bar < 5 {
        print!(" bar is {bar}");
        bar += 1;
    }

    let arr = [1, 2, 3, 4, 5];
    for ele in arr {
        println!("{} ", ele)
    }

    enum Color {
        Red,
        Green,
        Blue,
    }

    let color_matching = Color::Green;

    match color_matching {
        Color::Blue => println!("Matching the color BLUE"),
        Color::Red => println!("Matching the color RED"),
        Color::Green => println!("Matching the color GREEN")
    }

    // divide
    let res_divide = divide(10, 2);

    match res_divide {
        Ok(value) => println!("Matching value {}", value),
        Err(error) => println!("Error {}", error),
    }

}

fn multiply_both(x : f64, y: f64) -> f64 {
    x * y
}

fn get_x_tup(my_point: (i64,i64,i64)) -> i64 {
    my_point.0
}

fn get_y_tup(my_point: (i64,i64,i64)) -> i64 {
    my_point.1
}

fn get_z_tup(my_point: (i64,i64,i64)) -> i64 {
    my_point.2
}

fn set_x_tup(mut my_point: (i64,i64,i64), x: i64){
    my_point.0 = x;
}

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Division by zero!"))
    } else {
        Ok(a / b)
    }
}