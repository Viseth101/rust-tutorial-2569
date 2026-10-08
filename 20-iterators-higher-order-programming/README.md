# Rust Tutorial Project — Principles of Programming Languages

> **กลุ่มที่:** 20
> **Topic No.:** 20
> **Topic Name:** Iterators & Higher-Order Programming
> **ประเด็นหลักที่ควรครอบคลุม:** iter(), map, filter, fold, collect, lazy evaluation

---

## 1. Members

| # | Name | Student ID | GitHub Username | Main Responsibility |
|---|---|---|---|---|
| 1 | นายณัฐชนน รักวงศ์ | 670710646 | `@670710646` | Concept + Short Code Illustration (สรุปแนวคิดหลัก + โค้ดตัวอย่างสั้น) |
| 2 | นางสาวณัฐฐาพร เสตะวีระ | 670710647 | `@670710647` | Detailed Code + Live Demo (โค้ดเชิงลึก + สาธิตสด) |
| 3 | นายณัฐดนัย ศรีไวย | 670710648 | `@670710648` | Rust vs Other Language + PPL Analysis (เปรียบเทียบภาษา + วิเคราะห์เชิง PPL) |
| 4 | นายพรรสวกร สมใจ | 670710651 | `@670710651` | Exercises + Common Mistakes + Challenge (แบบฝึกหัด + ข้อผิดพลาดที่พบบ่อย + คำถามท้าทาย) |

> แก้ไข GitHub Username ของแต่ละคนให้ตรงกับบัญชีจริงก่อนเริ่มทำงาน (ผู้สอนจะใช้คอลัมน์นี้เชิญเป็น collaborator ของ repository)

---

## 2. Learning Objectives

หลังจากศึกษา Topic นี้แล้ว ผู้เรียนสามารถ:

1. `[อธิบายแนวคิดสำคัญได้]`
2. `[เขียนโปรแกรม Rust ที่เกี่ยวข้องได้]`
3. `[วิเคราะห์พฤติกรรม/กฎของภาษาได้]`
4. `[เปรียบเทียบ Rust กับภาษาอื่นได้]`

---

## 3. Introduction

อธิบายว่า Topic นี้คืออะไร มีความสำคัญอย่างไร และใช้แก้ปัญหาอะไรในการเขียนโปรแกรม

`Iterators คือ interface ที่ใช้ดูข้อมูลใน collection/array ทีละตัว มีคุณสมบัติ lazy evaluation โดยจะไม่ทำงานจนกว่าจะต้องส่งผลลัพธ์ ช่วยประหยัดหน่วยความจำ ส่วน High-order programming ฟังก์ชันที่สามารถรับฟังก์ชันอื่นเป็น Argument หรือส่งคืนฟังก์ชันอื่นเป็นผลลัพธ์ได้ นิยมใช้กับเมธอดอย่าง map, filter, และ fold เพื่อประมวลผลข้อมูลใน Vector หรือ Iterator โดยไม่ต้องใช้ลูป for `

---

## 4. Key Concepts

### 4.1 `iter()`

**คำอธิบาย**

`สร้างตัววนซ้ำ (Iterator) โดยขอยืมอ่านข้อมูล โดยส่งคืนค่าเป็น Reference`

**ตัวอย่าง**

```rust
fn main() {
    let numbers = vec![1, 2, 3];

    for number in numbers.iter() {
        println!("{}", number);
    }
}
```

**Explanation**

`numbers.iter() สร้าง Iterator ที่ขอยืมข้อมูลจาก numbers`

---

### 4.2 `map()`

`เปลี่ยนแปลง (Transform) ข้อมูลทุกตัวใน Iterator โดยส่งคืนค่าเป็น Iterator ตัวใหม่ ในแต่ละรอบ number จะเป็น Reference (&i32) ที่ชี้ไปยังข้อมูลเดิม ดังนั้นข้อมูลใน numbers ยังคงสามารถใช้งานต่อได้หลังจากวนลูปเสร็จ`

```rust
fn main() {
    let numbers = vec![1, 2, 3];

    let doubled = numbers.iter().map(|number| number * 2);

    for number in doubled {
        println!("{}", number);
    }
}
```

**Explanation**

`map() นำข้อมูลแต่ละตัวใน Iterator ไปผ่าน Closure ที่กำหนด (|number| number * 2) หมายถึงนำแต่ละค่าไปคูณ 2 โดย map() จะคืนค่าเป็น Iterator ใหม่ และเป็น Lazy Iterator คือจะยังไม่ประมวลผลทันที จนกว่าจะมีการเรียกใช้งาน Iterator เช่น for loop`

---

### 4.3 `filter()`

`คัดกรองข้อมูลตามเงื่อนไข โดย Closure ต้องคืนค่าเป็น true หรือ false`

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    let even_numbers = numbers
        .iter()
        .filter(|number| **number % 2 == 0);

    for number in even_numbers {
        println!("{}", number);
    }
}
```

**Explanation**

`filter() ตรวจสอบข้อมูลทีละตัวด้วยเงื่อนไข (**number % 2 == 0) ถ้าได้ true จะเก็บค่านั้นไว้ถ้าได้ false จะไม่ส่งค่านั้นต่อไป`

---

### 4.4 `fold()`

`ยุบรวมข้อมูลทั้งหมดใน Iterator ให้เหลือค่าเดี่ยว โดยคืนค่าผลลัพธ์สุดท้ายชนิดเดียว`

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4];

    let sum = numbers
        .iter()
        .fold(0, |total, number| total + number);

    println!("{}", sum);
}
```

**Explanation**

`โดย fold() เริ่มต้นด้วยค่า 0 จากนั้นนำข้อมูลแต่ละตัวมาสะสมใน total (0 + 1 = 1, 1 + 2 = 3, 3 + 3 = 6, 6 + 4 = 10) ดังนั้นผลลัพธ์ = 10`

---

### 4.5 `collect()`

`รวบรวมข้อมูลจาก Iterator กลับไปเป็น Collection คืนค่าเป็น collection ใหม่ เช่น Vec, HashMap`

```rust
fn main() {
    let numbers = vec![1, 2, 3];

    let doubled: Vec<i32> = numbers
        .iter()
        .map(|number| number * 2)
        .collect();

    println!("{:?}", doubled);
}
```

**Explanation**

`map() สร้าง Iterator ใหม่จากข้อมูลเดิม เป็น (2, 4, 6) จากนั้น collect() จะดึงข้อมูลทั้งหมดจาก Iterator และรวบรวมกลับมาเป็น Vec<i32> ได้เป็น [2, 4, 6]`

---

## 5. Important Syntax / Rules

| Syntax / Rule | Meaning | Example |
|---|---|---|
| `.iter()` | สร้าง Iterator ที่ยืมข้อมูล และส่งคืนค่าเป็น Reference | `numbers.iter()` |
| `.map(\|x\| ...)` | Transform ข้อมูลแต่ละตัว และคืนค่าเป็น Iterator ใหม่ | `.map(\|x\| x * 2)` |
| `.filter(\|x\| ...)` | คัดกรองข้อมูล โดยเก็บเฉพาะค่าที่เงื่อนไขเป็น `true` | `.filter(\|x\| x % 2 == 0)` |
| `.fold(init, \|acc, x\| ...)` | รวมข้อมูลทั้งหมดให้เหลือค่าเดียว โดยมีค่าเริ่มต้น | `.fold(0, \|sum, x\| sum + x)` |
| `.collect()` | รวบรวมข้อมูลจาก Iterator กลับเป็น Collection | `.collect::<Vec<_>>()` |
| `Iterator` เป็น Lazy | จะยังไม่ประมวลผลข้อมูลทันที จนกว่าเราจะเรียกใช้ Iterator เช่น `collect()`, `for`, หรือ `next()` | `.map(...).collect()` |
| `collect()` ต้องรู้ชนิดปลายทาง | Rust ต้องรู้ว่าต้องการสร้าง Collection ชนิดใด | `let v: Vec<_> = iter.collect()` |
| `Closure` | ฟังก์ชันแบบไม่ต้องตั้งชื่อที่สามารถรับค่าและใช้ตัวแปรจาก scope ภายนอกได้ | `let add = \|a, b\| a + b;` |
| `i32` | ชนิดข้อมูลจำนวนเต็ม (integer) แบบมีเครื่องหมาย (signed) ขนาด 32 บิต | `let x: i32 = 100;`| 
| `Lazy Evaluation` | ประมวลผลเมื่อจำเป็นต้องใช้ผลลัพธ์ ช่วยลดการคำนวณที่ไม่จำเป็น | `iter.map(...).collect()` |

### Important Rules

1. `map()` และ `filter()` คืนค่าเป็น `Iterator` ใหม่ และยังไม่ประมวลผลทันที เพราะ Iterator ใน Rust เป็น Lazy
2. `iter()` จะยืมข้อมูล (`borrow`) ดังนั้นข้อมูลต้นฉบับยังสามารถใช้งานต่อได้
3. `collect()` ใช้ Consume Iterator และรวบรวมค่าที่ได้กลับเป็น Collection เช่น `Vec` หรือ `HashMap`
4. `fold()` จะ Consume Iterator และคืนค่าเป็นค่าเดี่ยว ไม่ใช่ Iterator
5. `map()` ใช้สำหรับ Transform ข้อมูล ส่วน `filter()` ใช้สำหรับคัดกรองข้อมูล และสามารถ Chain ต่อกันได้
6. `collect()` ต้องสามารถอนุมานได้ว่าต้องการ Collection ชนิดใด หาก Rust อนุมานไม่ได้ ต้องระบุ Type เช่น `Vec<_>`
7. Iterator สามารถ Chain หลาย operation ต่อกันได้ เช่น:
```rust
let result: Vec<i32> = numbers
    .iter()
    .map(|x| x * 2)
    .filter(|x| *x > 5)
    .collect();
```

---


## 6. Runnable Code Examples

> **ข้อกำหนด:** Code ทุกตัวต้อง Compile และ Run ได้จริงก่อนนำมาใส่ในเอกสาร

### Example 1 — ระบบคัดเลือกนักศึกษารับทุน (Scholarship system)

**Purpose:** เพื่อสาธิตการประมวลผลข้อมูลด้วย Rust Iterator โดยใช้ filter() ในการคัดเลือกข้อมูล, map() ในการแปลงข้อมูล และ collect() ในการรวบรวมผลลัพธ์ พร้อมแสดงแนวคิดของ Lazy Evaluation ในการทำงานของ Iterator 

```rust
fn main() {
    let scores = vec![35, 50, 68, 72, 90, 45, 80];

    println!("Original Scores: {:?}", scores);

    let scholarship_students = scores // ยังไม่คำนวณ สร้าง pipeline ไว้ก่อน
        .iter()
        .filter(|score| **score >= 60)
        .map(|score| {
            println!("Adding bonus to: {}", score);
            score + 5
        });

    //จุดที่เริ่มประมวลผลจริง
    let final_scores: Vec<i32> = scholarship_students.collect();

    println!("\n========= Scholarship Award Results =========");

    for score in &final_scores {
        println!("Student recieved scholarship | Final score: {}", score);
    }

    println!("\nTotal Scholarship Students: {}", final_scores.len());

    println!("========= Congratuations ! =========");
}

```

**Expected Output**

```text
Original Scores: [35, 50, 68, 72, 90, 45, 80]
Adding bonus to: 68
Adding bonus to: 72
Adding bonus to: 90
Adding bonus to: 80

========= Scholarship Award Results =========
Student recieved scholarship | Final score: 73
Student recieved scholarship | Final score: 77
Student recieved scholarship | Final score: 95
Student recieved scholarship | Final score: 85

Total Scholarship Students: 4
========= Congratuations ! =========
```

**Explanation**

โปรแกรมเริ่มต้นจากการรับข้อมูลคะแนนของนักศึกษา จากนั้นสร้าง Iterator เพื่อเข้าถึงข้อมูลทีละรายการ แล้วทำการคัดเลือกเฉพาะนักศึกษาที่มีคะแนนตั้งแต่ 
คะแนนขึ้นไปด้วย filter() หลังจากนั้นใช้ map() เพื่อเพิ่มคะแนนโบนัสให้กับนักศึกษาที่่ผ่านเกณฑ์ ในขั้นตอนนี้ Rust จะยังไม่ประมวลผลข้อมูลจริง เนื่องจาก Iterator ใช้หลักการ Lazy Evaluation ซึ่งจะรอจนกว่าจะมีการร้องขอผลลัพธ์ เมื่อเรียกใช้ collect() โปรแกรมจึงเริ่มประมวลผลข้อมูลทั้งหมดและรวบรวมผลลัพธ์ออกมาเป็น Vector ใหม่ที่เก็บคะแนนของนักศึกษาที่ได้รับทุนหลังเพิ่มโบนัส

---

### Example 2 — ระบบสั่งอาหาร (Food Order System)

**Purpose:** เพื่อสาธิตการประมวลผลข้อมูลด้วย Rust Iterator โดยใช้ filter() ในการคัดเลือกเมนูที่มีราคาไม่เกิน 200 บาท, map() ในการคำนวณราคาหลังส่วนลด และ fold() ในการรวมราคาของเมนูที่ผ่านเงื่อนไขทั้งหมด พร้อมแสดงแนวคิด Higher-Order Programming ผ่านการใช้งาน Closure

```rust
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

```

**Expected Output**

```text
================================
       FOOD ORDER SUMMARY
================================
Price Limit : 200 Baht
Discount    : 10%
--------------------------------
Burger | Original: 150 Baht | Final: 135 Baht | Selected
French Fries | Original: 80 Baht | Final: 72 Baht | Selected
Pizza | 250 Baht | Not Selected
Coke | Original: 50 Baht | Final: 45 Baht | Selected
Steak | 300 Baht | Not Selected
--------------------------------
Total Payment: 252 Baht
================================
```

**Explanation**

โปรแกรมจำลองระบบสั่งอาหาร โดยมีรายการเมนูและราคาของแต่ละเมนู ลูกค้าต้องการเลือกเฉพาะเมนูที่มีราคาไม่เกิน 200 บาท

โปรแกรมจะใช้ filter() เพื่อคัดเลือกเฉพาะเมนูที่ตรงตามเงื่อนไข จากนั้นใช้ map() เพื่อคำนวณราคาหลังได้รับส่วนลด 10% และใช้ fold() เพื่อรวมราคาของเมนูทั้งหมดที่ผ่านเงื่อนไข เพื่อคำนวณยอดชำระทั้งหมด

ในแต่ละขั้นตอนจะมีการใช้ Closure เป็นฟังก์ชันสำหรับกำหนดวิธีการประมวลผลข้อมูล ทำให้ตัวอย่างนี้สามารถแสดงแนวคิด Higher-Order Programming ได้ด้วย
---

## 7. Common Mistakes

### Mistake 1 — การลืมว่า Iterator ใน Rust เป็นแบบ Lazy Evaluation

**Problem**

Iterator ใน Rust มีคุณสมบัติ "Lazy" หมายความว่ามันจะไม่ประมวลผลอะไรเลย จนกว่าจะถูกเรียกใช้งานด้วย consuming adaptors (เช่น `.collect()`, `.sum()`, หรือถูกใช้งานใน `for` loop) มือใหม่มักเขียน chain ของ Iterator ทิ้งไว้โดยไม่ดึงค่าออกมา ทำให้โค้ดส่วนนั้นไม่ถูกทำงาน

**Incorrect Code**

```rust
fn main() {
    let numbers = vec![1, 2, 3];
    
    // โค้ดนี้จะไม่ทำอะไรเลย compiler จะแจ้งเตือน unused `Map`
    numbers.iter().map(|x| println!("Processing: {}", x));
}
```

**Correct Code**

```rust
fn main() {
    let numbers = vec![1, 2, 3];
    
    // วิธีที่ 1: ใช้ for loop เพื่อบริโภคค่า
    for _ in numbers.iter().map(|x| println!("Processing: {}", x)) {
        // Iterator ถูกกระตุ้นให้ทำงานแล้ว
    }
    
    // วิธีที่ 2: ใช้ .collect() หากต้องการนำผลลัพธ์ไปใช้ต่อ
    let _processed: Vec<_> = numbers.iter().map(|x| x * 2).collect();
}
```

**Why?**

Rust ออกแบบมาให้ทำงานเร็วและประหยัดทรัพยากร การเป็น Lazy ทำให้ Rust ไม่ต้องเสียเวลาสร้าง Collection ชั่วคราวในแต่ละขั้นตอนของ chain จนกว่าจะถึงจุดที่จำเป็นต้องใช้ผลลัพธ์จริงๆ

---

### Mistake 2 — สับสนระหว่าง iter(), iter_mut(), และ into_iter()

**Problem**

การเลือกใช้เมธอดสร้าง Iterator ผิดประเภท ทำให้เกิดปัญหา Ownership โดยเฉพาะการเผลอใช้ into_iter() ซึ่งจะย้ายกรรมสิทธิ์ (Move) ของตัวแปรไป ทำให้ไม่สามารถเรียกใช้ Collection ต้นทางได้อีก

**Incorrect Code**

```rust
fn main() {
    let names = vec![String::from("Alice"), String::from("Bob")];
    
    // into_iter() จะกิน (consume) ownership ของ names เข้าไป
    for name in names.into_iter() {
        println!("Hello, {}", name);
    }
    
    // Error: borrow of moved value: `names`
    println!("Total names: {}", names.len()); 
}
```

**Correct Code**

```rust
fn main() {
    let names = vec![String::from("Alice"), String::from("Bob")];
    
    // ใช้ iter() เพื่อขอยืมค่า (Borrow) มาอ่านเท่านั้น
    for name in names.iter() {
        println!("Hello, {}", name);
    }
    
    // สามารถเรียกใช้ names ต่อได้ตามปกติ
    println!("Total names: {}", names.len()); 
}
```

**Why?**

.iter(): ยืมค่าแบบอ่านอย่างเดียว (&T)

.iter_mut(): ยืมค่าแบบแก้ไขได้ (&mut T)

.into_iter(): ย้ายกรรมสิทธิ์แบบสมบูรณ์ (T) เหมาะสำหรับตอนที่เราไม่ต้องการใช้ Collection ต้นทางอีกต่อไป

---

## 8. Exercises

> จัดทำแบบฝึกหัด **2 ข้อ** ที่สอดคล้องกับ Topic และมีระดับความยากเหมาะสม

### Exercise 1 — `ภารกิจส่งมาม่ากู้ภัยน้ำท่วม (mama rescue missions)`

**Problem**
สถานการณ์น้ำท่วมปีนี้หนักหน่วงมาก เราจึงมี Vector ที่เก็บข้อมูลชื่อจังหวัดและระดับน้ำ (เซนติเมตร) ดังนี้
`let areas = vec![("Chiang Rai", 150), ("Phayao", 80), ("Chiang Mai", 120), ("Bangkok", 15)];`

```rust
fn main() {
    let areas = vec![
        ("Chiang Rai", 150), 
        ("Phayao", 80), 
        ("Chiang Mai", 120), 
        ("Bangkok", 15)
    ];
    
    let rescue_missions: Vec<String> = areas
        .iter()
        .filter(|&&(_, water_level)| water_level >= 100) 
        .map(|&(province, _)| format!("{} prepare boats and mama rn", province)) 
        .collect(); 
        
    println!("{:?}", rescue_missions);
}
```

**คำถาม**   
เมื่อโปรแกรมรันเสร็จ ผลลัพธ์ (Output) ที่แสดงบนหน้าจอจะเป็นข้อความว่าอะไรบ้าง?

**Hint**

`ให้ลองไล่ดูข้อมูลทีละบรรทัดว่า จังหวัดไหนบ้างที่ผ่านด่าน filter (ระดับน้ำ >= 100) แล้วจังหวัดที่ผ่านเข้าไป จะถูก map แปลงร่างข้อความเป็นอย่างไร?`

**Solution**

ผลลัพธ์ที่ได้คือ: ["Chiang Rai prepare boats and mama rn ", "Chiang Mai prepare boats and mama rn"]

**Explanation**

โค้ดจะทำงานแบบ Lazy Evaluation โดยดึงข้อมูลมาทีละตัว:
1. Chiang Rai (150) -> ผ่าน filter -> เข้า map แปลงเป็นข้อความ
2. Phayao (80) -> ไม่ผ่าน filter (ถูกเตะทิ้งทันที)
3. Chiang Mai (120) -> ผ่าน filter -> เข้า map แปลงเป็นข้อความ
4. Bangkok (15) -> ไม่ผ่าน filter
สุดท้าย collect() จะรวบรวมข้อความที่ผ่านเข้ารอบทั้งหมดมาใส่กรอบเป็น Vector เดียวกัน

---

### Exercise 2 — `รวมเหรียญทองเอเชียนเกมส์ 2026 (total gold coin)`

**Problem**

ในการแข่งขัน Aichi-Nagoya 2026 Asian Games ทีมชาติไทยคว้าเหรียญทองมาได้จากหลายกีฬา เรามี Vector เก็บชนิดกีฬาและจำนวนเหรียญทองที่ได้ดังนี้:
let medals = vec![("Sepak Takraw", 4), ("Esports", 1), ("Tamiya", 5), ("Muay Tha", 3)];

```rust
fn main() {
    let medals = vec![
        ("Sepak Takraw", 4), 
        ("Esports (RoV)", 1), 
        ("Tamiya", 5), //กีฬา Tamiya เพิ่งถูกบรรจุใหม่ที่สนาม Nagoya-grand prix 
        ("Muay Tha", 3)
    ];
    
    // fold(ค่าเริ่มต้น, |ตัวสะสม, ไอเทมปัจจุบัน|)
    let total_gold = medals.iter().fold(2, |acc, &(_, count)| acc + count);
    
    println!("Thailand total gold medals won: {} medals", total_gold);
}
```

**คำถาม**

ให้สังเกตโค้ดบรรทัดที่มีการใช้ .fold() ผลลัพธ์ที่พิมพ์ออกมาในบรรทัดสุดท้ายคือตัวเลขใด?

**Hint**

พารามิเตอร์ตัวแรกของ .fold() คือ "ค่าเริ่มต้น" (Accumulator) ให้ลองหาผลรวมของเหรียญทั้งหมดใน Vector แล้วอย่าลืมนำไปจัดการกับค่าเริ่มต้นด้วย

**Solution**

Thailand Total gold medals won: 15 medals

**Explanation**

โจทย์ข้อนี้เป็นการทดสอบความเข้าใจพารามิเตอร์ของ fold(initial_value, closure)
    ใน Vector มีเหรียญรวมกันคือ 4 + 1 + 5 + 3 = 13 เหรียญ
    แต่เราตั้งค่าเริ่มต้น (ตัวแปร acc รอบแรก) ไว้ที่ 2 (อาจจะเป็นเหรียญทองที่ยกยอดมาจากเมื่อวาน)
    ดังนั้นระบบจะนำ 2 เป็นตัวตั้ง แล้วค่อยๆ บวกเพิ่มเข้าไปทีละรายการจนครบ ลัพธ์สุทธิที่ได้จึงเป็น 2 + 13 = 15 ครับ

---

## 9. PPL Perspective

> **ส่วนนี้เป็นหัวใจของรายวิชา Principles of Programming Languages**

วิเคราะห์ Topic นี้ในมุมมองของ Programming Languages

### 9.1 Syntax

การประมวลผลข้อมูลใน Rust แบบ Iterator เขียนเป็นลูกโซ่ 3 ส่วน คือ **Source → Adaptor → Consumer** โดย Adaptor เช่น `map` และ `filter` เป็น Higher-Order Function ที่รับ "ฟังก์ชัน" เข้าไปกำหนดพฤติกรรม

**ตัวอย่าง**

```rust
fn is_even(x: &i32) -> bool {
    x % 2 == 0
}

fn square(x: i32) -> i32 {
    x * x
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6];

    let result: Vec<i32> = numbers
        .iter()          // Source
        .copied()        // Adaptor: copy numbers แยกกับ numbers ownership ใหม่
        .filter(is_even) 
        .map(square)     
        .collect();      // Consumer

    println!("{:?}", result);
}
```

ผลลัพธ์

```text
[4, 16, 36]
```

รูปแบบไวยากรณ์ที่สำคัญ

- Source: `.iter()`, `.into_iter()`, `1..=5` สร้าง iterator จากข้อมูล
- Adaptor: `.map()`, `.filter()`, `.take()`, `.enumerate()`, `.zip()` คืน iterator ตัวใหม่
- Consumer: `.collect()`, `.sum()`, `.fold()`, `.count()` ดึงค่าออกมาเป็นผลลัพธ์
- `for x in iterator { ... }` วนลูปผ่าน iterator
- `fn func<F: Fn(i32) -> i32>(f: F)` ฟังก์ชันที่รับฟังก์ชันเป็น parameter (ส่งได้ทั้งฟังก์ชันชื่อและ closure `|x| x + 1`)

### 9.2 Semantics

- Iterator ทุกตัวทำงานผ่าน method `next()` ที่คืน `Option` โดย `Some(value)` คือยังมีข้อมูล และ `None` คือหมดแล้ว 
- loop `for` ก็คือการเรียก `next()` ซ้ำจนได้ `None` 
- Short-Circuit จะหยุดเมื่อเจอสิ่งที่ต้องการแล้ว `take_while()`, `any()` จึงใช้กับเลขไม่รู้จบได้ เช่น `(1..)`

**ตัวอย่าง**

```rust
let v: Vec<i32> = [1, 2, 3, 10, 4, 5]
    .iter()
    .copied()
    .take_while(|&x| x < 5)
    .collect();
```
ผลลัพธ์
```text
[1, 2, 3]
```
<br>

```rust
let has_even = [1, 3, 4, 7].iter().any(|&x| x % 2 == 0); // true, หยุดที่ 4
```

- Adaptor เป็น Lazy Evaluation คือ ไม่คำนวณจนกว่าจะมี Consumer มาดึงค่า

**ตัวอย่าง**

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    let first_two: Vec<i32> = numbers
        .iter()
        .map(|x| {
            println!("map {}", x);
            x * 2
        })
        .take(2)
        .collect();

    println!("{:?}", first_two);
}
```

ผลลัพธ์

```text
map 1
map 2
[2, 4]
```

ข้อมูลมี 5 ตัว แต่ `map` ทำงานเพียง 2 ครั้ง เพราะ `take(2)` ต้องการแค่ 2 ค่า และ Lazy Evaluation ทำให้ไม่คำนวณส่วนที่เหลือ

### 9.3 Type System

- `Iterator<Item = T>` คือ iterator ที่ส่งค่าชนิด `T` ออกมาทีละตัว (`Item` เป็น Associated Type)
- Adaptor แต่ละตัวคืน type ใหม่ที่ห่อตัวเดิมไว้ เช่น `Map<I, F>` และ `Filter<I, P>` ซึ่ง compiler รู้ตั้งแต่ตอน Compile
```rust
let v = vec![1, 2, 3, 4];

let a = v.iter();                    // Iter<i32>
let b = a.filter(|x| **x > 1);       // Filter<Iter<i32>, closure1>
let c = b.map(|x| x * 10);           // Map<Filter<Iter<i32>, closure1>, closure2>
```
- Parameter ที่เป็นฟังก์ชันระบุด้วย Trait Bound เช่น `fn func<F: Fn(i32) -> i32>(f: F, value: i32) -> i32` หรือใช้ function pointer `fn func(f: fn(i32) -> i32, value: i32) -> i32`
- `collect()` ต้องรู้ชนิดปลายทาง เช่น `Vec<i32>` เพราะ collect เป็น `Vec`, `HashSet` หรือ `String` ได้

**ตัวอย่าง**

```rust
fn func<I: Iterator<Item = i32>, F: Fn(i32) -> i32>(items: I, f: F) -> Vec<i32> {
    items.map(f).collect()
}
/*
fn func<I, F>(items: I, f: F) -> Vec<i32>
where
    I: Iterator<Item = i32>,
    F: Fn(i32) -> i32,
{
    items.map(f).collect()
}
*/

fn main() {
    println!("{:?}", apply_all(1..=3, |x| x * 10));                  
    println!("{:?}", apply_all(vec![4, 5].into_iter(), |x| x + 1));  
}
```

ผลลัพธ์

```text
[10, 20, 30]
[5, 6]
```

`func` รับ iterator ชนิดใดก็ได้ที่ส่ง `i32` และรับฟังก์ชันที่เป็น `i32` และส่ง `i32` ถ้าส่งชนิดไม่ตรง compiler จะแจ้ง error ก่อนรันโปรแกรม

### 9.4 Memory / Resource Management

Iterator ใช้ระบบ Ownership และ Borrowing โดยวิธีสร้าง iterator ที่ต่างกันมีผลต่อ collection เดิมต่างกัน

- `iter()`: ยืมข้อมูลมาอ่าน (`&T`) collection เดิมยังใช้ต่อได้
- `iter_mut()`: ยืมข้อมูลมาแก้ไข (`&mut T`)
- `into_iter()`: ย้าย ownership เข้า iterator (`T`) collection เดิมใช้ต่อไม่ได้
- Iterator Chain ไม่สร้าง collection ชั่วคราวระหว่างขั้นตอน จึงไม่เปลือง Heap เพิ่ม

**ตัวอย่าง**

```rust
fn main() {
    let mut scores = vec![50, 60, 70];

    let total: i32 = scores.iter().sum();             // ยืมอ่าน
    println!("{} {:?}", total, scores);             

    for s in scores.iter_mut() {                      // ยืมแก้ได้
        *s += 10;
    }
    println!("{:?}", scores);                       

    let doubled: Vec<i32> = scores.into_iter().map(|s| s * 2).collect(); // ย้าย ownership
    // println!("{:?}", scores);                      // Error: scores ถูก move แล้ว
    println!("{:?}", doubled);                       
```

ผลลัพธ์

```text
180 [50, 60, 70]
[60, 70, 80]
[120, 140, 160]
```

นอกจากนี้ Borrow Checker ป้องกันการแก้ collection ขณะวนลูปอยู่ เช่น `v.push(...)` ภายใน `for x in v.iter()` จะเกิด Compile-time Error (E0502) 

**ตัวอย่าง**

```rust
let mut v = vec![1, 2, 3];

for x in v.iter() {  // iter() ยืมแบบอ่านแต่แก้ไม่ได้
    v.push(*x);      // error[E0502]
}
```

### 9.5 Abstraction / Other PPL Concepts

Iterator เป็น Abstraction ของ "การ loop ข้อมูลจากตัวแรกจนถึงตัวสุดท้าย"

```rust
trait Iterator {
        type Item;
        fn next(&mut self) -> Option<Self::Item>;
}
```

**ตัวอย่าง**

```rust
struct Countdown(u32);

impl Iterator for Countdown {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if self.0 == 0 {
            None
        } else {
            self.0 -= 1;
            Some(self.0 + 1)
        }
    }
}

fn main() {
    let total: u32 = Countdown(5).filter(|x| x % 2 == 1).sum();
    println!("{}", total);
}
```

ผลลัพธ์

```text
9
```

เมื่อ implement `next()` เพียงตัวเดียว `Countdown` ก็ใช้ `filter`, `map`, `sum` และ Adaptor อื่น ๆ ได้ทันที

**Other PPL Concepts**

- Scope: ตัวแปรของ iterator และ closure อยู่ภายใต้ Lexical Scope 
- Binding: รับฟังก์ชันผ่าน Generic (`F: Fn`) ผูกตอน Compile (Static Dispatch) ส่วนกรณีที่ไม่รู้ชนิดจริง จะรู้ตอนรันโปรแกรม `dyn` ผูกตอน Runtime (Dynamic Dispatch)
- Paradigm: Rust เป็น Multi-paradigm โดย Iterator และ Higher-Order Function รองรับ Functional Programming (First-class Function) แต่ไม่ใช่ภาษา Functional แบบ pure เพราะยังมี Side Effect (ฟังก์ชันทำนอกเหนือจากการคืนค่า เช่น พิมพ์ออกจอ เขียนไฟล์ แก้ตัวแปรภายนอก)

### 9.6 Why Rust?

- Memory Safety: Borrow Checker ตรวจการยืมและการ move ของ iterator ป้องกันการใช้ข้อมูลที่ถูก move ไปแล้วหรือถูกแก้ไขขณะวนลูป
- Reliability: `next()` คืน `Option` จึงต้องจัดการกรณีข้อมูลหมดอย่างชัดเจน และ type ของ Adaptor ทุกตัวถูกตรวจตอน Compile
- Performance: Iterator Chain เป็น Zero-Cost Abstraction เพราะ Lazy ไม่สร้าง collection ชั่วคราว และ compiler inline ฟังก์ชันที่ส่งผ่าน Generic ได้
- No Garbage Collector: จัดการทรัพยากรด้วย Ownership และจัดการค่าตาม scope

---

## 10. Rust vs. Other Language

**Comparison Language:** `Python`, `Java`, `C`

| Aspect | Rust | Python | Java | C |
|-|-|-|-|-|
| Syntax | Iterator chain `.iter().filter().map().sum()` ส่งฟังก์ชันหรือ closure เข้า adaptor | `map(f, filter(g, data))` ใช้ `lambda` หรือ generator expression | Stream API `.stream().filter().mapToInt().sum()` ส่ง lambda เข้า operation | ไม่มี syntax พิเศษ เขียนลูป `for` เองและส่ง function pointer `int (*f)(int)` |
| Semantics / Behavior | Iterator เป็น Lazy ทำงานเมื่อมี consumer และ `next()` คืน `Option` | `map`, `filter`, generator เป็น Lazy ผ่าน Iterator Protocol (`__next__`) แต่ list comprehension คำนวณทันที | Stream เป็น Lazy ทำงานเมื่อเจอ terminal operation และใช้ซ้ำไม่ได้หลังถูกใช้แล้ว | ไม่มี Lazy Evaluation ลูปคำนวณทันทีตามที่เขียน |
| Type System | Static + strong typing ใช้ `Iterator<Item = T>` และ Trait `Fn` ตรวจตอน Compile | Dynamic + strong typing ตรวจ type ของ argument ใน lambda ตอน Runtime | Static + strong typing ใช้ `Stream<T>` และ Functional Interface เช่น `Predicate`, `Function` | Static typing function pointer ระบุ signature ได้ แต่ compiler ตรวจได้จำกัด (cast ผิดชนิดได้) |
| Memory Management | Ownership, Borrowing, Lifetime ไม่มี Garbage Collector | Reference Counting ร่วมกับ Garbage Collector | Garbage Collector และ Stream สร้าง Object ภายในระหว่าง pipeline | Programmer จัดการเอง (`malloc`/`free`) ไม่มี Garbage Collector |
| Safety | Compiler ตรวจการยืมและการ move ของ iterator ก่อน compile ป้องกัน Iterator Invalidation | แก้ list ขณะวนลูปได้ ผลลัพธ์อาจผิดปกติโดยไม่มี error | แก้ collection ขณะวนลูปอาจเกิด `ConcurrentModificationException` ตอน Runtime | ไม่ตรวจขอบเขต array เข้าถึงเกิน index จะเกิด Undefined Behavior |

### Rust Example

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let result: i32 = numbers
        .iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * x)
        .sum();

    println!("{}", result);
}
```

`filter` และ `map` เป็น Higher-Order Function ที่รับฟังก์ชันเข้าไปกำหนดเงื่อนไขและการแปลงค่า และทำงานแบบ Lazy เมื่อ `sum()` ดึงค่า โดยไม่สร้าง Vec ชั่วคราว

### Analysis

Rust ประมวลผลข้อมูลด้วย Iterator Chain ที่เป็น Lazy และตรวจ type กับการยืมข้อมูลของ iterator ตั้งแต่ Compile Time ส่วน Higher-Order Function อย่าง `filter` และ `map` รับฟังก์ชันผ่าน Trait Bound ที่ compiler ตรวจสอบได้

### Python Example

```python
numbers = list(range(1, 11))

result = sum(map(lambda x: x * x, filter(lambda x: x % 2 == 0, numbers)))

print(result)
```

Python มี `map` และ `filter` เป็น Higher-Order Function ในตัว และมี Iterator Protocol (`__next__`) ที่ `map`/`filter` เป็น Lazy เช่นกัน แต่ไม่มีการตรวจ type ก่อนรัน

### Analysis

Python เป็น Dynamic Typing จึงเขียนสั้นและยืดหยุ่น และมี Generator ที่สร้าง iterator ได้ง่าย แต่ข้อผิดพลาดด้าน type ในฟังก์ชันที่ส่งเข้า `map`/`filter` จะพบตอน Runtime ส่วน Rust ตรวจตั้งแต่ Compile Time

### Java Example

```java
import java.util.List;

public class Main {
    public static void main(String[] args) {
        List<Integer> numbers = List.of(1, 2, 3, 4, 5, 6, 7, 8, 9, 10);

        int result = numbers.stream()
                .filter(x -> x % 2 == 0)
                .mapToInt(x -> x * x)
                .sum();

        System.out.println(result);
    }
}
```

Java ใช้ Stream API ที่มี pipeline `filter → map → sum` คล้าย Rust แต่ต้องเรียก `.stream()` ก่อน และใช้ `mapToInt` เพื่อแปลงเป็น `IntStream`

### Analysis

ทั้งสองภาษามี pipeline แบบ Functional และเป็น Lazy เหมือนกัน แต่ Java ทำงานบน JVM พร้อม Garbage Collector และ Stream ใช้ซ้ำไม่ได้ ส่วน Rust ควบคุมการยืม/ย้ายข้อมูลของ iterator ด้วย Ownership 

### C Example

```c
#include <stdio.h>

int is_even(int x) { return x % 2 == 0; }
int square(int x) { return x * x; }

int sum_if_map(int *arr, int n, int (*pred)(int), int (*f)(int)) {
    int total = 0;
    for (int i = 0; i < n; i++) {
        if (pred(arr[i])) {
            total += f(arr[i]);
        }
    }
    return total;
}

int main() {
    int numbers[] = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10};

    printf("%d\n", sum_if_map(numbers, 10, is_even, square));
    return 0;
}
```

C ไม่มี iterator ในตัวภาษา จึงเขียนลูปเองและส่งขนาด array (`n`) เอง ส่วน Higher-Order Function ทำได้ผ่าน function pointer (เช่น `qsort` ในไลบรารีมาตรฐาน)

### Analysis
C ทำ Higher-Order Function ได้ผ่าน function pointer แต่ไม่มี Iterator, Adaptor หรือ Lazy Evaluation ให้ Programmer ต้องเขียนลูปและดูแลขอบเขตของ array เอง ส่วน Rust มี Iterator ที่ตรวจขอบเขตและการยืมข้อมูลให้ เพราะ C ออกแบบมาเพื่อควบคุม Hardware และ Memory โดยตรง จึงมีกลไกระดับภาษาน้อยและให้ Programmer รับผิดชอบเอง

---

## 11. Teach Your Topic

การนำเสนอมีสมาชิก **4 คน คนละประมาณ 5 นาที**

| Member | Responsibility | Time |
|---|---|---:|
| Member 1 | Concept + Short Code Illustration | 5 min |
| Member 2 | Detailed Code + Live Demo | 5 min |
| Member 3 | Rust vs Other Language + PPL Analysis | 5 min |
| Member 4 | Exercises + Common Mistakes + Challenge | 5 min |

### Individual Contribution

**Member 1**

`Concept + Short Code Illustration (สรุปแนวคิดหลัก + โค้ดตัวอย่างสั้น)`

**Member 2**

`Detailed Code + Live Demo (โค้ดเชิงลึก + สาธิตสด)`

**Member 3**

`Rust vs Other Language + PPL Analysis (เปรียบเทียบภาษา + วิเคราะห์เชิง PPL)`

**Member 4**

`Exercises + Common Mistakes + Challenge (แบบฝึกหัด + ข้อผิดพลาดที่พบบ่อย + คำถามท้าทาย)`

> สมาชิกทุกคนต้องสามารถอธิบาย Code ของกลุ่มได้ ไม่ใช่เฉพาะส่วนที่ตนเองเขียน

---

## 12. References

> แนะนำให้มีอย่างน้อย **4 แหล่งอ้างอิง** และควรใช้เอกสารทางการเป็นหลัก

1. `Rust Foundation. Iterator Trait — Rust Standard Library Documentation.` <br> 
   https://doc.rust-lang.org/std/iter/trait.Iterator.html

2. `Rust Foundation. Iterators — The Rust Programming Language.`<br>
   https://doc.rust-lang.org/book/ch13-02-iterators.html

3. `Rust Foundation. Closures — The Rust Programming Language.`<br> 
   https://doc.rust-lang.org/book/ch13-01-closures.html

4. `GeeksforGeeks. Rust - Higher Order Functions.`<br> 
   https://www.geeksforgeeks.org/rust/rust-higher-order-functions/

5. `Crust of Rust: Iterators - Jon Gjengset.` <br>
    https://www.youtube.com/watch?v=yozQ9C69pNs
 
6. `82: Iterators in Rust are Awesome - Rustfully.` <br>
    https://www.youtube.com/watch?v=BEPhAIkw6x8
---

## 13. AI Usage Declaration

สามารถใช้ AI เป็นเครื่องมือช่วยเรียนรู้และพัฒนาได้ แต่สมาชิกทุกคนต้องเข้าใจและสามารถอธิบายผลงานของกลุ่มได้

| AI Tool | Purpose | How the Result Was Verified |
|---|---|---|
| `ChatGPT` | `แก้ไขวิธีการรันโปรแกรม` | `สามารถรันโปรแกรมได้` |
| `Claude` | `ถาม syntax, semantic และตรวจสอบความถูกต้องของcode` | `code runได้,อ่านเทียบข้อมูลที่ได้จากแหล่งอ้างอิง` |

### Declaration

- [✅] Code ทุกส่วนที่นำเสนอได้รับการ Compile และทดสอบแล้ว
- [✅] สมาชิกทุกคนสามารถอธิบาย Code ที่นำเสนอได้
- [✅] ตรวจสอบข้อมูลจากแหล่งอ้างอิงที่น่าเชื่อถือแล้ว
- [✅] ระบุการใช้ AI อย่างโปร่งใส

**รายละเอียดการใช้ AI**

`1. ใช้ก่อนเริ่มทำ project เพื่อเตรียมพื้นฐานสำหรับภาษา Rust เช่น Cargo.toml` <br>
`2. ใช้ถาม syntax และวิธีการเขียน fn หรือรูปแบบอื่นๆ` <br>
`3. ใช้เปรียบเทียบ code ภาษา Rust กับภาษาอื่นๆ และแก้ error`

---

## 14. GitHub Contribution

| Member | Issues | Commits | Pull Requests | Code Reviews | Contribution |
|---|---:|---:|---:|---:|---|
| Member 1 | `0` | `12` | `0` | `0` | `เขียน/แก้โค้ด` |
| Member 2 | `0` | `47` | `0` | `0` | `ทำหัวข้อ Detailed code + Live Demo` |
| Member 3 | `0` | `8` | `0` | `0` | `ทำหัวข้อ ppl และ rust vs other language` |
| Member 4 | `0` | `7` | `0` | `0` | `ทำหัวข้อ common mistake และ exercises /เปลี่ยน exercises` |

### Teamwork Reflection

**How did your team collaborate?**

`ทำงานร่วมกันผ่าน Github โดย fork แยกออกมาจาก module หลัก แล้ว pull request เมื่อเสร็จ`

**Problems encountered**

`การแก้ไขโค้ดในเวลาเดียวกันทำให้เกิด merge conflict`

**How did you solve them?**

`แบ่งขอบเขตการทำงานให้ชัดเจน, แยกเวลากันทำงาน, เลือกของใครคนใดคนหนึ่งในส่วนที่เกิด conflict`

---

## 15. Final Checklist

- [✅] Learning Objectives ครบ 3–4 ข้อ
- [✅] Key Concepts ครบถ้วน
- [✅] Syntax / Rules
- [✅] Runnable Code Examples
- [✅] Code Compile และ Run ได้จริง
- [✅] Common Mistakes
- [✅] Exercises 2 ข้อ พร้อม Solutions
- [✅] PPL Perspective
- [✅] Rust vs Other Language
- [✅] References อย่างน้อย 4 แหล่ง
- [✅] AI Usage Declaration
- [✅] GitHub Contribution
- [✅] สมาชิกทั้ง 4 คนมีส่วนร่วม
- [✅] สมาชิกทั้ง 4 คนพร้อมนำเสนอคนละ 5 นาที
- [✅] สมาชิกทุกคนสามารถอธิบาย Code ของกลุ่มได้

---

## Submission Information

**Repository:** `https://github.com/670710647/rust-tutorial-2569`

**Chapter Path:** `/20-iterators-higher-order-programming/`

**Final PR:** `#42`

**Submitted by:** `Group 20`

**Date:** `2026-10-04`
