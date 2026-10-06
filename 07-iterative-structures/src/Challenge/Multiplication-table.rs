fn main() {
    let mut count = 0;
    let mut sum = 0;

    'outer: for i in 1..=9 {
        for j in 1..=9 {
            let product = i * j;

            if product > 50 {
                break 'outer; 
            }
            if product % 2 != 0 {
                continue; 
            }

            println!("{} x {} = {}", i, j, product);
            count += 1;
            sum += product;
        }
    }

    println!("พิมพ์ทั้งหมด {} ค่า, ผลรวม = {}", count, sum);
}