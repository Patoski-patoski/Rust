fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count: {count}");
    while_loop();
}

fn while_loop(){
    let mut num = 1;

    while num <= 5 {
        println!("Num: {num}");
        num += 1;
    }
    not_for_loop();
}

fn not_for_loop() {
    let a: [i32; 5] = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5{
        println!("the value is {}", a[index]);
        index += 1;
    }

    for_loop();
}

fn for_loop(){
    let a: [i32; 5] = [10, 20, 30, 40, 50];

    for element in a {
        println!("the for value is: {element}");
    }
    to_rev();
}

fn to_rev(){
    for num in (1..4).rev(){
        println!("{num}");
    }
}