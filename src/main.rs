use std::io;

// define the basic type of goods
enum GOODS {
    ESSENTIAL,
    ORDINARY,
    LUXURY,
}

impl std::fmt::Display for GOODS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GOODS::ESSENTIAL => write!(f, "essential"),
            GOODS::ORDINARY => write!(f, "ordinary"),
            GOODS::LUXURY => write!(f, "luxury"),
        }
    }
}

// define the CONSUMER, with it's id, life and happiness
struct CONSUMER {
    id: u8,
    life: i32,
    happiness: i32,
}


impl CONSUMER {
    fn new(id: u8, life: i32, happiness: i32) -> Self { 
        CONSUMER { id, life, happiness } 
    } 

    fn life(&self) -> i32 { self.life }
    fn happinees(&self) -> i32 { self.happiness }

    fn add_life(&mut self, n: u8) { self.life += n as i32; }
    fn add_happiness(&mut self, n: u8) { self.happiness += n as i32; }
}

// define the SELLER
struct SELLER {
    id: i8,
}

// get the goods an print
fn setGoods() -> (GOODS, i32) {
    let mut price = String::new();
    let mut good = String::new();

    println!("please choose the good you want to sell this term");
    io::stdin()
        .read_line(&mut good)
        .expect("failed to got the price");
    let good: GOODS = match good.trim() {
        "essential" => GOODS::ESSENTIAL,
        "ordinary" => GOODS::ORDINARY,
        "luxury" => GOODS::LUXURY,
        _ => panic!("unknown type!"),
    }; 

    println!("please set the price");
    io::stdin()
        .read_line(&mut price)
        .expect("failed to got the price");
    let price: i32 = match price.trim().parse() {
        Ok(num) => num,
        Err(_) => {println!("please input a number"); 0},
    };

    (good, price)
}

fn buy(good: &GOODS, price: &i32){
    println!("today's good is {}, {}", good, price);
    let mut amount = String::new();
    io::stdin()
        .read_line(&mut amount)
        .expect("failed to get the amount");
    let amount: i32 = match amount.trim().parse() {
        Ok(num) => num,
        Err(_) => {println!("please input a number"); 0},
    };


    
}

fn result(){
    println!("today's result is ..");
}


fn main() {
    while false {
        println!("please input GOODS");
        println!("please input what to buy");
        println!("the result is ...");
    }
    
}

