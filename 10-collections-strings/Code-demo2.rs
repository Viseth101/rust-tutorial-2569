// EXAM DEMO 2: ระบบจัดการสต๊อกร้านสะดวกซื้อ

use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone)]
struct Product {
    name: String,
    category: String,
    stock: u32,
    reorder_level: u32,
}

// TRAIT: implement Display เอง เพื่อกำหนดว่า struct นี้ print เป็นข้อความยังไง
// -> จุดเด่นของ Rust: trait แยก behavior ออกจาก data, เพิ่ม trait ทีหลังได้โดยไม่แก้ struct เดิม
impl fmt::Display for Product {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:<15} ({:<10}) คงเหลือ {} ชิ้น",
            self.name, self.category, self.stock
        )
    }
}

// PATTERN MATCHING แบบ "guard" (match ... if ...): จุดเด่นที่ Exam 1 ยังไม่ได้โชว์
// -> เทียบเงื่อนไขแบบไดนามิก (reorder_level ไม่ใช่ค่าคงที่) ซึ่ง match ปกติทำไม่ได้
fn stock_status(stock: u32, reorder_level: u32) -> &'static str {
    match stock {
        0 => "หมดสต๊อก",
        s if s <= reorder_level => "ใกล้หมด ต้องสั่งเพิ่ม",
        _ => "ปกติ",
    }
}

// TUPLE: แยกชื่อสินค้ากับหมวดหมู่ออกจาก 1 บรรทัด คืนค่าเป็น Tuple
fn split_name_category<'a>(name: &'a str, category: &'a str) -> (&'a str, &'a str) {
    (name, category)
}

fn main() {
    // 1) ARRAY: หมวดหมู่สินค้าที่ร้านรับเข้าสต๊อก
    let allowed_categories: [&str; 4] = ["เครื่องดื่ม", "ขนม", "ของใช้", "อาหารแห้ง"];
    println!("=== Array Demo ===");
    println!("หมวดหมู่ที่รับ: {}\n", allowed_categories.join(", "));

    // 2) &str + PARSING ด้วย match (Result) -> Vec<Product>
    let raw_stock: &str = "\
น้ำดื่ม,เครื่องดื่ม,120,30
มาม่า,อาหารแห้ง,8,20
ทิชชู่,ของใช้,45,15
ขนมปัง,ขนม,0,10
กาแฟกระป๋อง,เครื่องดื่ม,60,25
สบู่,ของใช้,5,10
ปลากระป๋อง,อาหารแห้ง,300,40";

    let mut inventory: Vec<Product> = Vec::new();

    for line in raw_stock.lines() {
        let fields: Vec<&str> = line.split(',').collect();
        if fields.len() != 4 {
            continue;
        }

        let (name, category) = split_name_category(fields[0].trim(), fields[1].trim());

        if !allowed_categories.contains(&category) {
            eprintln!("ข้ามบรรทัด (หมวดหมู่ไม่รู้จัก): {}", line);
            continue;
        }

        // PATTERN MATCHING: จัดการทั้ง Ok และ Err ตรงนี้ ไม่มี exception หลุดลอย
        let stock: u32 = match fields[2].trim().parse() {
            Ok(v) => v,
            Err(_) => {
                eprintln!("ข้ามบรรทัด (จำนวนสต๊อกไม่ใช่ตัวเลข): {}", line);
                continue;
            }
        };

        let reorder_level: u32 = match fields[3].trim().parse() {
            Ok(v) => v,
            Err(_) => {
                eprintln!("ข้ามบรรทัด (reorder level ไม่ใช่ตัวเลข): {}", line);
                continue;
            }
        };

        inventory.push(Product {
            name: name.to_string(), // &str -> String เพราะต้องเป็นเจ้าของข้อมูลเอง
            category: category.to_string(),
            stock,
            reorder_level,
        });
    }

    // 3) HashMap: รวมยอดสต๊อกตามหมวดหมู่ (การจัดการข้อมูลหลายค่า)
    let mut stock_by_category: HashMap<String, u32> = HashMap::new();
    for p in &inventory {
        *stock_by_category.entry(p.category.clone()).or_insert(0) += p.stock;
    }

    println!("\n=== รวมสต๊อกตามหมวดหมู่ ===");
    for cat in allowed_categories.iter() {
        let total = stock_by_category.get(*cat).copied().unwrap_or(0);
        println!("{:<12} | รวม {} ชิ้น", cat, total);
    }

    // 4) match guard: เช็คสถานะสต๊อกของแต่ละสินค้า + Display trait
    println!("\n=== สถานะสต๊อกรายสินค้า ===");
    for p in &inventory {
        println!("{} -> {}", p, stock_status(p.stock, p.reorder_level));
    }

    // 5) Closure: จัดอันดับสินค้าที่สต๊อกเหลือน้อยที่สุด (เร่งด่วนสั่งซื้อ)
    let mut urgency_ranking = inventory.clone();
    urgency_ranking.sort_unstable_by(|a, b| a.stock.cmp(&b.stock)); // closure เทียบสต๊อก น้อย -> มาก

    println!("\n=== อันดับสินค้าที่ต้องสั่งด่วนที่สุด ===");
    for (i, p) in urgency_ranking.iter().take(3).enumerate() {
        println!("{}. {} (เหลือ {} ชิ้น)", i + 1, p.name, p.stock);
    }

    // 6) Iterator chain + String: สรุปรายชื่อสินค้าที่ต้องสั่งเพิ่ม
    let need_restock: Vec<String> = inventory
        .iter()
        .filter(|p| p.stock <= p.reorder_level)
        .map(|p| p.name.clone())
        .collect();

    println!("\nสินค้าที่ต้องสั่งเพิ่ม: {}", need_restock.join(", "));
}