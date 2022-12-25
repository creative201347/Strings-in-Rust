fn main() {
    let mut name = String::new();
    println!("ok {} ok ", name); // ok   ok

    name.push_str("Nabin Dhami");
    println!("{} ", name); // Nabin Dhami

    let new_name = String::from("Nabin Dhami");
    println!("{}", new_name); // Nabin Dhami

    let fname = "Nabin".to_string();
    println!("Length: {}", fname.len());
    println!("Name: {}", fname.to_uppercase());
}
