fn main() {
    let is_vip: bool = true;
    let total_amount: f64 = 1250.0;

    let discount_rate = if is_vip {
        if total_amount >= 1000.0 {
            0.20
        } else {
            0.10
        }
    } else {
        if total_amount >= 1000.0 {
            0.05 
        } else {
            0.00 
        }
    };

    let final_price = total_amount * (1.0 - discount_rate);
    println!("Total Amount: {:.2} THB", total_amount);
    println!("Discount Rate: {}%", discount_rate * 100.0);
    println!("Final Price: {:.2} THB", final_price);
}
