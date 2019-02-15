use std::mem;
mod sh;
mod ctrlflw;

const MEANING_OF_LIFE:u8 = 42;

static mut KAKAJUNN:i32 = 123;

fn basics(){
    // unsigned 0 ..255
    let a:u8 = 123; // 8bits
    println!("a= {}", a);

    //a = 456;

    //mut

    let mut b:i8 = 0; //mutable
    println!("b = {}", b);
    b = 42;
    println!("b = {}", b);

    let mut c = 123456789;
    println!("c = {}, size = {} bytes", c, mem::size_of_val(&c));
    c = -1;
    println!("c  after mod = {}", c);

    let z:isize = 123;  //isize/usize

    let size_of_z = mem::size_of_val(&z);
    println!("z = {}, takes up {} bites, {}-bit OS",
    z, size_of_z, size_of_z * 8);

    let d:char = 'x';
    println!("d = {}, size = {} bytes", d, mem::size_of_val(&d));

    let e:f64 = 2.5;
    println!("e = {}, size = {} bytes", e, mem::size_of_val(&e));

    let g = false;
    println!("g = {}, size = {} bytes", g, mem::size_of_val(&g));

    let f = 4> 0; //true

    // arith
    let mut ab = 2+3*2;
    println!("{}", ab);
    ab = ab+1;

    let ab_cubed = i32::pow(ab, 3);
    println!("{} cubed is {}", ab, ab_cubed);

    let bb = 2.4;
    let bb_cubed = f64::powi(bb,3);
    let bb_to_pi = f64::powf(bb, std::f64::consts::PI);
    println!("{} cubed is {} and pied is {}", bb, bb_cubed, bb_to_pi);

    // bitwise | & ! etc

    //logical > < = ==

}

fn main(){
    unsafe {
        println!("{}", KAKAJUNN)
    }

    sh::stack_and_heap();
    ctrlflw::main();
}