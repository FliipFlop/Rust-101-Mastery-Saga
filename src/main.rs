fn main() {
    condition_1();
    println!("=====================");
    condition_2();
    println!("=====================");
    loop_1();
    println!("=====================");
    test_borrow();
    println!("=====================");

}

fn condition_1(){
    let weather = String::from("stormy");

    if weather == "sunny" {
        println!("Cross river");
    } else if weather == "rainy" {
        println!("Build a bridge");
    } else {
        println!("Wait for better weather");
    }
}

fn condition_2(){
    let enemy = "slime";
    match enemy {
        "goblin" => println!("Fight goblin"),
        "slime" => println!("Fight slime"),
        "dragon" => println!("Fight dragon"),
        _ => println!("Run away!"),
    }
}

fn loop_1(){
        let mut number = 0;
    loop {        

        number += 1;
        println!("get wood: {}" , number);        

        if number == 10 {
            println!("Finished !!");
            break;
        }
    }

    let wood_spare = sum(number,10);
    let print_msg = get_print_message("WOOD_SPARE", wood_spare);
    println!("{}", print_msg);
}

fn sum(a: i32 , b: i32) -> i32 {
    a + b    
}

fn get_print_message(wood_name: &str , wood_count: i32) -> String {
    format!("{}: {}", wood_name, wood_count)    
}

fn test_borrow(){
    let mut coin: String = String::from("gold coin");

    let friend1 = &coin;
    let friend2 = &coin;

    println!("Friend1 has: {}", friend1);
    println!("Friend2 has: {}", friend2);
    
    let friend3: &mut String = &mut coin;
    friend3.push_str(" and silver coin");
    println!("Friend3 has: {}", friend3);

}