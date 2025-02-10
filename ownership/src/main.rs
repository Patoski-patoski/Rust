fn main() {
    let s1 = String::from("hello big fat world");
    let s3 = takes_ownership( s1);

    println!("{s3}");
    
    let x = 5;
    makes_copy(x);
    println!("x is {x}");

    let s4 = String::from("Bye Bye");
    let s5 = calculate_length(&s4);

    println!("The length of '{s4}' is '{s5}'");

    let mut a =  String::from("By Love");
    change(& mut a);


    let f = String::from("Hello");
        let r1 = &f;
        let r2 = &f;

        println!("{r1} {r2}");

        let mut r3  = String::from("Joy is coming");
        let c1 = &r3;
        let c2 = &r3;
        println!("{c1} and {c2}");
        let c3 = &mut r3;
        print!("{c3}\n");

}


fn takes_ownership(some_string: String) -> String {
    let s1 = String::from(some_string);
    return s1;
}

fn calculate_length(s: &String) -> usize {
    return s.len();
}

fn change(some_string: & mut String) {
    some_string.push_str(", world");
}

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}
