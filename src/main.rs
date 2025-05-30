struct Crabby {
    name: String,
    health: u8,
}

impl Crabby {    
    fn print_hp(&self) {
        println!("{} HP: {}", self.name , self.health);
    }

    fn healing(&mut self, hp: u8){
        if self.health + hp > 100 {
            self.health = 100;
        } else {
            self.health += hp;
        }            
        println!("{} healed for {} HP", self.name, hp);
        self.print_hp();
    }
    
    fn take_damage(&mut self, damage: u8){
        if self.health > damage {
            self.health -= damage;
            println!("{} took {} damage", self.name, damage);
        } else {
            self.health = 0;
            println!("{} has been defeated", self.name);
        }

        self.print_hp();
    }
}

fn main() {
    condition_1();
    println!("=====================");
    condition_2();
    println!("=====================");
    loop_1();
    println!("=====================");
    test_borrow();
    println!("=====================");
    lifetime_example();
    println!("=====================");
    test_crabby();
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

fn lifetime_example(){
    let map1: &str = "map1 map1";
    let map2: &str = "map2 map2 map2";

    let longest = longest_map(map1, map2);
    println!("The longest map is: {}", longest);
}

fn longest_map<'a>(map1: &'a str, map2: &'a str) -> &'a str {
    if map1.len() > map2.len() {
        map1
    } else {
        map2
    }
}


fn test_crabby(){
    let mut crabby = Crabby {
        name: String::from("Crabby"),
        health: 70,
    };

    crabby.healing(20);
    crabby.healing(30);
    crabby.take_damage(30);
    crabby.take_damage(2);
}


















