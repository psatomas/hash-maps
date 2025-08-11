use std::collections::HashMap;

fn main() {
    let data = [
        ("Bobby", 7),
        ("Grant", 4),
        ("Ben", 6)
    ];
    let mut years_at_company = HashMap::from(data);
    println!("{:?}", years_at_company);

    let ben = years_at_company.remove("Ben");
    println!("{:?}", ben);
    println!("{}", ben.unwrap());
    println!("{:?}", years_at_company);

    let ben = years_at_company.remove("Ben");
    println!("{:?}", ben);
}