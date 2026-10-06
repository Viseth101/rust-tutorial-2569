// Exercise 2 — [Rust_Compro_02]
// Problem

// `[จงสร้าง Vector เก็บชุดตัวเลขจำนวนเต็ม (i32) ขนาด N ตัว (เช่น [1, 2, 3, 4, 5]) จากนั้นให้ทำการตรวจเช็คข้อมูลตัวเลขเดิมใน Vector: ถ้าเจอ เลขคู่ ให้ทำการเพิ่ม (Push) ค่า x *2 ต่อท้ายเข้าไปใน Vector ถ้าเจอ เลขคี่ ให้ทำการเพิ่ม (Push) ค่า x / 2 ต่อท้ายเข้าไปใน Vector เงื่อนไข:ต้องตรวจเช็คครบเฉพาะ N ตัวแรกเท่านั้น

// จากโจทย์ถ้าต้องการให้ Compile ผ่าน ตัวเลือกข้อไหนถูกต้อง

// A. for val in &numbers[..len] { if val % 2 == 0 { numbers.push(val * 2); } else { numbers.push(val / 2); } }

// B. let slice_vals = numbers[..len].to_vec(); for val in slice_vals { if val % 2 == 0 { numbers.push(val * 2); } else { numbers.push(val / 2); } }

// C. for i in 0..numbers.as_slice().len() { let val = &mut numbers[i]; if *val % 2 == 0 { numbers.push(*val * 2); } else { numbers.push(*val / 2); } }

// D. for i in 0..numbers.len() { let val = numbers[i]; if val % 2 == 0 { numbers.push(val * 2); } else { numbers.push(val / 2); } }]`

// Hint

// [ห้ามยืมอ่านข้อมูลค้างไว้ทั้ง Loop แล้วสั่งเพิ่มขนาด Vector พร้อมกัน]

// Solution

fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let len = numbers.len();

    let slice_vals = numbers[..len].to_vec();
    for val in slice_vals {
        if val % 2 == 0 {
            numbers.push(val * 2);
        } else {
            numbers.push(val / 2);
        }
    }

    println!("{:?}", numbers);
}
