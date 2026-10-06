# Rust Tutorial Project — Principles of Programming Languages

> **กลุ่มที่:** 10
> **Topic No.:** 10
> **Topic Name:** Collections & Strings
> **ประเด็นหลักที่ควรครอบคลุม:** Array, Tuple, Vector, String, &str, การจัดการข้อมูลหลายค่า

---

## 1. Members

| # | Name | Student ID | GitHub Username | Main Responsibility |
|---|---|---|---|---|
| 1 | นางสาวชัญญา เถระสวัสดิ์ | 670710146 | `therasawat_c@silpakorn.edu` | Concept + Short Code Illustration (สรุปแนวคิดหลัก + โค้ดตัวอย่างสั้น) |
| 2 | นายภัทรกฤต สังครบ | 670710148 | `sungkrob_p@silpakorn.edu` | Detailed Code + Live Demo (โค้ดเชิงลึก + สาธิตสด) |
| 3 | นายศุภณัฐ วิริยะนรอนันต์ | 670710149 | `wiriyanorraanun_s@silpakorn.edu` | Rust vs Other Language + PPL Analysis (เปรียบเทียบภาษา + วิเคราะห์เชิง PPL) |
| 4 | นายสิรวิชญ์ ปิ่นแสง | 670710150 | `pinsang_s@silpakorn.edu` | Exercises + Common Mistakes + Challenge (แบบฝึกหัด + ข้อผิดพลาดที่พบบ่อย + คำถามท้าทาย) |

> แก้ไข GitHub Username ของแต่ละคนให้ตรงกับบัญชีจริงก่อนเริ่มทำงาน (ผู้สอนจะใช้คอลัมน์นี้เชิญเป็น collaborator ของ repository)

---

## 2. Learning Objectives

หลังจากศึกษา Topic นี้แล้ว ผู้เรียนสามารถ:

1. `[อธิบายแนวคิดสำคัญได้]`
2. `[เขียนโปรแกรม Rust ที่เกี่ยวข้องได้]`
3. `[วิเคราะห์พฤติกรรม/กฎของภาษาได้]`
4. `[เปรียบเทียบ Rust กับภาษาอื่นได้]`

---

---

## 3. Introduction

Collection & String เป็นหัวข้อในภาษา Rust ที่เกี่ยวกับการเก็บและจัดการข้อมูลหลายค่า รวมถึงข้อมูลที่เป็นข้อความ โดยในหัวข้อนี้จะมีชนิดข้อมูลและโครงสร้างที่เกี่ยวข้อง เช่น Array, Tuple, Vector, String และ &str ซึ่งแต่ละแบบจะมีลักษณะและวิธีการใช้งานที่ต่างกัน

หัวข้อนี้มีความสำคัญ เพราะในการเขียนโปรแกรมเรามักต้องเก็บข้อมูลหลายค่าไว้ด้วยกัน เช่น คะแนน รายชื่อ หรือข้อมูลที่เกี่ยวข้องกัน ถ้าเก็บข้อมูลแต่ละค่าแยกเป็นตัวแปรจะทำให้จัดการข้อมูลได้ยากและไม่เป็นระเบียบ จึงต้องมีโครงสร้างที่ช่วยเก็บข้อมูลหลายค่าไว้ด้วยกัน

---

## 4. Key Concepts

### 4.1 Array

Array คือการเก็บข้อมูลหลายค่าไว้ในตัวแปรเดียว โดยข้อมูลทุกตัวต้องเป็นชนิดเดียวกัน และจำนวนข้อมูลต้องกำหนดไว้ตั้งแต่ตอนสร้าง

รูปแบบของ Array คือ

```text
[T; N]
```

โดย T คือชนิดข้อมูล และ N คือจำนวนสมาชิก

ตัวอย่าง

```rust
fn main() {
    let scores: [i32; 3] = [80, 90, 75];

    println!("{}", scores[0]);
}
```

Explanation

`[i32; 3]` หมายถึง Array ที่เก็บข้อมูลชนิด `i32` จำนวน 3 ค่า โดยสามารถเรียกดูข้อมูลแต่ละตัวได้จาก Index ซึ่งเริ่มจาก 0 เช่น `scores[0]` คือข้อมูลตัวแรก ซึ่งมีค่า 80

จำนวนสมาชิกของ Array จะเพิ่มหรือลดไม่ได้ แต่สามารถเปลี่ยนค่าของข้อมูลได้ถ้าประกาศตัวแปรด้วย `mut`

```rust
fn main() {
    let mut scores = [80, 90, 75];

    scores[0] = 85;

    println!("{}", scores[0]);
}
```

ผลลัพธ์คือ

```text
85
```

Array จึงเหมาะกับข้อมูลที่รู้จำนวนแน่นอน เช่น คะแนนสอบ 3 ครั้ง


### 4.2 Tuple

Tuple คือการนำข้อมูลหลายค่ามาเก็บไว้ในตัวแปรเดียว โดยข้อมูลแต่ละตัวสามารถเป็นคนละชนิดกันได้ และจำนวนข้อมูลจะกำหนดไว้ตั้งแต่ตอนสร้าง

ตัวอย่าง

```rust
fn main() {
    let student = ("Alice", 20, true);

    println!("{}", student.0);
}
```

Explanation

ตัวแปร `student` เป็น Tuple ที่เก็บข้อมูล 3 อย่าง ได้แก่ ข้อความ จำนวนเต็ม และค่า Boolean โดยสามารถเรียกข้อมูลแต่ละตัวได้จากตำแหน่ง เช่น `student.0`, `student.1` และ `student.2`

Tuple เหมาะกับกรณีที่ต้องการเก็บข้อมูลหลายอย่างที่เกี่ยวข้องกันไว้ในตัวแปรเดียว

ความแตกต่างระหว่าง Array กับ Tuple คือ Array ต้องเก็บข้อมูลชนิดเดียวกัน แต่ Tuple สามารถเก็บข้อมูลต่างชนิดกันได้


### 4.3 Vector

Vector หรือ `Vec<T>` ใช้สำหรับเก็บข้อมูลหลายค่าเหมือนกับ Array แต่สามารถเพิ่มหรือลดจำนวนข้อมูลได้ในระหว่างที่โปรแกรมทำงาน

รูปแบบคือ

```text
Vec<T>
```

โดย T คือชนิดข้อมูลที่ต้องการเก็บ

ตัวอย่าง

```rust
fn main() {
    let mut scores = vec![80, 90, 75];

    scores.push(85);

    println!("{:?}", scores);
}
```

Explanation

`vec![80, 90, 75]` ใช้สำหรับสร้าง Vector ที่มีข้อมูล 3 ค่า และ `push()` ใช้สำหรับเพิ่มข้อมูลเข้าไป ทำให้ได้ผลลัพธ์เป็น

```text
[80, 90, 75, 85]
```

นอกจากนี้ Vector สามารถใช้ Index เพื่อเข้าถึงข้อมูล และสามารถใช้ `len()` เพื่อดูจำนวนข้อมูล รวมถึงใช้ `pop()` เพื่อลบข้อมูลตัวสุดท้ายได้

```rust
scores.pop();
```

Vector จึงเหมาะกับข้อมูลที่จำนวนสามารถเปลี่ยนแปลงได้ เช่น รายการสินค้า หรือรายชื่อผู้ใช้

ความแตกต่างระหว่าง Array และ Vector คือ

```text
Array  → จำนวนข้อมูลคงที่
Vector → จำนวนข้อมูลเพิ่มหรือลดได้
```


### 4.4 String

String ใช้สำหรับเก็บข้อมูลที่เป็นข้อความ และสามารถเพิ่มหรือแก้ไขข้อความได้ในระหว่างที่โปรแกรมทำงาน

ตัวอย่าง

```rust
fn main() {
    let mut text = String::from("Hello");

    text.push_str(" Rust");

    println!("{}", text);
}
```

Explanation

`String::from()` ใช้สร้างข้อมูลชนิด `String` และ `push_str()` ใช้เพิ่มข้อความต่อท้าย String

ผลลัพธ์คือ

```text
Hello Rust
```

ถ้าต้องการเพิ่มตัวอักษรทีละ 1 ตัว สามารถใช้ `push()`

```rust
text.push('!');
```

สรุปการใช้งานคือ

```text
push()     → เพิ่มตัวอักษร 1 ตัว
push_str() → เพิ่มข้อความ
```

String รองรับข้อความแบบ UTF-8 จึงสามารถใช้เก็บข้อความหลายภาษาได้ เช่น ภาษาไทยและภาษาอังกฤษ


### 4.5 &str

`&str` เป็นชนิดข้อมูลที่ใช้สำหรับการอ้างอิงข้อมูลข้อความ และมักพบในข้อความที่เขียนไว้โดยตรง หรือที่เรียกว่า String Literal

ตัวอย่าง

```rust
fn main() {
    let name: &str = "Alice";

    println!("{}", name);
}
```

ในตัวอย่าง `"Alice"` เป็น String Literal และตัวแปร `name` มีชนิดเป็น `&str`

สามารถเขียนโดยไม่ระบุชนิดข้อมูลได้เช่นกัน

```rust
let message = "Hello Rust";
```

Rust สามารถรู้ได้เองว่า `message` เป็น `&str`

Explanation

`String` และ `&str` ใช้กับข้อความเหมือนกัน แต่มีลักษณะการใช้งานต่างกัน

```text
&str   → ใช้สำหรับการอ้างอิงข้อมูลข้อความที่มีอยู่แล้ว
String → ใช้สำหรับข้อความที่ต้องการสร้างหรือแก้ไข
```

ตัวอย่างการใช้ `&str` ร่วมกับ `String`

```rust
fn main() {
    let mut message = String::from("Hello");

    message.push_str(" Rust");

    println!("{}", message);
}
```

ข้อความ `" Rust"` ที่นำไปใช้กับ `push_str()` เป็น `&str` และสามารถใช้ร่วมกับ `String` ได้


### 4.6 การจัดการข้อมูลหลายค่า

การจัดการข้อมูลหลายค่าคือการนำข้อมูลที่เกี่ยวข้องกันมาเก็บไว้ด้วยกัน เพื่อให้จัดการข้อมูลได้ง่ายขึ้น

ตัวอย่าง ถ้าเก็บคะแนนแต่ละตัวแยกกัน

```rust
let score1 = 80;
let score2 = 90;
let score3 = 75;
```

ถ้ามีข้อมูลจำนวนมาก การเก็บแบบนี้จะทำให้ต้องสร้างตัวแปรหลายตัว จึงสามารถนำข้อมูลมารวมกันเป็น Array ได้

```rust
let scores = [80, 90, 75];
```

ถ้าจำนวนข้อมูลสามารถเพิ่มหรือลดได้ สามารถใช้ Vector

```rust
let mut scores = vec![80, 90, 75];
scores.push(85);
```

ถ้าต้องการเก็บข้อมูลหลายชนิดที่เกี่ยวข้องกัน สามารถใช้ Tuple

```rust
let student = ("Alice", 20, true);
```

ดังนั้นการเลือกใช้ Array, Tuple, Vector, String หรือ `&str` ควรดูจากลักษณะของข้อมูลที่ต้องการเก็บและการใช้งานของโปรแกรม เพื่อให้สามารถจัดการข้อมูลได้ง่ายและเหมาะสมกับงาน

---

## 5. Important Syntax / Rules

| **Syntax / Rule** | **Meaning** | **Example** |
| ----------------- | ------------ | ------------ |
| `[Type; จำนวน]` | ใช้สร้าง Array โดยกำหนดชนิดข้อมูลและจำนวนสมาชิก | `let scores: [i32; 3] = [80, 90, 75];` |
| `(ค่า1, ค่า2, ...)` | ใช้สร้าง Tuple เพื่อเก็บข้อมูลหลายค่าไว้ด้วยกัน และแต่ละค่าเป็นคนละชนิดกันได้ | `let student = ("Alice", 20, true);` |
| `vec![...]` | ใช้สร้าง Vector สำหรับเก็บข้อมูลหลายค่า ซึ่งสามารถเพิ่มหรือลดจำนวนสมาชิกได้ | `let mut scores = vec![80, 90, 75];` |
| `String::from("ข้อความ")` | ใช้สร้าง String จากข้อความที่กำหนด | `let text = String::from("Hello");` |
| `&str` | ใช้สำหรับการอ้างอิงข้อมูลข้อความ และมักใช้กับ String Literal | `let name: &str = "Alice";` |
| `.push(ค่า)` | ใช้เพิ่มข้อมูล 1 ค่าเข้าไปใน Vector หรือเพิ่มตัวอักษร 1 ตัวใน String | `scores.push(85);` |
| `.pop()` | ใช้ลบข้อมูลตัวสุดท้ายออกจาก Vector | `scores.pop();` |
| `.push_str("ข้อความ")` | ใช้เพิ่มข้อความต่อท้าย String | `text.push_str(" Rust");` |

### Important Rules

1. **Array** ต้องมีจำนวนสมาชิกคงที่ และสมาชิกทุกตัวต้องเป็นชนิดข้อมูลเดียวกัน

2. **Tuple** สามารถเก็บข้อมูลหลายชนิดไว้ด้วยกันได้ แต่จำนวนสมาชิกจะคงที่

3. **Vector** สามารถเพิ่มหรือลดจำนวนสมาชิกได้ และสมาชิกภายในต้องเป็นชนิดข้อมูลเดียวกัน

4. **String** ใช้สำหรับเก็บข้อความที่สามารถเพิ่มหรือแก้ไขได้ 

5. **&str** ใช้สำหรับอ้างอิงข้อมูลข้อความ เช่น String Literal

---

## 6. Runnable Code Examples

> **ข้อกำหนด:** Code ทุกตัวต้อง Compile และ Run ได้จริงก่อนนำมาใส่ในเอกสาร

### Example 1 — `[การจัดการข้อมูลนักเรียนด้วย Collection และ String]`

**Purpose:** `Purpose
แสดงการใช้งาน Array, Tuple, Vec, String และ &str สำหรับจัดการข้อมูลนักเรียนหลายค่า`

```rust
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
```

**Expected Output**

```
[1] Array: [80, 75, 90, 60, 88]
    min = 60, max = 90

[2] Tuple: ("Somchai", 85, "Math") -> name=Somchai, score=85, subject=Math

[3] Vector หลัง push: [70, 82, 95]
    หลัง insert/remove: [100, 82, 95] (>= 80 จำนวน 3 ตัว)

[4] String: "รายงาน: Somchai ได้ 85 คะแนน"

นักเรียนคนนี้ชื่อ: Wichai
    หลัง shout(): WICHAI

[5] &str slice 7 ตัวแรก: "Somchai"

=== Parse &str -> Vec<Student> สำเร็จ 5 รายการ ===
Student { name: "Somchai", score: 85, subject: "Math" }
Student { name: "Somsri", score: 90, subject: "Science" }
Student { name: "Wichai", score: 78, subject: "Math" }
Student { name: "Narin", score: 95, subject: "Science" }
Student { name: "Pim", score: 60, subject: "English" }
```

**Explanation**

`[

1. **`[u32; 5]` Array**  
   → เก็บคะแนนที่มีจำนวนข้อมูลแน่นอน และสามารถส่งเข้า `min_max()` ในรูปแบบ Slice ได้

2. **`(&str, u32, &str)` Tuple**  
   → เก็บข้อมูลนักเรียนหลายค่าที่มีชนิดข้อมูลต่างกัน เช่น ชื่อ คะแนน และวิชา

3. **`Vec<u32>`**  
   → เก็บคะแนนที่สามารถเพิ่มหรือลบข้อมูลระหว่างโปรแกรมทำงานได้

4. **`String`**  
   → เก็บข้อความที่เป็นเจ้าของข้อมูลเอง และสามารถเพิ่มหรือแก้ไขข้อความได้

5. **`&str`**  
   → ใช้อ้างอิงข้อความโดยไม่ต้องเป็นเจ้าของข้อมูล เหมาะกับการอ่านหรือยืมข้อมูล

6. **`min_max()`**  
   → รับข้อมูลแบบ Slice แล้วใช้ `iter()` เพื่อหาค่าคะแนนต่ำสุดและสูงสุด พร้อมคืนค่าเป็น Tuple

7. **`describe()`**  
   → รับ `&str` เพื่อยืมข้อมูล ทำให้ข้อมูล `String` เดิมยังสามารถนำไปใช้ต่อได้

8. **`shout()`**  
   → รับ `String` และเป็นเจ้าของข้อมูล ทำให้ตัวแปรเดิมไม่สามารถใช้ต่อหลังจากถูก Move

9. **`split(',')` + `collect()`**  
   → แยกข้อมูลนักเรียนจากข้อความ `&str` แล้วเก็บแต่ละส่วนไว้ใน `Vec<&str>`

10. **`parse()` + `match`**  
    → แปลงคะแนนจากข้อความเป็น `u32` และตรวจสอบกรณีที่ข้อมูลไม่ใช่ตัวเลข

11. **`Vec<Student>`**  
     → เก็บข้อมูลนักเรียนที่แปลงจากข้อความดิบให้เป็นข้อมูลที่มีโครงสร้าง
    ]`

---

### Example 2 — `[ระบบจัดการสต๊อกร้านสะดวกซื้อ]`

**Purpose:** `แสดงการใช้ Collection ในการจัดการข้อมูลสินค้า โดยใช้ Vec, HashMap และ Iterator รวมถึง Trait, Closure และ Pattern Matching`

```rust
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

```

**Expected Output**

```
=== Array Demo ===
หมวดหมู่ที่รับ: เครื่องดื่ม, ขนม, ของใช้, อาหารแห้ง


=== รวมสต๊อกตามหมวดหมู่ ===
เครื่องดื่ม  | รวม 180 ชิ้น
ขนม          | รวม 0 ชิ้น
ของใช้       | รวม 50 ชิ้น
อาหารแห้ง    | รวม 308 ชิ้น

=== สถานะสต๊อกรายสินค้า ===
น้ำดื่ม         (เครื่องดื่ม) คงเหลือ 120 ชิ้น -> ปกติ
มาม่า           (อาหารแห้ง ) คงเหลือ 8 ชิ้น -> ใกล้หมด ต้องสั่งเพิ่ม
ทิชชู่          (ของใช้    ) คงเหลือ 45 ชิ้น -> ปกติ
ขนมปัง          (ขนม       ) คงเหลือ 0 ชิ้น -> หมดสต๊อก
กาแฟกระป๋อง     (เครื่องดื่ม) คงเหลือ 60 ชิ้น -> ปกติ
สบู่            (ของใช้    ) คงเหลือ 5 ชิ้น -> ใกล้หมด ต้องสั่งเพิ่ม
ปลากระป๋อง      (อาหารแห้ง ) คงเหลือ 300 ชิ้น -> ปกติ

=== อันดับสินค้าที่ต้องสั่งด่วนที่สุด ===
1. ขนมปัง (เหลือ 0 ชิ้น)
2. สบู่ (เหลือ 5 ชิ้น)
3. มาม่า (เหลือ 8 ชิ้น)

สินค้าที่ต้องสั่งเพิ่ม: มาม่า, ขนมปัง, สบู่
```

**Explanation**

`[

1. **`Vec<Product>`**  
   → เก็บข้อมูลสินค้าหลายรายการ โดยสามารถเพิ่มสินค้าเข้าไปใน Collection ได้

2. **`HashMap<String, u32>`**  
   → เก็บยอดสต๊อกรวมแยกตามหมวดหมู่ ทำให้ค้นหาข้อมูลตามชื่อหมวดหมู่ได้ง่าย

3. **`Array` `allowed_categories`**  
   → เก็บรายชื่อหมวดหมู่ที่ร้านอนุญาตให้รับเข้าสต๊อก

4. **`split(',')` + `collect()`**  
   → แยกข้อมูลสินค้าแต่ละบรรทัดออกเป็นชื่อ หมวดหมู่ จำนวนสต๊อก และระดับที่ต้องสั่งเพิ่ม

5. **`match` + `parse()`**  
   → แปลงข้อมูลสต๊อกจากข้อความเป็นตัวเลข และจัดการกรณีข้อมูลไม่ถูกต้อง

6. **`entry().or_insert()`**  
   → ใช้สร้างหรือเพิ่มยอดสต๊อกใน `HashMap` ตามหมวดหมู่

7. **`stock_status()` + `match guard`**  
   → ตรวจสอบว่าสินค้าหมด ใกล้หมด หรือมีสต๊อกปกติ โดยใช้ `reorder_level` เป็นเงื่อนไข

8. **`Display Trait`**  
   → กำหนดรูปแบบการแสดงข้อมูล `Product` เอง ทำให้สามารถใช้ `println!("{}", product)` ได้

9. **`sort_unstable_by()` + Closure**  
   → เรียงสินค้าตามจำนวนสต๊อกจากน้อยไปมาก

10. **`iter().take(3)`**  
    → เลือกสินค้าที่มีสต๊อกน้อยที่สุดเพียง 3 รายการ

11. **`filter()` + `map()` + `collect()`**  
    → กรองสินค้าที่ต้องสั่งเพิ่ม แล้วดึงเฉพาะชื่อสินค้าออกมาเก็บเป็น `Vec<String>`

12. **`clone()`**  
     → สร้างข้อมูล `Product` ชุดใหม่สำหรับการจัดอันดับ โดยไม่เปลี่ยน `inventory` เดิม
    ]`

---

## 7. Common Mistakes

### Mistake 1 — `การลืมเรื่อง Ownership`

**Problem**

`การที่คุณส่งข้อมูล string หรือ vec เข้าไปในฟังก์ชั่นโดยไม่ใช้ "&"`

**Incorrect Code**

```rust
fn print_message(msg: String) {
  println!("2: {}", msg);
}
fn main() {
   let my_str = String::from("Hello Rust");
  print_message(my_str);

 println!("Message: {}", my_str);
}
```

**Correct Code**

```rust
// เปลี่ยนมารับพารามิเตอร์เป็น &String (หรือ &str)
fn print_message(msg: &String) {
    println!("Message: {}", msg);
}

fn main() {
    let my_str = String::from("Hello Rust");
    print_message(&my_str);

    println!("Message: {}", my_str);
}
```

**Why?**

`[สิทธิ์ความเป็นเจ้าของถูกย้ายไปเพราะไม่ได้ใช้ "&" ทำให้ไม่สามารถใช้ตัวแปรเดิมได้อีก]`

---

### Mistake 2 — `การพยายามเพิ่ม/ลบ ข้อมูลใน Collection ขณะกำลังวนลูปอ่าน`

**Problem**

`ในภาษาอื่นคุณอาจจะวนลูป Array แล้วใช้ push() เข้าไปตรงๆได้เลย แต่ใน Rust จะไม่อนุญาต`

**Incorrect Code**

```rust
fn main() {
    let mut my_vec = vec![1, 2, 3];


    for item in &my_vec {
        if *item == 2 {
            //Error : Rust ไม่ยอมให้เขียนข้อมูลตอนที่กำลังมีคนยืมอ่านอยู่
            my_vec.push(4);
        }
    }
}
```

**Correct Code**

```rust
fn main() {
    let mut my_vec = vec![1, 2, 3];
    let mut to_add = Vec::new(); // สร้าง Vector เปล่ามารอเก็บของใหม่

    for item in &my_vec {
        if *item == 2 {
            to_add.push(4); // เอาไปพักไว้ที่อื่นก่อน ปลอดภัย
        }
    }

// Loop ด้านบนจบไปแล้ว คืนสิทธิ์การอ่านแล้ว ตอนนี้เพิ่มข้อมูลเข้า my_vec ได้
    my_vec.append(&mut to_add);

    println!("{:?}", my_vec); // ผลลัพธ์: [1, 2, 3, 4]
}
```

**Why?**

`การเพิ่มข้อมูลอาจทำให้ Vector ต้องย้ายที่อยู่บน Heap ซึ่งจะทำให้ตัวแปรใน Loop ชี้ไปที่หน่วยความจำที่พังไปแล้ว`

---

## 8. Exercises

> จัดทำแบบฝึกหัด **2 ข้อ** ที่สอดคล้องกับ Topic และมีระดับความยากเหมาะสม

### Exercise 1 — `นับสต็อกสินค้า`

**Problem**

`คุณได้รับตะกร้าสินค้าที่มีของปะปนกันอยู่ ซึ่งข้อมูลมาในรูปแบบ Vec<String> ที่เก็บชื่อสินค้าเรียงต่อกันไปเรื่อยๆ (มีชื่อสินค้าซ้ำกัน) ให้เขียนฟังก์ชัน count_items เพื่อสรุปว่าเรามีสินค้าแต่ละชนิดอย่างละกี่ชิ้น โดยแปลงให้ออกมาอยู่ในรูปแบบ HashMap<String, u32>`

**Hint**

`ในภาษา Rust เราสามารถใช้ .entry(key).or_insert(0) เพื่อดึงค่าตัวเลขที่อยู่ใน HashMap ออกมาได้ (ถ้ายังไม่มีคีย์นั้น จะตั้งค่าเริ่มต้นเป็น 0 ให้ทันที)`
`เมื่อดึงค่าออกมาได้แล้ว มันจะได้เป็น Reference ที่สามารถแก้ไขค่าได้ (&mut u32)` `สามารถบวกค่าเพิ่มเข้าไปได้โดยใช้เครื่องหมาย * นำหน้า เช่น *count += 1; เพื่อบอกว่า ให้ไปบวกตัวเลขจริงๆ ที่อยู่ในตำแหน่งนี้เพิ่มอีก 1`

**Solution**

```rust
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
```

**Explanation**

`1. วนลูปอ่านของทีละชิ้น (for item in items)`
`โปรแกรมจะหยิบชื่อสินค้าจาก Vec<String> ออกมาดูทีละอันตามลำดับ (เช่น รอบแรกได้ "Apple", รอบสองได้ "Banana")`

`2. ตรวจสอบและจองพื้นที่ด้วย .entry(item)`
`เมื่อได้ชื่อสินค้ามาแล้ว เราจะเอาไปถาม HashMap ว่า "มีสินค้านี้จดไว้หรือยัง?"`

`ฟังก์ชัน entry() จะทำหน้าที่วิ่งไปหาคีย์นั้นๆ ให้ทันที`

`3. กำหนดค่าเริ่มต้นเมื่อเจอของใหม่ด้วย .or_insert(0)`

`ถ้าเพิ่งเจอสินค้านี้ครั้งแรก: HashMap จะสร้างช่องใหม่ขึ้นมา แล้วใส่ตัวเลข 0 ลงไปให้เป็นค่าเริ่มต้น`

`ถ้ามีสินค้านี้อยู่แล้ว: โปรแกรมจะไม่สนใจเลข 0 นี้ แต่จะดึงเอาตัวเลขจำนวนปัจจุบันออกมาแทน`

`สิ่งที่ได้กลับมา: ไม่ว่าจะเข้าเงื่อนไขไหน สิ่งที่คืนค่ากลับมาให้ตัวแปร count คือ Reference (ตัวชี้เป้า) ที่ชี้ไปยังตัวเลขจริงๆ ใน HashMap (มีชนิดข้อมูลเป็น &mut u32)`

`4. อัปเดตตัวเลขด้วย * (Dereference)`
`เนื่องจากตัวแปร count เป็นแค่ตัวชี้เป้า เราไม่สามารถเอา count + 1 ได้ตรงๆ ในภาษา Rust เราจึงต้องใส่เครื่องหมาย * ไว้ข้างหน้าเป็น *count เพื่อเป็นการสั่งว่า "ให้เข้าไปแก้ไขตัวเลขจริงๆ ที่เก็บอยู่ใน HashMap ตรงตำแหน่งที่ชี้อยู่นั้น ให้บวกเพิ่มอีก 1"`

---

### Exercise 2 — `ระบบดึงแฮชแท็กแบบไม่ซ้ำ`

**Problem**

`คุณได้รับข้อความโพสต์จากโซเชียลมีเดียที่มีแฮชแท็กปะปนอยู่ ให้เขียนฟังก์ชัน extract_hashtags ที่รับข้อความ (&str) แล้วดึงเอาเฉพาะคำที่เป็นแฮชแท็ก (คำที่ขึ้นต้นด้วย #) ออกมาโดย 1.ตัดเครื่องหมาย # ด้านหน้าออกไป เอาแค่ชื่อแท็ก 2.ถ้ามีแฮชแท็กซ้ำกัน ให้เก็บไว้แค่ชื่อเดียว 3.ส่งคืนผลลัพธ์เป็น HashSet<String>`

**Hint**

`ใช้ .split_whitespace() ในการแยกข้อความเป็นคำๆ`

`คุณสามารถเช็คว่า String หรือ &str ขึ้นต้นด้วยตัวอักษรอะไรได้โดยใช้เมธอด .starts_with('#')`

`การตัดตัวอักษรตัวแรก (#) ออก สามารถทำได้หลายวิธี แต่วิธีที่ง่ายที่สุดคือการใช้ String Slicing เช่น &word[1..] หมายถึง "เอาตั้งแต่ตัวอักษรที่ 1 (ตัวที่สอง) ไปจนจบข้อความ"`

`ใช้ .insert(...) ในการเพิ่มข้อมูลลงในHashSet`

**Solution**

```rust
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
```

**Explanation**

`1. แยกคำและค้นหาเป้าหมาย`
เราเริ่มจากการวนลูปข้อความทั้งหมดด้วย .split_whitespace() เพื่อดูทีละคำ จากนั้นใช้ if word.starts_with('#') เพื่อคัดกรองเอาเฉพาะคำที่เป็นแฮชแท็กจริงๆ`

`2. ตัดเครื่องหมายและแปลงชนิดข้อมูล`
`เมื่อเจอคำว่า "#rust" เราต้องการแค่ "rust" ในภาษา Rust เราสามารถเฉือน (Slice) ข้อความได้ด้วยการระบุตำแหน่ง (Index) โดยตัวแรกคือตำแหน่งที่ 0 ดังนั้นเราจะดึงข้อมูลด้วย &word[1..] ซึ่งหมายความว่าให้เอาตั้งแต่ตำแหน่งที่ 1 ไปจนสุดคำ`
`ข้อควรระวัง: ข้อมูลที่ได้จากการ Slice จะยังมีชนิดเป็น &str (เป็นแค่การชี้ไปยังข้อความเดิม) เราต้องใช้ .to_string() เพื่อคัดลอกมาสร้างเป็นก้อนข้อความใหม่ของเราเอง`

`3. จัดเก็บแบบไม่ซ้ำด้วย HashSet`
`เรานำคำที่ตัดเรียบร้อยแล้วไปใส่ใน tags.insert(tag) ความพิเศษของ HashSet คือเราไม่ต้องเขียน if เพื่อเช็คเลยว่ามีแท็กนี้อยู่แล้วหรือยัง ถ้าเราสั่ง .insert() คำซ้ำลงไป (เช่น "rust" รอบที่สอง) HashSet จะรู้ตัวและปฏิเสธการเพิ่มค่านั้นให้เองโดยอัตโนมัติ ทำให้เราได้ผลลัพธ์เฉพาะคำที่ไม่ซ้ำกันเลย`

---

## 9. PPL Perspective

> **ส่วนนี้เป็นหัวใจของรายวิชา Principles of Programming Languages**

วิเคราะห์ Topic นี้ในมุมมองของ Programming Languages

### 9.1 Syntax


**Syntax** คือรูปแบบและกฎในการเขียนโปรแกรมว่าต้องเขียนอย่างไรจึงจะเป็นคำสั่งที่ถูกต้องตามภาษานั้น

สำหรับ Collections & Strings ของ Rust Syntax ที่สำคัญประกอบด้วย

- การประกาศ Binding / Variable
- การกำหนด Type
- การสร้าง Collection
- การเข้าถึงข้อมูล
- การแก้ไขข้อมูล
- การลบข้อมูล
- การวนซ้ำข้อมูล
- การใช้ Method และ Function ที่เกี่ยวข้อง

ตัวอย่างง่าย ๆ

```rust
let mut name = String::from("Rust");
name.push_str(" Language");
```

ถ้ามองเฉพาะ Syntax เราจะเห็นว่า Rust ใช้

```text
let
 ↓
identifier
 ↓
=
 ↓
expression
 ↓
;
```

แต่ในมุม PPL เราจะไม่หยุดที่ “เขียนอย่างไร” เพราะ Syntax จะเชื่อมต่อไปยัง Semantics และ Type System ว่าโค้ดนี้มีความหมายอะไรและ `name` มี Type อะไร

---

### Collections ที่สำคัญใน Rust

Rust มี Collection หลายประเภท ในหัวข้อนี้เน้น 4 ประเภทหลัก

#### `Vec<T>`

เป็น Collection สำหรับเก็บข้อมูลหลายค่าในลำดับเดียวกัน และสามารถเพิ่มหรือลดจำนวน Element ได้

```rust
let numbers: Vec<i32> = vec![10, 20, 30];
```

`T` คือ **Type Parameter**

```text
Vec<i32>
    ↑
    T = i32
```

จึงหมายความว่า Collection นี้เก็บ `i32`

ตัวอย่างอื่น

```rust
Vec<String>
Vec<char>
Vec<u8>
```

---

#### `String`

เป็น Owned String ที่เก็บข้อความในรูปแบบ UTF-8 และสามารถขยายขนาดได้

```rust
let mut s = String::from("Hello");
s.push('!');
```

จุดสำคัญคือ `String` ไม่ควรถูกมองว่าเป็นเพียง “Array ของตัวอักษร” เพราะ representation ภายในเป็น UTF-8 byte sequence

---

#### `&str`

เป็น **String Slice** หรือ Reference ไปยังข้อมูลข้อความที่มีอยู่แล้ว

```rust
let s = String::from("Hello");
let part: &str = &s[0..5];
```

`&str` ไม่ได้เป็นเจ้าของข้อมูล String

---


### การประกาศตัวแปรและ Mutability

Rust ใช้ `let` สำหรับสร้าง Binding

```rust
let name = String::from("Rust");
```

ตัวแปรใน Rust เป็น **Immutable โดยค่าเริ่มต้น**

หากต้องการแก้ไขข้อมูลต้องระบุ `mut`

```rust
let mut name = String::from("Rust");
name.push_str(" Language");
```

ดังนั้น Syntax ของ Rust สะท้อนแนวคิดด้าน Mutability โดยตรง

```text
let       → Binding
let mut   → Mutable Binding
```

นี่เป็นตัวอย่างของ PPL ที่ Syntax ไม่ได้เป็นเพียงรูปแบบการเขียน แต่สัมพันธ์กับกฎของภาษาโดยตรง

---

### การสร้าง Collection

Collection แต่ละประเภทมี Syntax สำหรับสร้างแตกต่างกัน

```rust
// Vec
let a: Vec<i32> = Vec::new();
let b = vec![1, 2, 3];

// String
let s1 = String::new();
let s2 = String::from("Rust");

// &str
let text: &str = "Rust";
```

Rust ไม่มี Constructor แบบบังคับในรูปแบบเดียวกับภาษา OOP บางภาษา แต่ใช้ Associated Function, Macro และ API ของ Standard Library

---

### Creation, Access, Modification, Removal, Iteration

เพื่อมอง Syntax ของ Collection อย่างเป็นระบบ สามารถแบ่ง Operation ออกเป็น 5 กลุ่ม

| Operation | ความหมาย | ตัวอย่าง |
|---|---|---|
| Creation | สร้าง Collection | `Vec::new()`, `String::new()` |
| Access | เข้าถึงข้อมูล | `v[0]`, `v.get(0)`, `map.get(&key)` |
| Modification | เพิ่มหรือแก้ข้อมูล | `push`, `insert`, `push_str` |
| Removal | นำข้อมูลออก | `pop`, `remove` |
| Iteration | วนผ่านข้อมูล | `iter`, `chars`, `bytes` |

แต่ละ Collection มีรูปแบบการเข้าถึงไม่เหมือนกัน

```text
Vec       → ตำแหน่ง / Index
HashMap   → Key
String    → Byte / Unicode Scalar Value / Slice
```

---

### Syntax ของ String: `String` กับ `&str`

สอง Type ที่มักทำให้ผู้เริ่มต้นสับสนคือ

```rust
let a = "Hello";
let b = String::from("Hello");
```

แม้ข้อความเหมือนกัน แต่ Type ไม่เหมือนกัน

```text
a : &str
b : String
```

โดยทั่วไป String Literal เช่น `"Hello"` เป็น `&'static str`

ส่วน `String::from("Hello")` สร้าง Owned `String`

ดังนั้น Syntax ที่ดูคล้ายกันอาจนำไปสู่ Semantics และ Type ที่แตกต่างกัน

---

### Syntax กับ UTF-8

ตัวอย่าง

```rust
let s = String::from("สวัสดี");
```

สิ่งที่มนุษย์เห็นคือข้อความภาษาไทย แต่ภายใน String ถูกเก็บเป็น UTF-8 bytes

ดังนั้น

```rust
s.len()
```

ไม่ได้หมายถึงจำนวน “ตัวอักษรที่มองเห็น”

แต่หมายถึง **จำนวน Byte**

นี่เป็นจุดที่ Syntax เริ่มเชื่อมไปยัง Semantics อย่างชัดเจน

---

### ทำไม Rust จึงไม่มี `String[0]` แบบ Character Index?

ถ้าเขียน

```rust
s[0]
```

คำว่า `0` อาจถูกตีความได้หลายแบบ

```text
Byte ที่ 0
Unicode Scalar Value ที่ 0
Grapheme Cluster ที่ 0
```

สามอย่างนี้ไม่จำเป็นต้องตรงกัน

ดังนั้น Rust ไม่กำหนดให้ Integer Index ของ `String` หมายถึง Character โดยตรง

ถ้าต้องการทำงานแต่ละระดับ ควรระบุ operation ให้ชัดเจน เช่น

```rust
s.bytes()
s.chars()
s.char_indices()
```

หรือ String Slice

```rust
&s[start..end]
```

โดย Index ของ String Slice เป็น **Byte Index** และต้องอยู่บน UTF-8 Character Boundary

---



### 9.2 Semantics

#### ความหมายของ Semantics
**Semantics** คือความหมายหรือพฤติกรรมที่เกิดขึ้นเมื่อคำสั่งถูกประมวลผล

ถ้า Syntax ถามว่า

> “เขียนอย่างไร?”

Semantics จะถามว่า

> “สิ่งที่เขียนนั้นหมายความว่าอะไร และเมื่อทำงานแล้วเกิดอะไรขึ้น?”

ตัวอย่าง

```rust
let mut s = String::from("Hello");
s.push('!');
```

Syntax บอกว่าเขียนถูกต้อง

แต่ Semantics อธิบายว่า

```text
String::from("Hello")
        ↓
สร้าง Owned String

push('!')
        ↓
เพิ่มข้อมูลเข้าไปใน String
```

ผลคือ

```text
Hello
  ↓
Hello!
```

---

### Ownership, Move และ Borrowing ในมุมของ Semantics

`String` และ `Vec` เป็น Owned Values ที่เกี่ยวข้องกับ Resource

```rust
let s1 = String::from("Hello");
let s2 = s1;
```

ความหมายของคำสั่งนี้ไม่ใช่การ Copy String แบบง่าย ๆ แต่เป็น **Move**

```text
s1
 │
 │ Move
 ▼
s2
```

หลังจาก Move แล้ว `s1` ไม่สามารถนำ Value เดิมไปใช้ต่อได้

ในทางกลับกัน การ Borrow

```rust
let s = String::from("Hello");
let r = &s;
```

หมายถึง `r` ยืมข้อมูลจาก `s` โดยไม่ได้รับ Ownership

เนื่องจากกลุ่มอื่นทำ Ownership โดยตรงอยู่แล้ว ในบทนี้จะใช้ Ownership เป็นเพียงตัวอธิบายว่า Collection และ String มีพฤติกรรมด้าน Resource อย่างไร

---

### Semantics ของ `Vec`

`Vec<T>` มีพฤติกรรมเป็น Collection แบบลำดับ

```rust
let mut v = vec![10, 20, 30];
v.push(40);
```

หลัง `push`

```text
[10, 20, 30]
       ↓
[10, 20, 30, 40]
```

หากข้อมูลเพิ่มจนเกิน Capacity ปัจจุบัน ระบบอาจต้อง Reallocate Buffer ใหม่

ดังนั้นการเพิ่ม Element อาจทำให้ตำแหน่งของ Buffer เปลี่ยนแปลง

---

### Index กับ `get`

```rust
let v = vec![10, 20, 30];

let a = v[0];
let b = v.get(0);
```

มี Semantics แตกต่างกัน

```text
v[0]
↓
Index โดยตรง
↓
ถ้าเกินขอบเขต → panic

v.get(0)
↓
Option
↓
Some(value) หรือ None
```

ดังนั้น API ที่ต่างกันสะท้อนนโยบายด้าน Error Handling ของภาษา

---

## Semantics ของ String และ UTF-8

นี่คือส่วนสำคัญที่สุดของบทนี้

Rust `String` ใช้ **UTF-8 Encoding**

ดังนั้นข้อมูลที่อยู่ภายในไม่ได้เป็น

```text
1 Character = 1 ช่อง
```

เสมอไป

แต่เป็นลำดับของ Byte ที่เข้ารหัสตาม UTF-8

เราควรแยกอย่างน้อย 4 แนวคิด

```text
Byte
Unicode Scalar Value
Grapheme Cluster
String Slice
```

---

### 1. Byte

เป็นหน่วยข้อมูลที่อยู่ใน UTF-8 representation

ตัวอย่าง ASCII

```text
'A'
```

ใช้ 1 Byte

แต่ตัวอักษรที่อยู่นอก ASCII เช่นภาษาไทย ใช้หลาย Byte

ดังนั้น

```rust
let s = "ก";
println!("{}", s.len());
```

จะได้จำนวน Byte ไม่ใช่ “จำนวนตัวอักษร”

---

### 2. Unicode Scalar Value

ใน Rust `char` แทน **Unicode Scalar Value**

```rust
for c in s.chars() {
    println!("{}", c);
}
```

ดังนั้น `chars()` ไม่ได้วนทีละ Byte แต่แปลง UTF-8 เป็น Unicode Scalar Values ตามลำดับ

---

### 3. Grapheme Cluster

สิ่งที่มนุษย์รับรู้ว่าเป็น “ตัวอักษรหนึ่งหน่วย” ไม่จำเป็นต้องตรงกับ Unicode Scalar Value หนึ่งตัว

อักขระที่ผู้ใช้มองเห็นหนึ่งหน่วยอาจประกอบด้วยหลาย Unicode Scalar Values

ดังนั้น

```text
Byte
≠ Unicode Scalar Value
≠ Grapheme Cluster
```

นี่เป็นเหตุผลว่าทำไมคำว่า “จำนวนตัวอักษร” ต้องระบุให้ชัดว่าหมายถึงระดับไหน

---

### 4. String Slice

`&str` เป็น String Slice ที่อ้างอิงข้อมูล UTF-8

```rust
let s = String::from("Hello");
let part: &str = &s[0..3];
```

`part` เป็น View ไปยังข้อมูลเดิม ไม่ใช่ String ชุดใหม่ในความหมายของ Ownership

---

### `len()` กับความหมายของ Length

```rust
let s = String::from("สวัสดี");
println!("{}", s.len());
```

`String::len()` วัดเป็น **Byte**

ดังนั้น

```text
Length ของ String
=
จำนวน UTF-8 Bytes
```

ไม่ใช่

```text
จำนวน char
```

และไม่ใช่

```text
จำนวน Grapheme Cluster
```

---

### ทำไม Rust ไม่ให้ `String[0]`?

เพราะถ้า Rust อนุญาต

```rust
s[0]
```

จะต้องตัดสินใจว่า `0` หมายถึงอะไร

```text
Byte?
Char?
Grapheme?
```

และสำหรับ UTF-8 ข้อมูลแต่ละ Character มีจำนวน Byte ไม่เท่ากัน

Rust จึงเลือกไม่ให้ Integer Indexing กับ `String`

แทนที่จะบังคับให้ทุกคนต้องจำว่าการ Index หมายถึงอะไร

ผู้เขียนต้องเลือกวิธีที่สื่อความหมาย เช่น

```rust
s.bytes()
s.chars()
s.char_indices()
```

หรือ Slice ที่ใช้ Byte Index

นี่เป็นตัวอย่างสำคัญของ **Language Design ที่แลกความสะดวกกับความชัดเจนและความปลอดภัย**

---

### Semantics ของ `Option`

Collection บาง API คืน `Option`

```rust
let value = v.get(10);
```

ผลลัพธ์อาจเป็น

```text
Some(value)
```

หรือ

```text
None
```

ความหมายคือ “การไม่มีข้อมูล” ถูกทำให้เป็นส่วนหนึ่งของ Semantics อย่างชัดเจน แทนที่จะคืนค่าพิเศษที่อาจทำให้เกิดความสับสน

---

### Use-after-free, Dangling Reference และ Data Race

Ownership และ Borrowing ของ Rust ช่วยป้องกันปัญหาหลายประเภท

- **Use-after-free** — ใช้ข้อมูลหลังจาก Resource ถูกคืนแล้ว
- **Dangling Reference** — Reference ยังชี้ไปยังข้อมูลที่หมดอายุ
- **Data Race** — หลาย Thread เข้าถึงข้อมูลเดียวกันพร้อมกัน โดยมีอย่างน้อยหนึ่งส่วนแก้ไขข้อมูลและไม่มี Synchronization ที่เหมาะสม

Safe Rust ใช้ Type System และ Borrow Checker ช่วยตรวจสอบปัญหาเหล่านี้ตั้งแต่ Compile Time

---

### 9.3 Type System

#### ความหมายของ Type System

**Type System** คือระบบที่กำหนดและตรวจสอบว่าข้อมูลแต่ละส่วนมี Type อะไร และสามารถนำไปใช้กับ Operation ใดได้บ้าง

Rust มี **Static Type System** ซึ่งตรวจสอบ Type ในช่วง Compile Time

---

### Static Typing

ตัวอย่าง

```rust
let x: i32 = 10;
let s: String = String::from("Rust");
```

Compiler สามารถตรวจสอบ Type ก่อนโปรแกรมทำงานจริง

ดังนั้นข้อผิดพลาดบางประเภทถูกพบตั้งแต่ Compile Time

---

### Type Inference

Rust ไม่จำเป็นต้องเขียน Type ทุกตำแหน่ง

```rust
let x = 10;
let s = String::from("Rust");
```

Compiler สามารถอนุมาน Type จากบริบท

```text
10
↓
i32 โดยบริบทเริ่มต้นทั่วไป

String::from(...)
↓
String
```

ดังนั้น Rust จึงมีทั้ง Static Typing และ Type Inference

---

### Generics

`Vec<T>` เป็นตัวอย่างของ Generic

```rust
Vec<i32>
Vec<String>
Vec<char>
Vec<u8>
```

`T` เป็น Type Parameter

ทำให้ Collection เดียวกันสามารถนำไปใช้กับหลาย Type โดยไม่ต้องสร้างโครงสร้างใหม่สำหรับแต่ละ Type

---

### Monomorphization

สำหรับ Generic บางประเภท Rust Compiler สามารถใช้ **Monomorphization**

แนวคิดคือ Compiler สร้างรูปแบบโค้ดที่เหมาะกับ Type ที่ถูกใช้งานจริง

เช่น

```rust
fn print_value<T>(x: T) {
    // ...
}
```

เมื่อใช้กับหลาย Type Compiler สามารถสร้างรูปแบบที่เหมาะกับแต่ละ Type

ข้อดีคือสามารถใช้ Static Dispatch และ Optimize ได้ดี

ข้อแลกเปลี่ยนคือ Binary อาจมีขนาดเพิ่มขึ้นจากการสร้างโค้ดหลายรูปแบบ

---

## `String` และ `&str` เป็นคนละ Type

นี่เป็นหัวใจของ Type System ในเรื่อง String

```text
String
&str
```

ไม่ใช่ Type เดียวกัน

### `String`

เป็น Owned Value

```rust
let s = String::from("Rust");
```

มี Ownership ของข้อมูล

### `&str`

เป็น Borrowed String Slice

```rust
let s = String::from("Rust");
let view: &str = &s;
```

ไม่ได้เป็นเจ้าของข้อมูล

---

### `String`, `&String` และ `&str`

ควรแยก 3 แบบ

```text
String
↓
Owned Value

&String
↓
Reference ที่ยืม String

&str
↓
Reference ไปยัง String Slice
```

ตัวอย่าง

```rust
fn show(text: &str) {
    println!("{}", text);
}

let s = String::from("Rust");
show(&s);
```

Rust มี **Deref Coercion** ที่ช่วยให้ `&String` ถูกใช้ในตำแหน่งที่ต้องการ `&str` ได้ในหลายกรณี

จึงทำให้ Type ที่แตกต่างกันสามารถทำงานร่วมกันได้สะดวกขึ้นโดยไม่ต้องเปลี่ยน Ownership

---

### `&str` กับ Fat Reference

`str` เป็น **Dynamically Sized Type (DST)**

จึงไม่สามารถเก็บเป็น Value แบบ `str` ตรง ๆ ได้ในกรณีทั่วไป แต่จะถูกอ้างอิงผ่าน `&str`

ในเชิงแนวคิด `&str` ต้องรู้ทั้ง

```text
ตำแหน่งของข้อมูล
+
ความยาวของข้อมูล
```

จึงสามารถมองเป็น **Fat Reference / Fat Pointer** ได้

นี่เป็นตัวอย่างที่ดีของการที่ Type System เชื่อมกับ Representation ใน Memory

---

### `String` กับ `Vec<u8>`

แม้ทั้งสองเกี่ยวข้องกับข้อมูล Byte

```text
Vec<u8>
↓
ลำดับ Byte ทั่วไป

String
↓
UTF-8 Text
```

จึงไม่ควรเขียนว่า

```text
Vec<u8> = String
```

แต่ควรเข้าใจว่า `String` มีความสัมพันธ์กับ byte buffer และมี Invariant ที่สำคัญกว่า คือข้อมูลภายในต้องเป็น **Valid UTF-8**

---

### UTF-8 และ Type Safety

`String` มี Invariant ว่าข้อมูลภายในต้องเป็น UTF-8 ที่ถูกต้อง

ดังนั้นถ้ามี Raw Bytes

```rust
let bytes: Vec<u8> = vec![...];
```

เราไม่ควรสมมติว่า Bytes เหล่านี้เป็น String

ต้องตรวจสอบก่อน

```rust
let result = String::from_utf8(bytes);
```

ผลลัพธ์สามารถบอกได้ว่า

```text
Ok(String)
```

หรือ

```text
Err(...)
```

ดังนั้น Type System และ API ของ Rust ทำให้ “Valid UTF-8” กลายเป็นเงื่อนไขที่ต้องรักษา

---

### `Option<T>` ใน Type System

Rust ใช้

```text
Option<T>
```

แทนแนวคิด “มีค่าหรือไม่มีค่า”

```rust
Some(value)
None
```

ตัวอย่าง

```rust
let value = numbers.get(0);
```

Type ของผลลัพธ์คือ

```text
Option<&T>
```

ดังนั้น Compiler บังคับให้ผู้เขียนพิจารณากรณีที่ไม่มีข้อมูลอย่างชัดเจน

---

### Trait และ Trait Bound

Trait ใช้กำหนดพฤติกรรมหรือความสามารถของ Type

Trait Bound ใช้กำหนดเงื่อนไขสำหรับ Generic

ตัวอย่างสำคัญคือ `HashMap<K,V>` ต้องใช้คุณสมบัติที่เหมาะสมกับ Key

โดยทั่วไป Key ต้องรองรับ

```text
Eq
Hash
```

และคุณสมบัติของสอง Trait ต้องสอดคล้องกันเพื่อให้การค้นหาและจัดเก็บทำงานได้ถูกต้อง

---

### Type Compatibility

Rust ไม่ถือว่า Type ที่มีลักษณะคล้ายกันสามารถใช้แทนกันได้ทั้งหมด

```text
String
&String
&str
```

เป็นคนละ Type

แต่มีความสัมพันธ์ในการใช้งานผ่าน

```text
Reference
Deref Coercion
```

จึงเกิดสมดุลระหว่าง

```text
Strict Type System
+
Practical Usability
```

---

### 9.4 Memory / Resource Management

#### ความหมายของ Memory Management

Memory Management คือการจัดการพื้นที่หน่วยความจำตั้งแต่

```text
Allocation
↓
Usage
↓
Reallocation
↓
Deallocation
```

Rust ใช้แนวคิดสำคัญ เช่น

```text
Ownership
Borrowing
Lifetime
RAII
Drop
```

เพื่อจัดการ Resource โดยไม่ต้องใช้ Garbage Collector แบบภาษา Java หรือ Python

---

### Stack และ Heap

Rust ใช้ทั้ง Stack และ Heap

สำหรับ `String` และ `Vec` เราสามารถมองภาพแบบง่ายได้ว่า

```text
Stack
┌─────────────────────┐
│ String              │
│ pointer             │
│ length              │
│ capacity            │
└─────────┬───────────┘
          │
          ▼
Heap
┌─────────────────────┐
│ UTF-8 bytes         │
│ ...                 │
└─────────────────────┘
```

ข้อควรระวังคือภาพนี้เป็น **แนวคิดของ Representation / Implementation** ไม่ควรสรุปว่า Memory Layout ทุก Platform ต้องเหมือนกันทั้งหมด

---

### Length และ Capacity

สำหรับ `Vec`

```text
Length
=
จำนวน Element ที่ใช้งานจริง
```

สำหรับ `String`

```text
String::len()
=
จำนวน UTF-8 Bytes
```

ส่วน Capacity คือจำนวนพื้นที่ที่ Buffer สามารถรองรับได้ก่อนต้อง Allocation ใหม่

ดังนั้น

```text
Length <= Capacity
```

โดยทั่วไป

---

### UTF-8 กับ Memory

นี่เป็นประเด็นที่สำคัญมาก

ถ้า String มีข้อความ

```text
Hello
```

ข้อมูลเป็น ASCII และแต่ละตัวใช้ 1 Byte

แต่ถ้าเป็นข้อความภาษาไทย แต่ละ Unicode Scalar Value อาจใช้หลาย Byte

ดังนั้น

```text
จำนวนตัวอักษรที่มนุษย์เห็น
≠
จำนวน Bytes ที่ Memory ใช้
```

ตัวอย่างเช่น

```rust
let s = "ก";
println!("{}", s.len());
```

ผลคือจำนวน Byte ของ UTF-8 ไม่ใช่ 1 เพียงเพราะมนุษย์มองเห็นเป็นหนึ่งตัว

---

### Allocation

เมื่อสร้าง Collection ที่ต้องใช้ Dynamic Memory ระบบต้องขอพื้นที่จาก Allocator

ตัวอย่าง

```rust
let mut v = Vec::new();
v.push(1);
v.push(2);
```

หรือ

```rust
let mut s = String::new();
s.push_str("Rust");
```

พื้นที่ที่จัดสรรอาจมากกว่าข้อมูลที่ใช้งานจริง เพื่อให้สามารถเพิ่มข้อมูลได้โดยไม่ต้อง Reallocate ทุกครั้ง

---

### Reallocation

เมื่อข้อมูลเพิ่มจนเกิน Capacity

```text
Buffer เดิม
[ capacity ไม่พอ ]
       ↓
Allocate ใหม่
       ↓
ย้ายข้อมูล
       ↓
คืนพื้นที่เดิม
```

ตำแหน่งของข้อมูลบน Heap อาจเปลี่ยน

นี่เป็นเหตุผลสำคัญที่ Rust ต้องควบคุม Reference และ Borrowing อย่างเข้มงวด

---

### ความสัมพันธ์ระหว่าง Reallocation กับ Borrowing

สมมติ

```rust
let mut v = vec![1, 2, 3];

let first = &v[0];

// ถ้า operation นี้ต้อง Reallocate
v.push(4);
```

ปัญหาคือ Reference เดิมอาจชี้ไปยัง Buffer เก่าที่ถูกย้ายแล้ว

Borrow Checker จึงไม่อนุญาตให้เกิดสถานการณ์ที่ Reference ยังใช้งานอยู่ แต่ Collection ถูกแก้ไขในลักษณะที่อาจทำให้ Reference ไม่ Valid

นี่เป็นตัวอย่างสำคัญของ

```text
Memory Management
        +
Type System
        +
Borrowing
```

ที่ทำงานร่วมกัน

---

### Deallocation

เมื่อ Owner ของ Resource หมด Scope Rust สามารถจัดการ Resource ที่เกี่ยวข้องโดยอัตโนมัติ

```rust
{
    let s = String::from("Rust");
} // s ถูก Drop
```

ผู้เขียนไม่ต้องเรียก `free()` แบบ C สำหรับ Resource ที่อยู่ภายใต้ Ownership System

---

### RAII

แนวคิดนี้สอดคล้องกับ **RAII (Resource Acquisition Is Initialization)**

แนวคิดคือ Resource ถูกผูกกับอายุของ Value

```text
สร้าง Value
↓
Resource ถูกถือครอง
↓
Value หมดอายุ
↓
Cleanup
```

จึงช่วยลดปัญหาการลืมคืน Resource

---

### Drop

`Drop` ใช้กำหนดพฤติกรรม Cleanup เมื่อ Value ถูกทำลาย

สำหรับ Collection เช่น `String` และ `Vec` การ Drop จะนำไปสู่การจัดการ Resource ที่เกี่ยวข้องกับ Buffer

---

### Ownership กับ Memory

Ownership เป็นกลไกสำคัญในการตอบคำถามว่า

> “ใครรับผิดชอบ Resource นี้?”

ตัวอย่าง

```rust
let s1 = String::from("Rust");
let s2 = s1;
```

Ownership ถูก Move ไปยัง `s2`

จึงทำให้ระบบรู้ว่าเมื่อถึงเวลาทำลาย Resource ควรให้ใครรับผิดชอบ

ในบทนี้จึงไม่ลงลึกกฎ Ownership ซ้ำ แต่ใช้เพื่ออธิบายความสัมพันธ์ระหว่าง String กับ Memory Management

---

### Borrowing กับ Memory

Reference เป็นการยืมข้อมูล

```rust
let s = String::from("Rust");
let r = &s;
```

`r` ไม่ได้เป็นเจ้าของ String

ดังนั้น Reference ต้องไม่สามารถมีอายุยาวนานกว่าข้อมูลที่มันอ้างอิง

---

### Lifetime

Lifetime คือช่วงเวลาที่ Reference ต้องยังคงใช้งานได้อย่างถูกต้อง

ไม่ได้หมายความว่า Lifetime เป็นคำสั่งสำหรับจัดสรร Memory

แต่เป็นข้อมูลเชิงความสัมพันธ์ที่ช่วยให้ Compiler ตรวจสอบว่า Reference ไม่สามารถอยู่ได้นานกว่าข้อมูลต้นทาง

จึงช่วยป้องกัน Dangling Reference

---

### Shared Ownership

บางกรณีต้องการให้หลายส่วนของโปรแกรมถือ Ownership ร่วมกัน

Rust มี

```text
Rc<T>
Arc<T>
```

`Rc<T>` เหมาะกับ Shared Ownership ใน Single-Threaded Context

`Arc<T>` ใช้ Reference Counting ที่สามารถใช้กับกรณีข้าม Thread ได้เมื่อ Type อื่น ๆ เป็นไปตามข้อกำหนด

แนวคิดนี้แสดงว่า Rust ไม่ได้มีแค่ “Owner คนเดียว” ในทุกกรณี แต่การแชร์ Ownership ต้องใช้ Type ที่ระบุรูปแบบการจัดการ Resource อย่างชัดเจน

---

### Memory Safety

Ownership, Borrowing และ Lifetime ทำงานร่วมกันเพื่อช่วยป้องกัน

```text
Use-after-free
Double free
Dangling reference
Data race ใน Safe Rust
```

จุดสำคัญคือปัญหาหลายประเภทสามารถตรวจพบได้ตั้งแต่ Compile Time

---

### 9.5 Abstraction / Other PPL Concepts

**Abstraction** คือการซ่อนรายละเอียดภายในที่ผู้ใช้ไม่จำเป็นต้องจัดการเอง และเปิด Interface ที่เหมาะสมให้ใช้งาน

ตัวอย่าง

```rust
let mut s = String::from("Hello");
s.push_str(" Rust");
```

ผู้เขียนไม่จำเป็นต้องเขียนเองว่า

```text
Allocate Memory
↓
หา Buffer
↓
ตรวจ Capacity
↓
ขยาย Buffer หากจำเป็น
↓
Copy Bytes
↓
รักษา UTF-8 Invariant
```

รายละเอียดเหล่านี้ถูกซ่อนอยู่ภายใต้ Abstraction ของ `String`

---

## Abstraction ไม่ได้แปลว่า “ซ่อนทุกอย่าง”

Rust เปิดให้ผู้เขียนเลือกว่าจะทำงานในระดับใด

```rust
s.len()
```

→ มอง Length ในระดับ Byte

```rust
s.as_bytes()
```

→ มอง Representation เป็น Bytes

```rust
s.chars()
```

→ มอง Unicode Scalar Values

```rust
s.char_indices()
```

→ มอง Character พร้อมตำแหน่ง Byte

ดังนั้น

```text
High-level
String
   │
   ├── push_str()
   ├── chars()
   ├── bytes()
   ├── len()
   │
   ▼
Low-level
UTF-8 bytes
```

นี่คือ Abstraction ที่สามารถลงไปดูรายละเอียดเมื่อจำเป็น

---

## String เป็น Abstraction ของ UTF-8 Text

เราสามารถมอง

```text
Raw Bytes
   ↓
UTF-8 Validation
   ↓
String
   ↓
Text Operations
```

ได้

จุดสำคัญคือ `String` ไม่ได้เป็นเพียง Container ของ Byte แต่เป็น Abstraction สำหรับ **Valid UTF-8 Text**

ดังนั้นการสร้าง String จาก Raw Bytes จึงต้องผ่านการตรวจสอบ

```rust
String::from_utf8(bytes)
```

ถ้า Byte Sequence ไม่ถูกต้อง ก็ไม่ควรสร้าง `String` ปกติขึ้นมา

---

## ทำไม String ไม่ใช่ `Vec<char>`?

ถ้ามองจากชื่อ “String = ตัวอักษรหลายตัว” อาจเกิดความคิดว่า

```text
String = Vec<char>
```

แต่ Rust ไม่ออกแบบแบบนั้น

เพราะ UTF-8 เป็น Variable-width Encoding

จึงสามารถเก็บข้อความเป็น UTF-8 Bytes ได้โดยไม่ต้องใช้พื้นที่ระดับ Unicode Scalar Value สำหรับทุกหน่วย

แนวคิดจึงเป็น

```text
String
↓
UTF-8 bytes

chars()
↓
Unicode Scalar Values
```

แทนการบังคับให้ String เป็น

```text
Vec<char>
```

---

## Collection เป็น Abstraction

Collection แต่ละชนิดเป็น Abstraction สำหรับปัญหาที่ต่างกัน

### `Vec<T>`

```text
Sequence
↓
Element ตามลำดับ
↓
Index
```

### `String`

```text
UTF-8 Text
↓
Bytes / chars / slices
```


ดังนั้น Collection ไม่ได้เป็นเพียง “ที่เก็บข้อมูล” แต่เป็นการออกแบบ Abstraction ที่กำหนดวิธีคิดเกี่ยวกับข้อมูล

---

## Trait

Trait ใช้กำหนดพฤติกรรมหรือความสามารถร่วมกันของ Type

เช่น

```rust
trait Printable {
    fn print(&self);
}
```

แนวคิดนี้ช่วยให้สามารถสร้าง Abstraction ที่ไม่ได้ผูกกับ Type ใด Type หนึ่งโดยตรง

---

## Generic

Generic ช่วยให้เขียนโครงสร้างหรือ Function ที่ทำงานกับหลาย Type

```text
Vec<T>
HashMap<K,V>
```

ทำให้ Code Reuse สูงขึ้นโดยยังรักษา Type Safety

---

## Iterator

Iterator เป็น Abstraction สำหรับการประมวลผลข้อมูลทีละ Element

ตัวอย่าง

```rust
let numbers = vec![1, 2, 3, 4, 5];

let result: Vec<i32> = numbers
    .iter()
    .map(|x| x * 2)
    .filter(|x| x > &5)
    .collect();
```

แนวคิดสามารถอ่านเป็น

```text
อ่านข้อมูล
↓
map
↓
filter
↓
collect
```

แทนการเขียนรายละเอียดของ Loop และการจัดการ Index ด้วยตนเอง

---

## Zero-Cost Abstraction

Rust มีแนวคิด **Zero-Cost Abstraction**

แนวคิดหลักคือ

> Abstraction ที่ใช้ไม่ควรเพิ่ม Runtime Cost ที่ไม่จำเป็นเพียงเพราะเราเขียนโค้ดในระดับสูงขึ้น

Generic และ Iterator หลายรูปแบบสามารถถูก Compiler Optimize ได้

จึงพยายามรักษาสมดุลระหว่าง

```text
Readable Code
+
High-level Abstraction
+
Performance
```

---

## Scope และ Binding

**Scope** คือขอบเขตที่ Binding สามารถใช้งานได้

```rust
{
    let s = String::from("Rust");
}
```

` s ` ใช้ได้ภายใน Scope นี้

**Binding** คือความสัมพันธ์ระหว่างชื่อกับ Value

แนวคิดนี้เชื่อมกับ Ownership และ Lifetime เพราะ Scope มีผลต่ออายุของ Value

---

## Programming Paradigm

Rust รองรับหลาย Programming Paradigm

### Imperative Programming

เน้นการสั่งให้โปรแกรมทำงานเป็นขั้นตอน

```rust
let mut total = 0;

for n in numbers {
    total += n;
}
```

### Functional Programming

สามารถใช้ Iterator และ Function มาประกอบกัน

```rust
let total: i32 = numbers
    .iter()
    .map(|x| x * 2)
    .sum();
```

ดังนั้น Rust สามารถผสมแนวคิดหลาย Paradigm ได้

---

### 9.6 Why Rust?

## Rust ถูกออกแบบมาเพื่ออะไร?

จากทั้ง 5 หัวข้อก่อนหน้า เราสามารถเห็นเหตุผลของการออกแบบ Rust ได้จาก

```text
Syntax
   ↓
Semantics
   ↓
Type System
   ↓
Memory Management
   ↓
Abstraction
   ↓
Why Rust?
```

Rust ให้ความสำคัญกับ

- Memory Safety
- Performance
- Type Safety
- Resource Control
- Concurrency Safety
- Predictable Resource Management

---

## 1. Memory Safety

Rust ใช้

```text
Ownership
Borrowing
Lifetime
Type System
```

เพื่อช่วยป้องกันปัญหา Memory หลายประเภทตั้งแต่ Compile Time

เช่น

```text
Use-after-free
Dangling Reference
Double free
Data race ใน Safe Rust
```

---

## 2. ไม่ต้องใช้ Garbage Collector

Rust ไม่ใช้ Garbage Collector แบบ Java หรือ Python

แต่ใช้ Ownership และ Drop เพื่อกำหนดว่า Resource ควรถูกจัดการเมื่อใด

```text
Owner สร้าง Resource
       ↓
ใช้ Resource
       ↓
Owner หมด Scope
       ↓
Drop / Cleanup
```

ข้อดีคือสามารถควบคุม Resource ได้โดยไม่ต้องมี GC ทำงานอยู่เบื้องหลัง

---

## 3. Performance

Rust เป็น Compiled Language ที่สามารถสร้าง Native Code

จึงเหมาะกับงานที่ต้องการควบคุม Resource และประสิทธิภาพสูง เช่น

- System Programming
- Software ที่ต้องใช้ Memory อย่างมีประสิทธิภาพ
- Application ที่ต้องการ Performance สูง
- งานที่ต้องควบคุม Resource ในระดับต่ำ

---

## 4. ทำไม UTF-8 จึงสำคัญต่อ Why Rust?

การเลือก UTF-8 ไม่ใช่แค่เรื่องการรองรับภาษาไทยหรือภาษาต่างประเทศ

แต่สะท้อนการออกแบบที่พยายามรักษาทั้ง

```text
Performance
+
Correctness
+
Explicitness
```

Rust ไม่ทำให้ String กลายเป็น Array ของ Character ที่มีขนาดเท่ากันทุกตัว

แต่เก็บเป็น UTF-8 Bytes และให้ API แยกระดับการเข้าถึง

```text
bytes()
chars()
char_indices()
slices
```

ผู้เขียนจึงต้องบอกให้ชัดว่าต้องการประมวลผลข้อมูลในระดับใด

---

## 5. ทำไม Rust ไม่ให้ `String[index]`?

เพราะ Rust ไม่ต้องการสร้าง Abstraction ที่อาจทำให้เกิดความเข้าใจผิด

ถ้าเขียน

```rust
s[0]
```

ไม่ชัดเจนว่า

```text
Byte?
Unicode Scalar Value?
Grapheme Cluster?
```

ดังนั้น Rust เลือกไม่ให้ operation นี้มีความหมายโดยตรง

นี่เป็นตัวอย่างของ Language Design ที่

```text
เสียความสะดวก
       ↓
เพื่อเพิ่มความชัดเจน
       ↓
และลดโอกาสเกิดข้อผิดพลาด
```

---

## 6. Type Safety

Rust แยก

```text
String
&str
Vec<u8>
char
```

ออกจากกัน

ทำให้ Compiler สามารถตรวจสอบได้ว่า Operation ที่ใช้เหมาะสมกับ Type หรือไม่

โดยเฉพาะ

```text
Vec<u8>
```

สามารถเก็บ Byte ใด ๆ

แต่

```text
String
```

ต้องรักษา Valid UTF-8

นี่เป็นตัวอย่างว่า Type System สามารถช่วยรักษา Invariant ของข้อมูลได้

---

## 7. Resource Control

String และ Vec มี

```text
Length
Capacity
Dynamic Buffer
Allocation
Reallocation
Deallocation
```

Rust จึงเปิดให้โปรแกรมเมอร์ควบคุม Resource ได้มากกว่าภาษาที่ซ่อน Memory Management ไว้ทั้งหมด

แต่ยังคงมี Safety จาก Ownership และ Borrow Checker

---

## 8. Concurrency Safety

Ownership และ Borrowing ยังมีผลกับ Concurrent Programming

Safe Rust ใช้ Type System และ Borrow Checker ช่วยป้องกัน Data Race

ดังนั้นแนวคิดที่เริ่มจาก String และ Collection สามารถเชื่อมไปยังระบบ Concurrent ของภาษาได้

---

## 9. Trade-off ของ Rust

Rust ไม่ได้ทำให้ทุกอย่างง่ายที่สุด

สิ่งที่ Rust แลกคือ

```text
เรียนรู้ยากขึ้นในช่วงแรก
        ↓
ต้องเข้าใจ
Ownership
Borrowing
Lifetime
Type System
        ↓
แลกกับ
Memory Safety
Type Safety
Resource Control
```

ดังนั้นการออกแบบภาษาเป็นการแลกเปลี่ยนระหว่าง

```text
Performance
Safety
Ease of Use
Control
```

ไม่มีภาษาหนึ่งที่ได้ทุกอย่างโดยไม่มี Trade-off

---

## 10. Rust vs. Other Language

**Comparison Language:** `[Python / C / C++ / Java /]`

---
10.1 Rust vs Python
---

| Aspect |  Rust |  Python |
| :--- | :--- | :--- |
| **Syntax** | ใช้ `Vec<T>` (list ขยายได้), `String` (ข้อความที่เราเป็นเจ้าของ) และ `&str` (ยืมดูข้อความ)<br>ต้องเขียน `mut` ถึงจะแก้ไขได้ | ใช้ `list` (เขียนด้วย `[]`) และ `str`<br>ไม่ต้องบอก type และไม่มีคำว่า `mut` ทุกตัวแปรแก้ไขได้เท่ากันหมด (ยกเว้น `str` ที่แก้ตรง ๆ ไม่ได้) |
| **Semantics / Behavior** | **Move:** `let b = a;` คือยกความเป็นเจ้าของให้ `b` แล้ว `a` ใช้ต่อไม่ได้<br>ถ้าแค่อยากดู ใช้ `&a` (ยืม) | **Reference:** `b = a` คือมีป้ายชื่อสองป้ายชี้ไปที่ของชิ้นเดียวกัน ใช้ได้ทั้งคู่<br>ถ้าเป็น `list` แก้ผ่าน `b` แล้ว `a` ก็เห็นการเปลี่ยนแปลงด้วย |
| **Type System** | **Static + strong:** รู้ชนิดของทุกตัวแปรก่อนรัน ใส่ผิดชนิดไม่ยอม compile<br>`Vec<i32>` เก็บได้เฉพาะ `i32` | **Dynamic:** รู้ชนิดตอนรันเท่านั้น<br>`list` ใส่ปนชนิดได้ เช่น `[1, "a", 3.5]` ถ้าใช้ผิดชนิดจะรู้ตอนรัน (`TypeError`) |
| **Memory Management** | **Ownership + `Drop`:** คืนความจำอัตโนมัติเมื่อเจ้าของออกจาก scope ตรวจตอน compile **ไม่มี GC** | **Reference counting + Garbage Collector** (ใน CPython) ทำงานตอนรัน ผู้เขียนไม่ต้องคิดเรื่องนี้เลย แต่มี overhead |
| **Safety** | กัน use-after-free, การแก้ list ระหว่าง loop และ data race ตั้งแต่ compile<br>`v[10]` เกินขอบเขต = **panic** (หยุดโปรแกรมทันที ไม่อ่านค่าขยะ) | ไม่มีปัญหาความจำเสียหาย<br>แต่ error หลายอย่างโผล่ตอนรัน เช่น `IndexError`, `TypeError` |

---
10.2 Rust vs C
---

| Aspect |  Rust |  C |
| :--- | :--- | :--- |
| **Syntax** | ใช้ `Vec<T>` และ `String` ที่ภาษามีให้ในตัว<br>ต้องเขียน `mut` ถึงจะแก้ไขได้ | **ไม่มี** list ขยายได้หรือ string type ในตัว ต้องใช้ array ขนาดตายตัว หรือขอความจำเองด้วย `malloc` / `realloc`<br>string คือ `char[]` ที่ต้องปิดท้ายด้วย `\0` |
| **Semantics / Behavior** | **Move:** `let b = a;` ยกความเป็นเจ้าของ แล้ว `a` ใช้ต่อไม่ได้ | **Copy:** assign ตัวแปรธรรมดาคือคัดลอกค่า<br>แต่ถ้าเป็น pointer จะคัดลอกแค่ "ที่อยู่" ทำให้สอง pointer ชี้ที่เดียวกันได้ โดยไม่มีใครตรวจว่าใครเป็นเจ้าของ |
| **Type System** | **Static + strong:** ไม่แปลงชนิดตัวเลขให้เอง ต้องเขียน `as` ชัดเจน<br>`&str` และ slice `&[T]` **พกความยาวไปด้วยเสมอ** | **Static แต่หลวม (weak typing):** แปลงชนิดกันได้ง่าย (implicit conversion, cast, `void*`)<br>array พอส่งเข้าฟังก์ชันจะกลายเป็น pointer และ **ไม่รู้ความยาวของตัวเอง** |
| **Memory Management** | คืนความจำอัตโนมัติเมื่อเจ้าของออกจาก scope (`Drop`) ผู้เขียนไม่ต้องเรียก `free` | ผู้เขียนต้อง `malloc` และ `free` เองด้วยมือทุกครั้ง<br>ลืมคืน = memory leak, คืนซ้ำ = double free, ใช้หลังคืน = use-after-free |
| **Safety** | ตรวจขอบเขตตอนรัน (เกินขอบเขต = panic) และตรวจ ownership ตอน compile<br>จะหลุดจากการตรวจได้ก็ต่อเมื่อเขียนใน `unsafe` เองเท่านั้น | **ไม่ตรวจขอบเขตเลย** เช่น `strcpy` ใส่ข้อความยาวเกินบัฟเฟอร์ = buffer overflow<br>เป็น undefined behavior ที่อาจทำงานผิดเงียบ ๆ โดยไม่ error |

---

---
10.3 Rust vs C++
---

| Aspect |  Rust |  C++ |
| :--- | :--- | :--- |
| **Syntax** | `Vec<T>`, `String`, `&str`<br>**ห้ามแก้เป็นค่าเริ่มต้น** ต้องเขียน `mut` ถึงแก้ได้ | `std::vector<T>`, `std::string` และ `std::string_view` (คล้าย `&str`) ต้อง `#include`<br>**แก้ได้เป็นค่าเริ่มต้น** ถ้าจะห้ามแก้ต้องเขียน `const` (ตรงข้ามกับ Rust) |
| **Semantics / Behavior** | **Move เป็นค่าเริ่มต้น:** `let b = a;` ยกของให้ `b` และ compiler **ห้าม** ใช้ `a` ต่อ | **Copy เป็นค่าเริ่มต้น:** `b = a` คัดลอกข้อมูลทั้งชุด<br>ถ้าอยากย้ายต้องเรียก `std::move(a)` เอง และหลังย้าย `a` ยังเรียกใช้ได้ (อยู่ในสถานะ "ใช้ได้ แต่ค่าไม่แน่นอน") compiler ไม่ห้าม |
| **Type System** | **Static + strong:** generics ระบุเงื่อนไขไว้ที่ trait bound ตั้งแต่ประกาศ เช่น key ของ `HashMap` ต้องเป็น `Eq + Hash` | **Static:** template ตรวจเงื่อนไขของ type ตอนใช้จริง ข้อความ error มักยาวและอ่านยาก (C++20 เพิ่ม concepts มาช่วย)<br>ยังมี implicit conversion และ cast แบบเก่าที่เสี่ยงผิดพลาด |
| **Memory Management** | ใช้ **RAII** ผ่าน `Drop` และ **บังคับ** ด้วยระบบ Ownership ทุกกรณี | ใช้ **RAII** ผ่าน destructor เหมือนกัน แต่ **ไม่บังคับ** ผู้เขียนยังใช้ `new` / `delete` เองได้ และ smart pointer เป็นแค่ทางเลือก |
| **Safety** | `v[i]` เกินขอบเขต = panic เสมอ<br>Borrow Checker กันการแก้ list ระหว่าง loop และ `&str` ที่ชี้ของที่ถูกทำลายแล้ว ตั้งแต่ compile | `v[i]` **ไม่ตรวจขอบเขต** (undefined behavior) ต้องใช้ `.at(i)` ถึงจะตรวจ<br>`push_back` ระหว่าง loop ทำให้ iterator ใช้ไม่ได้ และ `string_view` ชี้ของที่ถูกทำลายแล้วได้ |

---
10.4 Rust vs Java
---

| Aspect |  Rust |  Java |
| :--- | :--- | :--- |
| **Syntax** | `Vec<T>`, `String`, `&str`<br>เก็บ `i32` ลง `Vec<i32>` ได้ตรง ๆ | `ArrayList<T>` และ `String`<br>generics ใช้ได้เฉพาะ object จึงต้องใช้ `Integer` แทน `int` (autoboxing)<br>ไม่มี `mut` — `final` กันแค่การ "ชี้ไปที่อื่น" ไม่ได้กันการแก้ข้อมูลข้างใน |
| **Semantics / Behavior** | **Move:** `let b = a;` ยกความเป็นเจ้าของ แล้ว `a` ใช้ต่อไม่ได้ | **Reference:** `String b = a;` สองตัวแปรชี้ object เดียวกัน ใช้ได้ทั้งคู่<br>`ArrayList` แก้ผ่านตัวหนึ่ง อีกตัวก็เห็นด้วย ส่วน `String` แก้ตรง ๆ ไม่ได้ (immutable) |
| **Type System** | **Static + strong:** generics สร้างโค้ดแยกให้แต่ละชนิด (monomorphization)<br>**ไม่มี null** ใช้ `Option<T>` แทน ("มีค่า" หรือ "ไม่มีค่า") | **Static + strong:** generics แบบ **type erasure** (ตอนรัน JVM ไม่รู้ว่าเป็น `ArrayList<Integer>` หรือ `ArrayList<String>`)<br>ตัวแปร reference เป็น `null` ได้ |
| **Memory Management** | คืนทันทีที่เจ้าของออกจาก scope จึง **คาดเดาเวลาได้แน่นอน** ไม่มี GC | **Garbage Collector ของ JVM** เก็บกวาดตอนรัน ไม่รู้ว่าของจะถูกคืนเมื่อไหร่ และอาจมี GC pause |
| **Safety** | จับการแก้ list ระหว่าง loop ได้ตอน compile<br>กัน data race ตอน compile ผ่าน Borrow Checker | ไม่มีปัญหาความจำเสียหาย แต่ error ตอนรันมีหลายแบบ เช่น `NullPointerException`, `IndexOutOfBoundsException`, `ConcurrentModificationException` (แก้ list ระหว่าง loop)<br>data race ป้องกันตอน compile ไม่ได้ ผู้เขียนต้องระวังเอง |

---


### Rust Example

```rust

fn main() {
    // Vec<T>: เหมือน list ที่ขยายขนาดได้เรื่อย ๆ
    let mut numbers: Vec<i32> = Vec::new();
    numbers.push(1);
    numbers.push(2);
    numbers.push(3);

    // ยืมดู (borrow) เพื่ออ่านค่า ไม่ได้เอาไปเป็นของตัวเอง
    let total: i32 = numbers.iter().sum();
    println!("Sum = {}", total);

    // String: ข้อความที่เราเป็นเจ้าของเต็มตัว แก้ไข/ต่อได้
    let mut name = String::from("Rust");
    name.push_str(" Language");

    // &str: แค่ "มองไปที่" ส่วนหนึ่งของ String เท่านั้น (view)
    let slice: &str = &name[0..4];
    println!("{} | {}", name, slice);

    // Move semantics: ยกความเป็นเจ้าของจาก name ไปให้ name2
    let name2 = name; // ตั้งแต่นี้ name ใช้งานต่อไม่ได้แล้ว (compile error ถ้าเรียก name อีก)
    println!("{}", name2);
}

```

### `[Other Language]` Example

> python
``` 
def main():
    # list: เหมือน Vec ในภาษา Rust ใส่ของเพิ่มได้เรื่อย ๆ
    numbers = []
    numbers.append(1)
    numbers.append(2)
    numbers.append(3)

    total = sum(numbers)
    print(f"Sum = {total}")

    # str: ข้อความที่แก้ไขไม่ได้เลย (immutable)
    name = "Python"
    name = name + " Language"  # จริง ๆ คือสร้าง object ใหม่ทั้งชิ้น เพราะ str แก้ไขตรง ๆ ไม่ได้

    slice_ = name[0:6]
    print(f"{name} | {slice_}")

    # Reference semantics: name2 แค่ "ชี้" ไปที่ object เดียวกับ name
    name2 = name  # name ยังใช้งานได้ตามปกติ ไม่มีการยึดไปเป็นของ name2 ฝ่ายเดียว
    print(name2)


main()
```

> C
``` 
#include <stdio.h>
#include <string.h>
#include <stdlib.h>

int main(void) {
    // C ไม่มี dynamic array ให้ในตัว ต้องขอความจำเองด้วย malloc
    int *numbers = malloc(3 * sizeof(int));
    numbers[0] = 1;
    numbers[1] = 2;
    numbers[2] = 3;

    int total = 0;
    for (int i = 0; i < 3; i++) {
        total += numbers[i];
    }
    printf("Sum = %d\n", total);

    // string ใน C คือ char array ที่จบด้วย \0 ต้องจองที่ให้พอกับข้อความสุดท้าย
    char name[20];
    strcpy(name, "C");
    strcat(name, " Language");

    char slice[2];
    strncpy(slice, name, 1);
    slice[1] = '\0';
    printf("%s | %s\n", name, slice);

    // ค่าตัวแปรถูก copy ตรง ๆ ไม่มีแนวคิดเจ้าของแบบ Rust
    char name2[20];
    strcpy(name2, name); // name ยังใช้งานต่อได้ตามปกติ เพราะเป็นการ copy ข้อมูลจริง
    printf("%s\n", name2);

    free(numbers); // ต้องคืนความจำเองด้วยมือ ลืมคืนคือ memory leak
    return 0;
}
```

> C++
``` 
#include <iostream>
#include <vector>
#include <string>
#include <numeric>

int main() {
    // std::vector: dynamic array ที่จัดการความจำให้อัตโนมัติ (คล้าย Vec ของ Rust)
    std::vector<int> numbers;
    numbers.push_back(1);
    numbers.push_back(2);
    numbers.push_back(3);

    int total = std::accumulate(numbers.begin(), numbers.end(), 0);
    std::cout << "Sum = " << total << std::endl;

    // std::string: จัดการความจำให้อัตโนมัติเหมือน String ของ Rust
    std::string name = "C++";
    name += " Language";

    std::string slice = name.substr(0, 3); // ดึงข้อความส่วนหนึ่งออกมา (เป็นการ copy)
    std::cout << name << " | " << slice << std::endl;

    // ค่าเริ่มต้นคือ copy semantics: name2 ได้ข้อมูลชุดใหม่ที่แยกจาก name
    std::string name2 = name; // name ยังใช้งานต่อได้ตามปกติ เพราะถูก copy ไม่ใช่ move
    std::cout << name2 << std::endl;

    return 0;
}
```

> Java
```
import java.util.ArrayList;

public class Main {
    public static void main(String[] args) {
        // ArrayList: dynamic array ที่ Garbage Collector จัดการความจำให้
        ArrayList<Integer> numbers = new ArrayList<>();
        numbers.add(1);
        numbers.add(2);
        numbers.add(3);

        int total = 0;
        for (int n : numbers) {
            total += n;
        }
        System.out.println("Sum = " + total);

        // String ใน Java เป็น immutable object เหมือนใน Python
        String name = "Java";
        name = name + " Language"; // สร้าง object ใหม่ทั้งชิ้น เพราะ String แก้ไขตรง ๆ ไม่ได้

        String slice = name.substring(0, 4);
        System.out.println(name + " | " + slice);

        // Reference semantics: name2 ชี้ไปที่ object เดียวกับ name
        String name2 = name; // name ยังใช้งานได้ตามปกติ ไม่มีการยึดไปฝ่ายเดียว
        System.out.println(name2);
    }
}
```

### Analysis

จากการเปรียบเทียบ Rust กับ Python, C, C++ และ Java ในหัวข้อ Collection & String ความแตกต่างสำคัญทั้งหมดสรุปได้จาก **4 คำถามหลัก** (หน่วยความจำ, Semantics, Type System, และช่วงเวลาที่ตรวจพบข้อผิดพลาด) โดยใช้ `Vec`, `String` และ `&str` เป็นตัวอย่าง
 
#### คำถามที่ 1: ใครเป็นผู้คืนหน่วยความจำของ `Vec` และ `String`?
 
   ทุกภาษาเก็บข้อมูลของ list และข้อความไว้บน heap เหมือนกัน สิ่งที่ต่างคือ **"ใครเป็นคนคืน"**
     
   - **C** — ผู้เขียนเรียก `free` เองด้วยมือ ลืมคือ memory leak คืนซ้ำหรือใช้หลังคืนคือบั๊กร้ายแรง
   - **Python และ Java** — Garbage Collector ตรวจและคืนให้ตอนรัน สะดวกแต่มี overhead และเวลาคืนคาดเดาไม่ได้
   - **C++** — ใช้ RAII (destructor ของ `std::vector` / `std::string` คืนให้เมื่อออกจาก scope) แต่ยังใช้ `new` / `delete` เองได้ จึง **ไม่บังคับ** ให้ปลอดภัย
   - **Rust** — ใช้ Ownership ร่วมกับ `Drop` คืนอัตโนมัติเมื่อเจ้าของออกจาก scope และตรวจทุกอย่างตอน compile จึง **ไม่ต้องมี GC**
     ในเชิงกลไก C++ กับ Rust ใกล้กันที่สุด (ทั้งคู่ใช้ RAII) ต่างกันที่ Rust **บังคับ** ด้วยระบบภาษา ส่วน C++ ปล่อยให้ผู้เขียนเลือก
     
#### คำถามที่ 2: `b = a` แล้วเกิดอะไรขึ้นกับข้อมูล?
 
  จุดนี้คือความต่างที่ชัดที่สุดของ Rust
     
   - **C / C++ (ค่าเริ่มต้น):** คัดลอกข้อมูล ได้สองชุดแยกกัน (C ที่เป็น pointer จะคัดลอกแค่ที่อยู่)
   - **Python / Java:** สองตัวแปรชี้ไปที่ object เดียวกัน ใช้ได้ทั้งคู่
   - **Rust:** **ย้ายความเป็นเจ้าของ** (Move) ตัวแปรเดิมใช้ต่อไม่ได้ ถ้าฝืนใช้ compiler ไม่ยอมให้ผ่าน
     Rust เลือกแบบนี้เพื่อ **รับประกันว่าข้อมูลหนึ่งชิ้นมีเจ้าของได้คนเดียวเสมอ** เมื่อรู้ว่าใครเป็นเจ้าของ compiler ก็รู้ว่าใครต้องเป็นคนคืนหน่วยความจำ และไม่มีทางคืนซ้ำสองครั้ง
 
#### คำถามที่ 3: ตรวจชนิดข้อมูลอย่างไร และ generics ทำงานอย่างไร? (Type System)
 
   `Vec<T>` และ `ArrayList<T>` ต้องรองรับ "ชนิดอะไรก็ได้" แต่แต่ละภาษาทำต่างกัน
     
| ภาษา | การตรวจชนิด | Generics ทำงานอย่างไร |
| :--- | :--- | :--- |
| **Rust** | Static + strong ตรวจตอน compile | **Monomorphization** สร้างโค้ดแยกให้แต่ละชนิด ไม่มีค่าปรับตอนรัน และต้องระบุเงื่อนไขด้วย trait bound |
| **C++** | Static ตรวจตอน compile | **Template** สร้างโค้ดแยกให้แต่ละชนิดเช่นกัน แต่ตรวจเงื่อนไขตอนใช้จริง |
| **Java** | Static + strong ตรวจตอน compile | **Type erasure** ตอนรัน JVM ไม่รู้ชนิดที่ใส่ใน `<T>` |
| **Python** | Dynamic ตรวจตอนรัน | ไม่ต้องมี generics เพราะ list ใส่ชนิดอะไรก็ได้ (duck typing) |
| **C** | Static แต่หลวม (weak) | ไม่มี generics ต้องใช้ `void*` หรือ macro แลกกับการสูญเสียความปลอดภัยของ type |
     
   Rust จึงได้ทั้ง *ความเร็วแบบ C++* (สร้างโค้ดแยกต่อชนิด) และ *ความปลอดภัยของ type แบบ Java* (ตรวจตอน compile) โดยเพิ่ม trait bound ให้เงื่อนไขชัดเจนตั้งแต่ประกาศ
 
#### คำถามที่ 4: รู้ว่าโค้ดผิดตอนไหน?
 
   ลองดูตัวอย่างที่พบบ่อยของ Collection คือ **การแก้ list ขณะกำลัง loop อยู่**
     
| ภาษา | ผลลัพธ์ | รู้ตอนไหน |
| :--- | :--- | :--- |
| **C** | ไม่มีการตรวจ พฤติกรรมไม่แน่นอน | ไม่รู้ / อาจผิดเงียบ ๆ |
| **C++** | iterator ใช้ไม่ได้ (undefined behavior) | ไม่รู้ / อาจผิดเงียบ ๆ |
| **Python** | ไม่มี error แต่ผลเพี้ยน (เช่น ข้ามสมาชิก หรือวนไม่จบ) | รู้เมื่อผลผิดตอนรัน |
| **Java** | `ConcurrentModificationException` | ตอนรัน |
| **Rust** | compile error | **ก่อนรัน** |
     
   ส่วน "การเข้าถึงช่องที่ไม่มีอยู่" (เช่น `v[10]` ทั้งที่มีของ 3 ชิ้น) Rust จะ **panic** ตอนรัน คือหยุดโปรแกรมทันที ซึ่งต่างจาก C/C++ ที่อ่านค่าขยะแล้วทำงานต่อโดยไม่รู้ตัว
 
#### ข้อควรระวัง: Rust ไม่ได้ "ไร้ข้อผิดพลาดทุกกรณี"
 
   เพื่อความถูกต้องทางวิชาการ ควรระบุว่า Rust รับประกัน **memory safety** ในส่วนที่เป็น safe Rust เท่านั้น ไม่ได้แปลว่าไม่มี error เลย เช่น การเข้าถึงเกินขอบเขตยัง panic ตอนรัน, โค้ดใน `unsafe` หลุดจากการตรวจได้, และ memory leak ยังเกิดได้ (เช่น การอ้างอิงวนกันด้วย `Rc`)
 
#### บทสรุปด้านเหตุผลของการออกแบบภาษา
 
   การออกแบบภาษาต้องแลกระหว่าง **ความเร็ว**, **ความปลอดภัย** และ **ความง่ายในการเขียน**
     
   - ภาษาที่ให้ผู้เขียนคุมความจำเอง (**C, C++**) → เร็ว แต่เสี่ยงบั๊กสูง
   - ภาษาที่ให้ Garbage Collector ดูแล (**Python, Java**) → เขียนง่ายและปลอดภัยจากบั๊กความจำ แต่ช้ากว่าและตรวจพบปัญหาตอนรัน
   - **Rust** → ผลักภาระการตรวจสอบไปไว้ที่ **compile time** ผ่าน Ownership และ Borrow Checker เพื่อให้ได้ทั้ง **ความเร็วระดับ C/C++** และ **ความปลอดภัยด้านความจำระดับใกล้เคียง Python/Java**
     ราคาที่ต้องจ่ายคือ **ผู้เขียนต้องเรียนรู้แนวคิด Ownership และ Borrowing** ซึ่งไม่เหมือนภาษาใดมาก่อน จึงมักรู้สึกว่า Rust "เขียนยากในช่วงแรก" แต่แลกมาด้วยโปรแกรมที่ผ่านการ compile แล้วมั่นใจได้มากกว่า
     
---

## 11. Teach Your Topic

การนำเสนอมีสมาชิก **4 คน คนละประมาณ 5 นาที**

| Member   | Responsibility                          |  Time |
| -------- | --------------------------------------- | ----: |
| Member 1 | Concept + Short Code Illustration       | 5 min |
| Member 2 | Detailed Code + Live Demo               | 5 min |
| Member 3 | Rust vs Other Language + PPL Analysis   | 5 min |
| Member 4 | Exercises + Common Mistakes + Challenge | 5 min |

### Individual Contribution

**Member 1**

`[อธิบาย Concept + Short Code ของ Collection & String ได้แก่ Array, Tuple, Vector, String และ &str พร้อมตัวอย่างการใช้งานพื้นฐาน]`

**Member 2**

`[อธิบาย Detailed Code + Live Demo โดยสาธิต Example 1,2 ประกอบการอธิบายการทำงานของ Array, Tuple, Vec, String, &str, Ownership และ Borrowing]`

**Member 3**

`[เปรียบเทียบความแตกต่าง Rust vs Other Language + PPL Analysis โดยวิเคราะห์ Syntax, Semantics, Type System และ Memory / Resource Management รวมถึงเปรียบเทียบ Rust กับภาษาอื่น]`

**Member 4**

`[รับผิดชอบ Exercises + Common Mistakes + Challenge โดยอธิบายปัญหา Ownership/Borrowing, การแก้ไข Collection ระหว่าง Loop และนำเสนอแบบฝึกหัดที่เกี่ยวข้องกับ]`

> สมาชิกทุกคนต้องสามารถอธิบาย Code ของกลุ่มได้ ไม่ใช่เฉพาะส่วนที่ตนเองเขียน

---

## 12. References

> แนะนำให้มีอย่างน้อย **4 แหล่งอ้างอิง** และควรใช้เอกสารทางการเป็นหลัก

1. `[https://doc.rust-lang.org/book/ch08-00-common-collections.html?utm_source=chatgpt.com]`
2. `[https://doc.rust-lang.org/stable/book/ch08-02-strings.html?highlight=String&utm_source=chatgpt.com]`
3. `[https://doc.rust-lang.org/std/collections/]`
4. `[https://doc.rust-lang.org/std/primitive.str.html]`
5. `[https://doc.rust-lang.org/std/collections/hash_set/struct.HashSet.html]`

---

## 13. AI Usage Declaration

สามารถใช้ AI เป็นเครื่องมือช่วยเรียนรู้และพัฒนาได้ แต่สมาชิกทุกคนต้องเข้าใจและสามารถอธิบายผลงานของกลุ่มได้

| AI Tool | Purpose | How the Result Was Verified |
|---|---|---|
| `ChatGPT` | ตรวจสอบ Rust Syntax และหลักการเขียนให้ถูกต้องตาม Rust Official Documentation รวมถึงศึกษา Collection, String, Ownership และ Borrowing | Compile และ Run Code จริง และตรวจสอบกับ Rust Official Documentation |
| `Gemini` | ศึกษา Git/GitHub และใช้ค้นคว้าเพื่อเปรียบเทียบแนวคิดของ Rust กับภาษาโปรแกรมอื่น ๆ | ทดลองใช้ Git/GitHub จริง และตรวจสอบข้อมูลกับเอกสารของแต่ละภาษา |
| `Claude` | ช่วยเขียนและปรับปรุง Rust Code สำหรับตัวอย่างและการทำงานของโปรเจกต์ | Compile และ Run Code จริง ตรวจสอบผลลัพธ์และทดสอบการทำงานของ Code |
### Declaration

- [x] Code ทุกส่วนที่นำเสนอได้รับการ Compile และทดสอบแล้ว
- [x] สมาชิกทุกคนสามารถอธิบาย Code ที่นำเสนอได้
- [x] ตรวจสอบข้อมูลจากแหล่งอ้างอิงที่น่าเชื่อถือแล้ว
- [x] ระบุการใช้ AI อย่างโปร่งใส

**รายละเอียดการใช้ AI**

`[พวกเราอ่านและทำเนื้อหาส่วนของตัวเองจาก Rust Official Documentation เป็นหลัก แล้วใช้ AI ช่วยบางจุด ChatGPT ใช้เช็ก syntax และช่วยทำความเข้าใจ ใช้Gemini ใช้หาข้อมูลเรื่อง Git/GitHub กับเทียบ Rust กับภาษาอื่น, Claude ใช้ช่วย [เช่น ปรับโค้ดตัวอย่าง / ดู error] เทียบกับเอกสารของRust และให้ช่วยในการตรวจสอบความเรียบร้อยของงาน error ต่างๆ]`

---

## 14. GitHub Contribution

| Member   |    Issues |   Commits | Pull Requests | Code Reviews | Contribution   |
| -------- | --------: | --------: | ------------: | -----------: | -------------- |
| Member 1 | `[0]` | `[15]` |     `[7]` |    `[0]` | `[Concept + Short Code Illustration]` |
| Member 2 | `[0]` | `[23]` |     `[8]` |    `[0]` | `[Detailed Code + Live Demo   ]` |
| Member 3 | `[0]` | `[23]` |     `[5]` |    `[0]` | `[Rust vs Other Language + PPL Analysis]` |
| Member 4 | `[0]` | `[7]` |     `[3]` |    `[0]` | `[Exercises + Common Mistakes + Challenge]` |

### Teamwork Reflection

**How did your team collaborate?**

`[เราแบ่งงานกันตามหน้าที่ที่ตกลงไว้ตั้งแต่แรก แต่ละคนอ่านและทำส่วนของตัวเองโดยดูจาก Rust Official Documentation เป็นหลัก แล้วใช้ GitHub รวมงานของทุกคนไว้ที่เดียว ใครทำเสร็จก็ Commit แล้วเปิด Pull Request เข้า Main และทุกครั้งก่อนเริ่มทำต่อจะ Pull งานล่าสุดมาก่อน งานจะได้ไม่ทับกัน ระหว่างทำ เราช่วยกันเช็กว่าเนื้อหาของแต่ละส่วนตรงกันไหม เช่น ความหมายของ Vec, String, &str และเรื่อง Ownership กับ Borrowing ที่ใช้ซ้ำหลายจุด ส่วนโค้ดทุกตัวอย่างเรารันจริงก่อนใส่ลงเอกสาร และพยายามให้ทุกคนอ่านโค้ดของกลุ่มให้เข้าใจทั้งหมด ไม่ใช่แค่ส่วนของตัวเอง]`

**Problems encountered**

`[ปัญหาหลักคือ Rust เป็นภาษาที่ทุกคนในกลุ่มยังไม่เคยเขียนมาก่อน เลยต้องใช้เวลากับ Syntax พอสมควร โดยเฉพาะเรื่อง Ownership กับ Borrowing ที่ต่างจากภาษาอื่นที่เคยเรียนมา รวมถึงการแยกให้ออกว่า String กับ &str ใช้ต่างกันยังไง อีกเรื่องคือ GitHub เพราะหลายคนยังไม่คุ้นกับการ Pull จาก Main, Commit และ Push งานกลับเข้าไป ช่วงแรกเลยมีงงว่าควรอัปเดตไฟล์ตอนไหน]`



**How did you solve them?**

`[เราแบ่งกันศึกษาตามหน้าที่ของแต่ละคน โดยดูจาก Rust Official Documentation เป็นหลัก แล้วนำมาเทียบกันในแต่ละ Part ให้เข้าใจตรงกัน ส่วน GitHub ก็ลองใช้จริงทั้ง Pull, Commit และ Push จนคล่องขึ้น ส่วนปัญหาตอนรันโค้ด เราอ่านข้อความ error ของ compiler ซึ่งบอกบรรทัดที่ผิดชัดเจน แล้วแก้ทีละจุดจนคอมไพล์ผ่าน ช่วยกันศึกษาหาความรู้เพิ่มเติมเกี่ยวกับการใช้ GitHub เพื่อให้ทำงานร่วมกันได้]`

---

## 15. Final Checklist

- [x] Learning Objectives ครบ 3–4 ข้อ
- [x] Key Concepts ครบถ้วน
- [x] Syntax / Rules
- [x] Runnable Code Examples
- [x] Code Compile และ Run ได้จริง
- [x] Common Mistakes
- [x] Exercises 2 ข้อ พร้อม Solutions
- [x] PPL Perspective
- [x] Rust vs Other Language
- [x] References อย่างน้อย 4 แหล่ง
- [x] AI Usage Declaration
- [x] GitHub Contribution
- [x] สมาชิกทั้ง 4 คนมีส่วนร่วม
- [x] สมาชิกทั้ง 4 คนพร้อมนำเสนอคนละ 5 นาที
- [x] สมาชิกทุกคนสามารถอธิบาย Code ของกลุ่มได้

---

## Submission Information

**Repository:** `[https://github.com/670710146/collection-string]`

**Chapter Path:** `[10-collections-strings]`

**Final PR:** `#[PR number #24]`

**Submitted by:** `[Group 10]`

**Date:** `[2026-10-03]`
