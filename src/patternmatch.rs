#![allow(dead_code)]
#![allow(unused_variables)]


fn how_many(x:i32) -> &'static str{

    match x{
        z@ 0 => "no", // a dedicated range
        1 | 2 => "one or two",
        _ => "a few"
    }
}

fn matching(){

    for x in 0..13{
        println!("{}", how_many(x));
    }

    let point = (3,4);

    match point {
        (0, 0) => println!("origin"),
        (0, y) => println!("{}", y),
        (x, 0) => println!("{}", x),
        (x, y) => println!("{}, {}", x, y)
    }
}

fn generics(){

    struct Point<T>{

        x: T,
        y: T
    }

    let a = Point {x: 0, y: 0};

}

fn product(x: i32, y: i32) -> i32 {

    x * y
}

fn functions(){

    let a = 3;
    let b = 8;
    let p = product(a, b);
    println!("{}", p)
}

pub fn main(){

    //matching();
    functions();

}