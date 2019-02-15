#![allow(dead_code)]
#![allow(unused_variables)]


fn say_hello(){ println!("hello");}

fn stuffs(){

    let sh = say_hello;
    sh();

    let plus_one = |x:i32| -> i32 {x + 1};
    let a = 6;
    println!("{} + 1 = {}", a, plus_one(a));

    let two = 2;
    let plus_two = |x|{

        let mut z = x;
        z += two;
        z
    };
    println!("{} + 2 = {}", 3, plus_two(3));

    let plus_three = |x: &mut i32| * x += 3;
    let mut f= 12;
    plus_three(&mut f);
    println!("{}", f)

}

fn is_even(x: i32) -> bool{

    x % 2 == 0
}

fn hof(){

    let limit = 500;
    let mut sum = 0;

    for i in 0..{

        let isq = i*i;

        if isq > limit {break;}
        else if is_even(isq) {sum += isq; }
    }

    println!("loop sum = {}", sum);

    let sum2 =
        (0..).map(|x| x * x)
            .take_while(|&x| x <= limit)
            .filter(|x| is_even(*x))
            .fold(0, |sum,x| sum + x);

    println!("{}", sum2);
}

pub fn main(){

    hof();
    //stuffs();
}