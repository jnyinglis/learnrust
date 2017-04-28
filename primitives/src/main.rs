fn main() {
    // boolean
    {
        println!("==Booleans==");

        let x = true;

        let y: bool = false;

        println!("x is {} and y is {}", x, y);

        println!("==Booleans==\n");
    }

    // char
    {
        println!("==char==");

        let x = 'x';

        let two_hearts = '💕';

        println!("x is {} and two_hearts is {}", x, two_hearts);

        println!("==char==\n");
    }

    // numeric types
    // i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64
    // signed and unsigned
    {
        println!("==numeric==");

        let _x = 42; // 'x' has type 'i32'.

        let _y = 1.0; // 'y' has type 'f64'.

        println!("==numeric==\n");
    }

    // arrays, they are immutable by default
    {
        println!("==arrays==");

        let _a = [1, 2, 3]; // a: [i32; 3]
        
        let mut _m = [1, 2, 3]; // m: [i32; 3]

        // shorthand initialization

        let _a = [0; 20]; // a: [i32; 20]

        // number of elements in the array

        let a = [1, 2, 3];
        println!("a has {} elements", a.len());

        let names = ["Graydon", "Brian", "Niko"]; // names: [&str; 3]
        println!("The second name is {}", names[1]);

        println!("==arrays==\n");
    }

    // slices
    {
        println!("==slices==\n");

        let a = [0, 1, 2, 3, 4];
        let complete = &a[..];
        let middle = &a[1..4];

        println!("a {}", a.len());
        println!("complete {}", complete.len());
        println!("middle {}", middle.len());

        println!("==slices==\n");
    }

    // tuples
    {
        println!("==tuples==\n");

        let _x = (1, "hello");
        let _x: (i32, &str) = (1, "hello");

        let mut x = (1, 2); // x: (i32, i32)
        let y = (2, 3); // y: (i32, i32)
        x = y;

        let (x, _y, _z) = (1, 2, 3);
        println!("x is {}", x);

        // (0, ); // A single-element tuple
        // (0); // A zero in parenthesis

        let (y,) = (0,);

        println!("y is {}", y);

        println!("==tuples==\n");
    }

    // tuple indexing
    {
        println!("==tuple indexing==\n");
        let tuple = (1, 2, 3);

        let x = tuple.0;
        let y = tuple.1;
        let z = tuple.2;

        println!("z is {}", z);

        println!("==tuple indexing==\n");
    }

    // functions
    {
        println!("==functions==\n");
        fn foo(x: i32) -> i32 {x}

        let x: fn(i32) -> i32 = foo;
        println!("==functions==\n");
    }
}
