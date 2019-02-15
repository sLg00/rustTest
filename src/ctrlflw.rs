#![allow(dead_code)]
#![allow(unused_variables)]

fn ifstatement(){

    let temp = 15;

    if temp > 30{
        println!("hawt");
    }

    else if temp < 10{
        print!("coldlol");
    }

    else {
        println!("ok");
    }

    let day = if temp > 20{"sunny!"} else{"cloudy"};
    //if statement can be expressed in the print macro as well
    println!("today is {}", day);
}

fn while_loop (){

    let mut x = 1;

    while x < 1000{
        x = x + 100;

        if x == 801 {continue; }
        println!("the value is {}", x)
    }

    let mut y = 1;
    loop {

        y = y + 100;
        println!("the valuea is {}", y);

        if y == 901 {break; }
    }
}

fn for_loop(){

    for x in 1..11{
        if x == 8 {break; }
        println!("x is equal to {}", x);
    }

    for (pos, y) in (30..41).enumerate(){
        println!("{} : {}", pos, y);
    }
}

fn match_statement(){

    let country_code = 58; // 1...999

    let country = match country_code{
        44 => "UK",
        46 => "Sweden",
        372 => "Estonia",
        1...999  => "Unknown", // .. vs ... - second includes last value in range
        _ => "invalid"
    };

    println!("the country with code {} is {}", country_code, country);

}

pub fn main(){
    //ifstatement();
    //while_loop();
    //for_loop();
    //match_statement();
}