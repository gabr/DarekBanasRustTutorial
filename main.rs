use std::{i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64};
// use std::io::stdin;

fn main() {
    print!("Hello World");

    let num = 10;
    let mut age: i32 = 40;

    println!("Range i8    {} - {}",     i8::MIN,    i8::MAX);
    println!("Range i16   {} - {}",    i16::MIN,   i16::MAX);
    println!("Range i32   {} - {}",    i32::MIN,   i32::MAX);
    println!("Range i64   {} - {}",    i64::MIN,   i64::MAX);
    println!("Range u8    {} - {}",     u8::MIN,    u8::MAX);
    println!("Range u16   {} - {}",    u16::MIN,   u16::MAX);
    println!("Range u32   {} - {}",    u32::MIN,   u32::MAX);
    println!("Range u64   {} - {}",    u64::MIN,   u64::MAX);
    println!("Range isize {} - {}",  isize::MIN, isize::MAX);
    println!("Range usize {} - {}",  usize::MIN, usize::MAX);
    println!("Range f32   {} - {}",    f32::MIN,   f32::MAX);
    println!("Range f64   {} - {}",    f64::MIN,   f64::MAX);

    let is_it_true: bool = true;
    let char_x: char = 'x';
    let char_y = 'y';

    let (f_name, l_name) = ("Gabr", "P");

    println!("It is {0} that {1} is {0}",
        is_it_true, char_x);

    println!("{:.2}", 1.23456);
    println!("{0:b}b 0x{0:x} 0{0:o}", 10);

    println!("{ten:>ws$}", ten=10, ws=5);
    println!("{ten:>0ws$}", ten=10, ws=5);

    println!("5 + 4 = {}", 5 + 4);
    println!("5 - 4 = {}", 5 - 4);
    println!("5 * 4 = {}", 5 * 4);
    println!("5 / 4 = {}", 5 / 4);
    println!("5 % 4 = {}", 5 % 4);

    let mut neg_4 = -4i32;
    println!("abs(-4) = {}", neg_4.abs());
    println!("-4 ^ 6 = {}", -4i32.pow(6));
    println!("sqlrt 9 = {}", 9f32.sqrt());
    println!("cbrt 27 = {}", 27f64.cbrt());
    println!("Round 1.45 = {}", 1.45f32.round());
    println!("Floor 1.45 = {}", 1.45f32.floor());
    println!("Ceiling 1.45 = {}", 1.45f32.ceil());
    println!("e ^ 2 = {}", 2f32.exp());
    println!("log(2) = {}", 2f32.ln());
    println!("llog10(2) = {}", 2f32.log10());
    println!("90 to Radians = {}", 90f64.to_radians());
    println!("PI to Degrees = {}", f32::consts::PI.to_degrees());
    println!("3.14 to Degrees = {}", 3.14f32.to_degrees());
    println!("Max 4, 5 = {}", 4f64.max(5f64));
    println!("Min 4, 5 = {}", 4f64.min(5f64));

    println!("Sin 3.14 = {}", 3.14f32.sin());
    // ...

    // != == > < >= <=
    // && || !
    let age_old = 6;
    if age_old == 5 {
        println!("Go to Kindergarten");
    } else if (age_old > 5) && (age_old <= 18) {
        println!("Go to grade {}", age_old - 5);
    } else {
        println!("Do what you want");
    }

    println!("!true = {}", !true);
    // ...

    let can_vote = if age_old >= 18 { true } else { false };
    println!("Can Vote: {}", can_vote);


    let mut x = 1;
    loop {
        if x % 2 == 0 {
            println!("LOOP: {}", x);
        }

        if x > 10 {
            break;
        }

        x += 1;
    }

    let mut y = 1;
    while y <= 10 {
        println!("WHILE: {}", y);
        y += 2;
    }

    // range
    for z in 1..10 {
        println!("FOR: {}", z);
    }



    let rand_string = "I am a random string";
    println!("Length: {}", rand_string.len());
    let (first, second) = rand_string.split_at(6);
    println!("first: {}, Second: {}", first, second);

    let count = rand_string.chars().count();
    let mut chars = rand_string.chars();
    let mut indiv_char = chars.next();

    loop {
        match indiv_char {
            Some(x) => println!("{}", x),
            None => break,
        }
        indiv_char = chars.next();
    }

    let iter = rand_string.split_whitespace();
    for w in iter {
        println!("{}", w);
    }

    let multiline = "I am a random string\nThere are other strings like it\nThis string is the best";
    for line in multiline.lines() {
        println!("{}", line);
    }

    println!("Find 'best': {}", multiline.contains("best"));

    let rand_array = [1, 2, 3, 4];
    println!("{}", rand_array[0]);
    println!("len {}", rand_array.len());
    println!("Slice: {:?}", &rand_array[1..3]);

    let mut vector = vec![1, 2, 3, 4, 5];
    println!("Item 2: {}", vector[1]);

    println!("-----------");
    for i in &vector {
        println!("Vect: {}", i);
    }

    vector.push(6);
    println!("-----------");
    for i in &vector {
        println!("Vect: {}", i);
    }

    vector.pop();
    println!("-----------");
    for i in &vector {
        println!("Vect: {}", i);
    }
    println!("-----------");


    let rand_tuple = ("g", 40);
    let rand_tuple_2: (&str, i8) = ("q", 30);
    println!("t1: {:?}", rand_tuple);
    println!("t2: {:?}", rand_tuple_2);

    fn say_hello(name: &str) {
        println!("Hello {}", name);
    }

    fn get_sum(a: i32, b: i32) -> i32 {
        a + b
    }

    say_hello("Kaa");
    println!("{}", get_sum(5, 4));

    let sum = get_sum;
    println!("{}", sum(5, 4));


    // closures
    //             parameters
    let sum_nums = |x: i32, y: i32| x + y;
    println!("7 + 8 = {}", sum_nums(7, 8));

    let num_ten = 10;
    let add_10 = |x: i32| x + num_ten;
    println!("2 + 10 = {}", add_10(2));


    let vect1 = vec![1, 2, 3];
    let vect2 = vect1;
    //println!("vect1[0]: {}", vect1[0]); // <-- use of moved value error

    let prim_val1 = 1;
    let primval2 = prim_val1;
    println!("prim_val1: {}", prim_val1);

    fn sum_vects(vector: &Vec<i32>) -> i32 {
        //let mut sum = 0;
        //for v in vector {
        //    sum += v;
        //}
        //return sum;

        // equivalent of this ^^^ loop
        vector
            // change vector to iterator
            .iter()
            // sum all elements
            .fold(0, |sum, &v| sum + v)
    }

    println!("vector sum: {}", sum_vects(&vect2));


    //#[derive(Debug)]
    struct Circle {
        x: f64,
        y: f64,
        r: f64
    }

    // not recomended
    fn get_circle_area(circle: &Circle) -> f64 {
        f64::consts::PI * circle.r.powi(2)
    }

    // recomended
    impl Circle {
        /* pub */ fn get_area(&self) -> f64 {
            f64::consts::PI * self.r.powi(2)
        }

    }

    /*
        In order to use {:?} in println() the struct
        need to satisfy Debug triat.

        In Most cases just:
            #[derive(Debug)]
        will be enough.

        But it can be done manually:
     */
    impl std::fmt::Debug for Circle {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "Circle {}x{} radius: {}", self.x, self.y, self.r)
        }
    }

    let c1 = Circle {
        x: 0.0, y: 0.0, r: 2.0
    };

    println!("Circle ({:?}) area: {}", c1, get_circle_area(&c1));
    println!("Circle ({:?}) area: {}", c1, c1.get_area());

    #[derive(Debug)]
    struct Rectangle {
        h: f64,
        w: f64,
    }

    // interface
    trait HasArea {
        fn get_area(&self) -> f64;
    }

    impl HasArea for Rectangle {
        fn get_area(&self) -> f64 {
            self.w * self.h
        }
    }

    let r1 = Rectangle {
        h: 10.0,
        w: 2.5,
    };

    println!("Rectangle ({:?}) area is: {}", r1, r1.get_area());

    // enums
    enum Hero {
        Fast,
        Strong(i32),
        Info {name: String, secret: String}
    }

    fn get_info(h: Hero) {
        match h{
            Hero::Fast => println!("Fast"),
            Hero::Strong(i) => println!("Lifts {tons}", tons = i),
            Hero::Info {name, secret} => println!("{} ({})", name, secret),
        }
    }

    let hulk = Hero::Strong(100);
    let flash = Hero::Fast;
    let ironMan = Hero::Info {
        name: "IronMan".to_owned(),
        secret: "Tony Stark".to_owned()
    };

    get_info(hulk);
    get_info(flash);
    get_info(ironMan);
}

