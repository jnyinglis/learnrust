fn main() {
    let mut x: i32 = 5;
    let (mut x, y) = (1, 2);
    x = 5;

    let y: i32;
    {
        let z: i32 = 3;
        println!("The value of x is {} and the value of z is {}", x, z);
        let x = 7;
        println!("The value of x is {}", x);
    }

    println!("The value of x is {}", x);
    let x = 42;
    println!("The value of x is {}", x);
//  println!("The value of x is {} and the value of z is {}", x, z);
//  println!("The value of y is :{}", y);

    let mut a: i32 = 7;
    a = 21;
    println!("The value of a is {}", a);
    let a = a;
    println!("The value of a is {}", a);
}
