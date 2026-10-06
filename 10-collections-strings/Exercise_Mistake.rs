// ## 7. Common Mistakes

// ### Mistake 1 — `การลืมเรื่อง Ownership`

// **Problem**

// `การที่คุณส่งข้อมูล string หรือ vec เข้าไปในฟังก์ชั่นโดยไม่ใช้ "&"`

//**Incorrect Code**

//fn print_message(msg: String) {
//  println!("2: {}", msg);
//}
//fn main() {
//   let my_str = String::from("Hello Rust");
//  print_message(my_str);

// println!("Message: {}", my_str);
//}


//**Correct Code**

fn print_message(msg: &String) {
    println!("Message: {}", msg);
}

fn main() {
    let my_str = String::from("Hello Rust");
    print_message(&my_str);

    println!("Message: {}", my_str);
}


//### Mistake 2 — `การพยายามเพิ่ม/ลบ ข้อมูลใน Collection ขณะกำลังวนลูปอ่าน`

//**Problem**

//`ในภาษาอื่นคุณอาจจะวนลูป Array แล้วใช้ push() เข้าไปตรงๆได้เลย แต่ใน Rust จะไม่อนุญาต`

//**Incorrect Code**

//fn main() {
//    let mut my_vec = vec![1, 2, 3];
//
//
//    for item in &my_vec {
//        if *item == 2 {
//            //Error : Rust ไม่ยอมให้เขียนข้อมูลตอนที่กำลังมีคนยืมอ่านอยู่
//            my_vec.push(4);
//        }
//    }
//}

//**Correct Code**

fn main() {
    let mut my_vec = vec![1, 2, 3];
    let mut to_add = Vec::new();

    for item in &my_vec {
        if *item == 2 {
            to_add.push(4);
        }
    }

    my_vec.append(&mut to_add);

    println!("{:?}", my_vec); // ผลลัพธ์: [1, 2, 3, 4]
}


//## 8. Exercises

//### Exercise 1 — `นับสต็อกสินค้า`

//**Problem**

//`คุณได้รับตะกร้าสินค้าที่มีของปะปนกันอยู่ ซึ่งข้อมูลมาในรูปแบบ Vec<String> ที่เก็บชื่อสินค้าเรียงต่อกันไปเรื่อยๆ (มีชื่อสินค้าซ้ำกัน) ให้เขียนฟังก์ชัน count_items เพื่อสรุปว่าเรามีสินค้าแต่ละชนิดอย่างละกี่ชิ้น โดยแปลงให้ออกมาอยู่ในรูปแบบ HashMap<String, u32>`

//**Solution**

use std::collections::HashMap;

fn count_items(items: Vec<String>) -> HashMap<String, u32> {
   let mut item_counts: HashMap<String, u32> = HashMap::new();

    for item in items {
        // ใช้ .entry() เพื่อเข้าถึงจำนวนของสินค้านั้น
        // ถ้าไม่เคยมีสินค้านี้ ให้ใส่ 0 เป็นค่าเริ่มต้นด้วย .or_insert(0)
        // จากนั้นใช้ * เพื่อนำค่าที่ได้มาบวกเพิ่ม 1
        let count = item_counts.entry(item).or_insert(0);
        *count += 1;

        // หมายเหตุ: สามารถเขียนแบบย่อบรรทัดเดียวได้แบบนี้:
        // *item_counts.entry(item).or_insert(0) += 1;
    }

    item_counts
}

fn main() {
    let basket = vec![
        String::from("Apple"),
        String::from("Banana"),
        String::from("Apple"),
        String::from("Orange"),
        String::from("Banana"),
        String::from("Apple"),
    ];

    let result = count_items(basket);
   println!("{:#?}", result);
}


//### Exercise 2 — `ระบบดึงแฮชแท็กแบบไม่ซ้ำ`

//**Problem**

//`คุณได้รับข้อความโพสต์จากโซเชียลมีเดียที่มีแฮชแท็กปะปนอยู่ ให้เขียนฟังก์ชัน extract_hashtags ที่รับข้อความ (&str) แล้วดึงเอาเฉพาะคำที่เป็นแฮชแท็ก (คำที่ขึ้นต้นด้วย #) ออกมาโดย 1.ตัดเครื่องหมาย # ด้านหน้าออกไป เอาแค่ชื่อแท็ก 2.ถ้ามีแฮชแท็กซ้ำกัน ให้เก็บไว้แค่ชื่อเดียว 3.ส่งคืนผลลัพธ์เป็น HashSet<String>`

//**Solution**

use std::collections::HashSet;

fn extract_hashtags(text: &str) -> HashSet<String> {
    let mut tags = HashSet::new();

    // 1. แยกข้อความเป็นคำๆ และวนลูป
    for word in text.split_whitespace() {
        // 2. เช็คว่าขึ้นต้นด้วย #
        if word.starts_with('#') {
            // 3. ตัด # ตัวแรกออก (ตำแหน่งที่ 0) แล้วแปลงเป็น String
            let tag_name = word[1..].to_string();

            // 4. โยนเข้า HashSet (ถ้าซ้ำ มันจะจัดการเพิกเฉยให้เอง)
            tags.insert(tag_name);
        }
    }

    tags
}

fn main() {
    let post = "I love #rust and #coding so much! #rust is the best #programming language.";
    let result = extract_hashtags(post);
    println!("{:#?}", result);
}
