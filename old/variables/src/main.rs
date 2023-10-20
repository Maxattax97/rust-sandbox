const MAX_POINTS: u32 = 100_000;

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);

    x = 6;
    println!("The value of x is: {}", x);

    let x = x + MAX_POINTS;
    let x = x * 6;
    println!("The value of x is: {}", x);

    let word = "doggo";
    println!("look, is a {}", word);

    let word = word.len();
    println!("look, was {} chars long", word);

    let _whole_number: i64 = 100_000_000;
    let float_number: f32 = 1.00001;
    let dyna_float = 1.01;
    let to_be_or_not_to_be: bool = true;

    println!(
        "let's add two floats: {} -> {}",
        float_number + dyna_float,
        to_be_or_not_to_be
    );

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("cool chars {} {} {}", c, z, heart_eyed_cat);

    let tuple: (i32, f64, u8, char) = (500, 6.4, 1, 'a');

    println!("toopel {} {} {} {}", tuple.0, tuple.1, tuple.2, tuple.3);

    let months = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];

    let zeroes = [0; 100];

    println!("calendar {}", months[2]);
    println!("zeroes {}", zeroes[99]);

    let _index = 13;
    //println!("dne {}", months[index]);

    another_func(1337, 10_010);
    println!("returned value is {}", express_test());
    discriminate();
    insist();
}

fn another_func(x: i32, y: i64) {
    println!("Jumped! {} {}", x, y);
}

fn express_test() -> i32 {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of x is {} and y is: {}", x, y);

    x
    // x;
}

fn discriminate() {
    let number = 3;

    if number != 0 {
        println!("Was something other than zero.");
    }

    if number % 4 == 0 {
        println!("modulo 4");
    } else if number % 3 == 0 {
        println!("modulo 3");
    }

    let value = if number == 3 {
        5
    } else {
        6
    };

    println!("value {}", value);
}

fn insist() {

    let mut i = 0;
    loop {
        print!("around the world and ");
        i += 1;
        if i > 3 {
            break;
        }
    }
    println!("around the world again");

    i = 0;
    while i <= 3 {
        print!("around the world and ");
        i += 1;
    }
    println!("around the world again");


    let values = [10, 11, 14, 99, 101, 5];
    for elem in values.iter() {
        println!("Now serving... {}", elem);
    }

    for countdown in (1..5 + 1).rev() {
        println!("{} ...", countdown);
    }
    println!("Liftoff!");
}
