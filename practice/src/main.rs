fn main() {
    let fib =  fibonacci(40);
    println!("{:#?}", fib);

}


// 0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144
fn fibonacci (v:i32) -> i32 {
    let mut a: i32 = 0;
    let mut b: i32 = 1;
    let mut c: i32 = 0;
    let mut temp: i32 = 3;

    if v == 1 {
        return 0;
    }
    if v == 2 {
        return 1;
    }

    while temp <= v {
        c = a + b;
        a = b;
        b = c;
        temp += 1;
    }
    return c;
}

