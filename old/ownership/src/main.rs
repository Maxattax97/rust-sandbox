fn main() {
    ownership();
    ref_and_borrow();
    slice();
}

fn ownership() {
    {
        let mut s = String::from("Hello");
        s.push_str(", world!");
        println!("{}", s);
    }

    {
        let s1 = String::from("hello");
        let s2 = s1;

        // Cannot use s1 because s1 is "borrowed after move."
        println!("{}, world!", s2);

        let mut s3 = s2.clone();

        s3 = s3.replace("e", "u");
        s3 = s3.replace("h", "H");
        println!("{}, wuld!", s3);
    }

    {
        // Copy types.
        let sl1 = "hello";
        let sl2 = sl1;

        let x = 5;
        let y = x;

        println!("sl1 {} sl2 {} x {} y {}", sl1, sl2, x, y);
    }

    //{
    //let dog = String::from("dog");
    //let mut cat = String::from("cat");
    //{
    //let zebra = String::from("zebra");
    //cat = zebra;
    //}

    //println!("{}", cat);
    //}
}

fn ref_and_borrow() {
    let s = String::from("hello");

    let s2 = return_to_sender(s);
    println!("Baby came back! {}", s2);

    enslave(s2);

    // s2 has been lost inside of enslave's scope.
    //println!("Now you don't see me {}", s);

    let x = 5;

    conscript(x);

    println!("You can't hide! {}", x);

    let mut s1 = String::from("whaddup dawg");
    println!("{} is {} chars long", s1, get_str_len(&s1));

    mutate_str_ref(&mut s1);
    println!("Did it change? {} IT DID", &mut s1);
    println!("Did it change? {} IT DID", &s1);
    println!("Did it change? {} IT DID", s1);

    // STOP DATA RACES TODAY!
    let r1 = &s1;
    let r2 = &s1; // &s1;

    //let r3 = &mut s1;

    println!("ABSOLUTELY NOT OKAY {} {}", r1, r2);
    //println!("ABSOLUTELY NOT OKAY {} {} {}", r1, r2, r3);

    println!("magic {}", &mut s1);

    println!("No {} ?", no_dangle());
    // Impossible to contain!
    //tell_it_to_her();
}

fn enslave(some_string: String) {
    println!("You're mine now, {}!", some_string);
}

fn conscript(some_integer: i32) {
    println!("He slipped past us! {}", some_integer);
}

fn return_to_sender(package: String) -> String {
    println!("Send 'er home, {}!", package);
    package
}

fn get_str_len(s: &String) -> usize {
    s.len()
}

fn mutate_str_ref(s: &mut String) {
    s.push_str(" WOAH THERE");
}

fn tell_it_to_her() {
    let pretty_girl = String::from("Michaela");
    let mut mult: u128 = 1;
    loop {
        println!("{} is this cute: {}", pretty_girl, mult);
        mult += 1;
        mult *= mult;
    }
}

fn no_dangle() -> String {
    let s = String::from("dangle");

    s
}

fn slice() {
    let lorem = String::from("Lorem ipsum dolor sit amet");

    let word = first_word(&lorem[..]);
    println!("First word end index {}", word);

    // Causes compile error because requires a mutable reference while an
    // immutable one exists.
    //lorem.clear();
    //println!("First word end index {}", word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
            //&s[..i]
        }
    }

    &s[..]
}
