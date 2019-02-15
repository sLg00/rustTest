#![allow(dead_code)]
#![allow(unused_variables)]

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

}

pub fn main(){

    //structs();
    //enums();
    //unions();
    //options();
    arrays();
}