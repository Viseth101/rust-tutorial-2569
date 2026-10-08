fn main() {
    let menus = vec![
        ("Burger", 150),
        ("French Fries", 80),
        ("Pizza", 250),
        ("Coke", 50),
        ("Steak", 300),
    ];

    let total = menus
        .iter()
        // เลือกเฉพาะเมนูที่ราคาไม่เกิน 200 บาท
        .filter(|(_, price)| *price <= 200)
        // ลดราคา 10%
        .map(|(_, price)| *price - (*price / 10))
        // รวมราคาทั้งหมด
        .fold(0, |acc, price| acc + price);

    println!("================================");
    println!("       FOOD ORDER SUMMARY");
    println!("================================");

    println!("Price Limit : 200 Baht");
    println!("Discount    : 10%");
    println!("--------------------------------");

    for (name, price) in &menus {
        if *price <= 200 {
            let final_price = *price - (*price / 10);

            println!(
                "{} | Original: {} Baht | Final: {} Baht | Selected",
                name, price, final_price
            );
        } else {
            println!("{} | {} Baht | Not Selected", name, price);
        }
    }

    println!("--------------------------------");
    println!("Total Payment: {} Baht", total);
    println!("================================");
}
