#![allow(dead_code)]
#![allow(unused_variables)]

use std::mem;

fn structs(){

    struct Point
    {
        x: f64,
        y: f64
    }

    struct Line
    {
        start: Point,
        end: Point
    }

    let p = Point {x: 3.0, y: 4.0};
    println!("point p is at ({}, {})", p.x, p.y);

    let p2 = Point{x: 5.0, y: 7.0};
    println!("point p2 is at ({}, {}", p2.x, p2.y);

    let myline = Line{start: p, end: p2};

}

fn enums(){

    enum Color{
        red,
        green,
        blue,
        Rgbcolor(u8,u8,u8), //tuple
        Cmyk{cyan:u8, magenta:u8, yellow:u8, black:u8}, //struct
    }

    let c:Color = Color::Cmyk{cyan:0, magenta:128, yellow:0,black:255};

    match c
        {
            Color::red => println!("r"),
            Color::green => println!("g"),
            Color::blue => println!("b"),
            Color::Rgbcolor(0,0,0)
            | Color::Cmyk{cyan:_, magenta:_, yellow:_, black:255} => println!("black"),
            Color::Rgbcolor(r,g,b) => println!("rgb({},{},{})", r, g, b),
            _ => ()
        }
}

union IntOrFloat{
    i: i32,
    f: f32
}

fn process_value(iof: IntOrFloat){
    unsafe {
        match iof{
            IntOrFloat {i: 42} => { println!("meaning of life");}
            IntOrFloat { f } => {println!("f32 = {}", f);}
        }
    }
}

fn unions(){

    let mut iof = IntOrFloat{i: 123 };

    unsafe {iof.i = 42;}

    let value = unsafe {iof.i };

    process_value(iof);
    process_value(IntOrFloat{f:1.23});
}

fn options(){
    let x = 3.0;
    let y = 0.0;

    let result:Option<f64> =
        if y != 0.0 {Some(x/y)} else { None };

    println!("{:?}", result);

    match result{
        Some(z) => println!("{}/{} = {}",x,y,z),
        None => println!("cannot divide {} by {}",x ,y)
    }

    // if let / while let

    if let Some(z) = result { eprintln!("z = {}", z);}

}

fn arrays(){

    let mut a:[i32;5] = [1,2, 3, 4, 5];

    println!("a has {} elements, first is {}", a.len(), a[0]);

    a[0] = 321;
    println!("a at 0 is equal to {}", a[0]);
    println!("{:?}", a);

    if a != [1,2,3,4,5]{
        println!("does not match");
    }

    let b = [1;10];

    for i in 0..b.len(){
        println!("{}", b[i]);
    }

    println!("b took up {} bytes", mem::size_of_val(&b));

let mtx:[[f32;3];2] = //array of arrays. first is columns, second is rows
    [
        [1.0, 0.0, 0.0],
        [0.0, 2.0, 0.0]
    ];

    println!("{:?}", mtx);

    for i in 0..mtx.len()
        {
            for j in 0..mtx[i].len()
                {if i == j
                {
                    println!("mtx[{}][{}] = {}", i, j, mtx[i][j]);
                }
                }
        }
}

fn vectors(){

    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);

    println!("{:?}", a);

    a.push(44);

    let idx = 0;

    a[idx] = 321;
    println!("a[0] = {}", a[idx]);

    //Option
    match a.get(3) {
        Some(x) => println!("a[3] = {:?}", x),
        None => println!("No such element")
    }

    for x in &a{
        println!("{}", x);
    }

    a.push(52);
    println!("{:?}", a);

    //Some
    let last_elem = a.pop(); // Option
    println!("last elem is {:?}, a = {:?}", last_elem, a);

    while let Some(x) = a.pop(){
        println!("{}", x)
    }

}

fn use_slice(slice: &mut[i32]){

    println!("first = {}, len = {}", slice[0], slice.len());
    slice[0] = 4321;
}

fn slices(){

    let mut data = [1,2,3,4,5];

    use_slice(&mut data[1..4]);

    //use_slice(&mut data);

    println!("{:?}", data);
}

fn strings(){

    // utf-8
    let s:&'static str = "hello"; // &str = string slice

    for c in s.chars().rev(){
        println!("{}", c);
    }

    if let Some(first_character) = s.chars().nth(0){
        println!("first is {}", first_character)
    }

    //heap
    //String
    let mut letters = String::new();
    let mut a = 'a' as u8;
    while a <= ('z' as u8){
        letters.push(a as char);
        letters.push_str(",");
        a +=1;
    }
    println!("{}", letters);

    // &str <> String

    let u:&str = &letters;

    //concat
    let z = letters + "abc";

    //let z = letters + &letters;

    let mut abc: String = String::from("hello");
    let mut abc = "hello".to_string();
    abc.remove(0);
    abc.push_str("!!1");
    let mut a = abc.replace("ello", "bye");

    println!("{}", a);

}

fn sum_and_product(x:i32, y:i32) -> (i32, i32){

    (x+y, x*y)
}

fn tuples(){

    let x = 3;
    let y = 4;
    let sp = sum_and_product(x,y);

    println!("{:?}", sp);

    //destructure
    let (a,b) = sp;
    let sp2 = sum_and_product(6,5);
    let combined = (sp,sp2);
    let ((c,d),(e.f)) = combined;
}

pub fn main(){

    //structs();
    //enums();
    //unions();
    //options();
    //arrays();
    //vectors();
    //slices();
    //strings();
    //tuples();
}