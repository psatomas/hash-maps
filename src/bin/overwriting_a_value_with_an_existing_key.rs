use std::collections::HashMap;

fn main() {
    let mut coffee_pairings: HashMap<&str, &str> = HashMap::new();
    let drink = String::from("Latte");
    let milk = String::from("Oat Milk");
    coffee_pairings.insert(&drink, &milk);
    coffee_pairings.insert("Flat White", "Almond Milk");
    println!("{:?}", coffee_pairings);
    coffee_pairings.insert("Latte", "Pistachio Milk");
    println!("{:?}", coffee_pairings);
}