# Rust Tutorial Project — Principles of Programming Languages

> **สำหรับนักศึกษา:** ใช้ไฟล์นี้เป็น Template สำหรับจัดทำบทเรียน Rust ของกลุ่ม
> **Topic No.:** `12`
> **Topic Name:** `References & Borrowing`
> **Group No.:** `12`

---

## 1. Members

| # | Name                                      | Student ID | GitHub Username | Main Responsibility                     |
| - | ----------------------------------------- | ---------- | --------------- | --------------------------------------- |
| 1 | Mr.UDTARAKVISETH LAY                      | 670710259  | `@Viseth101`  | Concept + Short Code                    |
| 2 | นายกรันต์ชัย คำทรัพย์ | 670710290  | `@670710290`  | Detailed Code + Live Demo               |
| 3 | นางสาวณัฐณิชา ภู่วงษ์ | 670710291  | `@nncp-fs`    | Rust vs Other Language + PPL Analysis   |
| 4 | นายเทพพิทักษ์ นิลดำ     | 670710292  | `@670710292`  | Exercises + Common Mistakes + Challenge |

---

## 2. Learning Objectives

หลังจากศึกษาเรื่องนี้แล้ว ผู้เรียนสามารถ:

- อธิบายความสัมพันธ์ระหว่าง Ownership, References และ Borrowing เพื่อจัดการหน่วยความจำอย่างปลอดภัยได้
- แยกความแตกต่างระหว่าง Immutable Reference (`&T`) และ Mutable Reference (`&mut T`) พร้อมเลือกใช้ได้อย่างเหมาะสม
- อธิบายบทบาทของ Borrow Checker และกฎการยืมที่ช่วยป้องกัน Data Race และ Dangling Reference ได้
- วิเคราะห์ข้อดีของการตรวจสอบความปลอดภัยของหน่วยความจำในระยะคอมไพล์ โดยไม่ต้องใช้ Garbage Collector ได้

---

## 3. Introduction

การจัดการหน่วยความจำเป็นความท้าทายสำคัญในการเขียนโปรแกรม ภาษาอย่าง C ให้ผู้พัฒนาจัดการหน่วยความจำด้วยตนเอง จึงอาจเกิดปัญหา เช่น Pointer ที่ชี้ไปยังข้อมูลซึ่งหมดอายุหรือการรั่วไหลของหน่วยความจำ ขณะที่บางภาษาใช้ Garbage Collector เพื่อจัดการหน่วยความจำโดยอัตโนมัติ

Rust ใช้แนวคิด **Ownership** ซึ่งกำหนดให้ข้อมูลแต่ละชิ้นมีเจ้าของได้เพียงหนึ่งราย เมื่อเจ้าของออกจากขอบเขต (Scope) Rust จะคืนหน่วยความจำให้อัตโนมัติ อย่างไรก็ตาม หากต้องย้าย Ownership ทุกครั้งที่ส่งข้อมูลระหว่างฟังก์ชัน โค้ดจะไม่สะดวกและอาจไม่เหมาะกับการใช้งานบางรูปแบบ Rust จึงมี **References และ Borrowing** สำหรับยืมข้อมูลไปใช้งานโดยไม่รับ Ownership มา

การยืมถูกตรวจสอบโดย **Borrow Checker** ในระยะคอมไพล์ ทำให้ Rust ตรวจพบการใช้หน่วยความจำที่ไม่ปลอดภัยก่อนโปรแกรมทำงาน เช่น การใช้ข้อมูลหลังถูกทำลายหรือการแก้ไขข้อมูลพร้อมกับการอ่านจากหลายจุด การตรวจสอบนี้ช่วยรับประกัน Memory Safety โดยไม่ต้องพึ่งพา Garbage Collector ในระยะรันไทม์

---

## 4. แนวคิดสำคัญ (Key Concepts)

### 4.1 Immutable References (การอ้างอิงแบบอ่านอย่างเดียว)

**คำอธิบาย**

References (`&`) ช่วยให้ฟังก์ชันเข้าถึงข้อมูลโดยไม่รับช่วงความเป็นเจ้าของ (Ownership) การใช้งานนี้เรียกว่า **Borrowing (การยืม)** โดย Reference แบบ `&T` เป็นการยืมเพื่ออ่าน จึงไม่สามารถแก้ไขข้อมูลต้นฉบับผ่าน Reference นั้นได้

**ตัวอย่างจากไฟล์ `src/bin/01_concept.rs`**

```rust
let my_string = String::from("Silpakorn");
let length = calculate_length(&my_string);
println!("The length of '{}' is {}.", my_string, length);

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

**คำอธิบายการทำงานทีละบรรทัด**

- `let my_string = String::from("Silpakorn");` สร้าง `String` บนฮีป และให้ `my_string` เป็นเจ้าของข้อมูล
- `let length = calculate_length(&my_string);` ใช้ `&` สร้าง Immutable Reference แล้วส่งไปยังฟังก์ชัน จึงเป็นการยืมโดยไม่ย้าย Ownership
- `fn calculate_length(s: &String) -> usize` กำหนดให้ `s` รับ Reference ของ `String` และคืนค่าชนิด `usize`
- `s.len()` อ่านความยาวของ String ผ่าน Reference แล้วส่งค่าความยาวกลับไป โดยไม่ได้แก้ไขข้อมูล
- `println!(...)` ใช้ `my_string` ได้อีกครั้งหลังเรียกฟังก์ชัน เพราะ Ownership ยังคงอยู่ที่ `my_string` และ `s` ไม่ได้เป็นเจ้าของข้อมูล

---

### 4.2 Mutable References (การอ้างอิงแบบแก้ไขค่าได้)

**คำอธิบาย**

หากต้องการแก้ไขข้อมูลผ่าน Reference ต้องใช้ **Mutable Reference (`&mut T`)** ตัวแปรเจ้าของข้อมูลต้องประกาศด้วย `mut` และ Reference ที่ส่งเข้าไปต้องใช้ `&mut` เช่นกัน การยืมชนิดนี้ให้สิทธิ์แก้ไขข้อมูลต้นฉบับได้ภายใต้กฎของ Borrow Checker

**ตัวอย่างจากไฟล์ `src/bin/01_concept.rs`**

```rust
let mut greeting = String::from("Hello");
append_text(&mut greeting);
println!("After borrowing mutably: {}", greeting);

fn append_text(s: &mut String) {
    s.push_str(", Rust!");
}
```

**คำอธิบายการทำงานทีละบรรทัด**

- `let mut greeting = String::from("Hello");` สร้าง String และประกาศตัวแปรเจ้าของให้แก้ไขค่าได้ด้วย `mut`
- `append_text(&mut greeting);` สร้าง Mutable Reference ด้วย `&mut` แล้วส่งให้ฟังก์ชัน โดยยังคงให้ `greeting` เป็นเจ้าของข้อมูล
- `fn append_text(s: &mut String)` กำหนดให้ `s` รับ Mutable Reference ของ String จึงแก้ไขข้อมูลที่ `s` อ้างถึงได้
- `s.push_str(", Rust!");` เติมข้อความลงใน String เดิมผ่าน Reference ทำให้ค่าของ `greeting` เปลี่ยนเป็น `Hello, Rust!`
- `println!(...)` แสดงค่าหลังการยืมสิ้นสุด โดย `greeting` ยังคงเป็นเจ้าของ String เดิม

---

## 5. ไวยากรณ์และกฎสำคัญ (Important Syntax / Rules)

| ไวยากรณ์ | ความหมาย                                                                                                                                         | ตัวอย่าง                  |
| ---------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------- |
| `&T`           | Immutable Reference: ยืมค่าเพื่ออ่าน โดยไม่รับ Ownership และไม่สามารถแก้ไขค่าผ่าน Reference นี้ได้ | `let ref_val = &my_string;`     |
| `&mut T`       | Mutable Reference: ยืมค่าเพื่ออ่านและแก้ไข โดยต้องได้รับอนุญาตจากตัวแปรเจ้าของ                 | `let mut_ref = &mut my_string;` |

### กฎการยืมที่ต้องปฏิบัติตาม

1. ในขอบเขตเวลาที่การยืมยังใช้งานอยู่ สามารถมี Mutable Reference (`&mut T`) ได้เพียงหนึ่งตัว หรือมี Immutable Reference (`&T`) ได้หลายตัว
2. ห้ามมี Mutable Reference และ Immutable Reference ของข้อมูลเดียวกันพร้อมกัน เพราะอาจทำให้เกิด Data Race
3. Reference ทุกตัวต้องชี้ไปยังข้อมูลที่ยังมีอยู่จริงและมีอายุการใช้งาน (Lifetime) ครอบคลุม Reference นั้นเสมอ Rust จึงป้องกัน Dangling Reference ตั้งแต่ระยะคอมไพล์
4. การสร้าง Mutable Reference ต้องใช้ตัวแปรเจ้าของที่ประกาศด้วย `mut`
