// EXAM 1 - DEMO 1: &str ดิบ -> Collection ที่มีโครงสร้าง

#[derive(Debug, Clone)]
struct Student {
    name: String,
    score: u32,
    subject: String,
}

// SLICE: รับได้ทั้ง Array และ Vector เพราะทั้งคู่ coerce เป็น &[u32] ได้
// คืนค่าเป็น Tuple: วิธีมาตรฐานของ Rust ในการ return หลายค่าพร้อมกัน
fn min_max(scores: &[u32]) -> (u32, u32) {
    let min = *scores.iter().min().unwrap();
    let max = *scores.iter().max().unwrap();
    (min, max)
}

// OWNERSHIP: ฟังก์ชันนี้ "ยืม" (borrow) ข้อมูล ไม่ได้เอาไปเป็นเจ้าของ
fn describe(name: &str) {
    println!("นักเรียนคนนี้ชื่อ: {}", name);
}

// OWNERSHIP: ฟังก์ชันนี้ "เอาไปเป็นเจ้าของ" (take ownership) แล้วคืนค่าใหม่กลับมา
fn shout(name: String) -> String {
    name.to_uppercase()
}

fn main() {
    // [1] ARRAY — ขนาดคงที่ กำหนดไว้ตั้งแต่ compile-time
    // ใช้ min_max() ที่รับ &[u32] ได้ เพราะ Array coerce เป็น slice อัตโนมัติ
    println!("[1] Array: {:?}", [80u32, 75, 90, 60, 88]);
    let quiz_scores: [u32; 5] = [80, 75, 90, 60, 88];
    let (q_min, q_max) = min_max(&quiz_scores);
    println!("    min = {}, max = {}\n", q_min, q_max);

    // [2] TUPLE — รวมค่าคนละชนิดไว้ด้วยกันโดยไม่ต้องสร้าง struct
    // เข้าถึงทีละตัวด้วย .0 .1 .2 หรือ destructure ด้วย let (a, b, c) = ...
    let student_record: (&str, u32, &str) = ("Somchai", 85, "Math");
    let (name, score, subject) = student_record;
    println!("[2] Tuple: {:?} -> name={}, score={}, subject={}\n", student_record, name, score, subject);

    // [3] VECTOR — ขนาดยืดหยุ่น เพิ่ม/ลบ/ค้นข้อมูลระหว่างรันได้ (Array ทำไม่ได้)
    let mut makeup_scores: Vec<u32> = Vec::new();
    makeup_scores.push(70);
    makeup_scores.push(82);
    makeup_scores.push(95);
    println!("[3] Vector หลัง push: {:?}", makeup_scores);

    makeup_scores.insert(0, 100); // แทรกที่ตำแหน่งแรก
    makeup_scores.remove(1); // เอาตัวที่ index 1 ออก
    let count_above_80 = makeup_scores.iter().filter(|&&s| s >= 80).count();
    println!("    หลัง insert/remove: {:?} (>= 80 จำนวน {} ตัว)\n", makeup_scores, count_above_80);

    // [4] STRING — เจ้าของข้อมูลตัวอักษรที่แก้ไข/ขยายได้ ต่างจาก &str
    // ต่อข้อความด้วย push_str / += ได้ เพราะเป็นเจ้าของ heap memory เอง
    let mut full_report = String::new();
    full_report.push_str("รายงาน: ");
    full_report.push_str("Somchai ");
    full_report += &format!("ได้ {} คะแนน", score);
    println!("[4] String: \"{}\"\n", full_report);

    // Ownership & Borrowing: describe() ยืมข้อมูล -> ใช้ต่อได้, shout() ยึด ownership -> ใช้ต่อไม่ได้
    let owned_name = String::from("Wichai");
    describe(&owned_name); // ยืมชั่วคราว
    let shouted = shout(owned_name); // ยก ownership ให้ฟังก์ชัน
    // println!("{}", owned_name); // <- เปิดบรรทัดนี้แล้ว compile error ทันที: value moved
    println!("    หลัง shout(): {}\n", shouted);

    // [5] &str — ตัวอ้างอิงข้อความแบบ read-only ที่ยืมมาจากที่อื่น ไม่ copy ข้อมูล
    let raw_data: &str =
        "Somchai,85,Math\nSomsri,90,Science\nWichai,78,Math\nNarin,95,Science\nPim,60,English";
    println!("[5] &str slice 7 ตัวแรก: \"{}\"\n", &raw_data[0..7]);

    // รวมทุกหัวข้อ: PARSING ด้วย match (Result) + Iterator chain -> Vec<Student>
    let allowed_subjects: [&str; 3] = ["Math", "Science", "English"]; // [1] Array ใช้เป็น whitelist
    let mut records: Vec<Student> = Vec::new(); // [3] Vector เก็บผลลัพธ์

    for line in raw_data.lines() {
        let fields: Vec<&str> = line.split(',').collect(); // [5] &str split
        if fields.len() != 3 {
            continue;
        }

        let name = fields[0].trim();
        let subject = fields[2].trim();

        // PATTERN MATCHING: Rust บังคับจัดการทั้ง Ok และ Err ตรงนี้ ไม่มี exception หลุดลอย
        let score = match fields[1].trim().parse::<u32>() {
            Ok(v) => v,
            Err(_) => {
                eprintln!("ข้ามบรรทัด (คะแนนไม่ใช่ตัวเลข): {}", line);
                continue;
            }
        };

        if allowed_subjects.contains(&subject) {
            records.push(Student {
                name: name.to_string(), // [4] &str -> String เพราะต้องเป็นเจ้าของข้อมูลเอง
                score,
                subject: subject.to_string(),
            });
        }
    }

    println!("=== Parse &str -> Vec<Student> สำเร็จ {} รายการ ===", records.len());
    for s in &records {
        println!("{:?}", s); // derive(Debug) -> print struct ได้เลยไม่ต้องเขียน toString เอง
    }
}