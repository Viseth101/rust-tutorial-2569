
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
| 2 | นายกรันต์ชัย คำทรัพย์ | 670710290  | `@[670710290]` | Detailed Code + Live Demo               |
| 3 | นางสาวณัฐณิชา ภู่วงษ์ | 670710291  | `@nncp-fs` | Rust vs Other Language + PPL Analysis   |
| 4 | นายเทพพิทักษ์ นิลดำ     | 670710292  | `@670710292` | Exercises + Common Mistakes + Challenge |

---

## 2. Learning Objectives

หลังจากศึกษา Topic นี้แล้ว ผู้เรียนสามารถ:

1. อธิบายแนวคิดสำคัญของการใช้ References และ Borrowing ในภาษา Rust ได้
2. เขียนโปรแกรม Rust ที่มีการยืมข้อมูล (Borrowing) ทั้งแบบอ่านอย่างเดียว (Immutable) และแบบแก้ไขค่าได้ (Mutable)
3. วิเคราะห์พฤติกรรมของ Borrow Checker และกฎการจัดการหน่วยความจำที่ป้องกัน Data Races ของภาษา Rust ได้
4. เปรียบเทียบความปลอดภัยในการจัดการหน่วยความจำของ Rust (Compile-time) กับภาษาอื่นที่ใช้ Garbage Collector หรือ Manual Memory Management ได้

---

## 3. Introduction

การจัดการหน่วยความจำ (Memory Management) เป็นความท้าทายหลักในการเขียนโปรแกรม ภาษาอย่าง C ใช้การจัดการแบบ Manual (เช่น `malloc`/`free`) ซึ่งเสี่ยงต่อการเกิดข้อผิดพลาดอย่าง Dangling Pointers หรือ Memory Leaks ส่วนภาษาอย่าง Java ใช้ Garbage Collector (GC) ซึ่งมีความปลอดภัยแต่แลกมาด้วยประสิทธิภาพที่ลดลงระหว่างรันไทม์

Rust นำเสนอวิธีการใหม่ที่เรียกว่า **Ownership** โดยข้อมูลจะมีเจ้าของได้เพียงคนเดียว อย่างไรก็ตาม การส่งข้อมูลไปมาระหว่างฟังก์ชันโดยย้ายความเป็นเจ้าของ (Move) ตลอดเวลาเป็นเรื่องที่ไม่สะดวก Rust จึงมีระบบ **References & Borrowing** ซึ่งอนุญาตให้เรา "ยืม" ข้อมูลไปใช้งานโดยไม่ต้องยึดความเป็นเจ้าของ ระบบนี้ถูกควบคุมอย่างเข้มงวดโดย **Borrow Checker** ในช่วง Compile-time ทำให้ Rust รับประกันความปลอดภัยของหน่วยความจำ (Memory Safety) และป้องกัน Data Races ได้ 100% โดยไม่มี Overhead รันไทม์เหมือน GC

---

## 4. Key Concepts

### 4.1 Immutable References (การยืมแบบอ่านอย่างเดียว)

**คำอธิบาย**

References (`&`) อนุญาตให้เราอ้างอิงถึงค่าบางอย่างโดยไม่ต้องรับช่วงความเป็นเจ้าของ (Ownership) มา การกระทำนี้เรียกว่า **Borrowing (การยืม)** โดยค่าเริ่มต้น Reference จะเป็นแบบอ่านอย่างเดียว (Immutable) เราไม่สามารถแก้ไขข้อมูลที่ยืมมาได้

**ตัวอย่าง**

```rust
fn main() {
    let my_string = String::from("Silpakorn");
    let length = calculate_length(&my_string);
    println!("ความยาวของ '{}' คือ {}", my_string, length);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

**Explanation**

* บรรทัด `let length = calculate_length(&my_string);`: เครื่องหมาย `&` สร้าง Reference ที่ชี้ไปยังค่าของ `my_string` แต่ไม่ได้เป็นเจ้าของมัน
* ในฟังก์ชัน `calculate_length`: พารามิเตอร์ `s` มีชนิดข้อมูลเป็น `&String` (Reference ของ String)
* เมื่อฟังก์ชันทำงานจบ `s` จะหายไปจาก Scope แต่เนื่องจาก `s` ไม่ได้เป็นเจ้าของข้อมูล ข้อมูลต้นฉบับใน `my_string` จึงไม่ถูกทำลาย (Drop) ทำให้เรายังสามารถใช้ `my_string` ในคำสั่ง `println!` ต่อได้

---

### 4.2 Mutable References (การยืมแบบแก้ไขค่าได้)

**คำอธิบาย**

หากเราต้องการแก้ไขข้อมูลที่เรายืมมา เราต้องใช้ **Mutable Reference (`&mut`)** ทั้งตัวแปรต้นฉบับและ Reference ที่สร้างขึ้นจะต้องถูกประกาศเป็น `mut` ทั้งคู่

**ตัวอย่าง**

```rust
fn main() {
    let mut greeting = String::from("Hello");
    add_world(&mut greeting);
    println!("{}", greeting);
}

fn add_world(s: &mut String) {
    s.push_str(", World!");
}
```

**Explanation**

* `let mut greeting = String::from("Hello");`: ตัวแปรต้นฉบับต้องเป็น `mut` ถึงจะยอมให้คนอื่นยืมไปแก้ไขได้
* `&mut greeting`: เราส่ง Mutable Reference เข้าไปในฟังก์ชัน
* ฟังก์ชัน `add_world` รับพารามิเตอร์ชนิด `&mut String` ทำให้สามารถเรียกใช้เมธอด `.push_str()` เพื่อแก้ไขข้อมูลต้นฉบับได้โดยตรง
* ผลลัพธ์ที่พิมพ์ออกมาจะเป็น `Hello, World!`

---

## 5. Important Syntax / Rules

| Syntax / Rule | Meaning                                                                                                                              | Example                           |
| ------------- | ------------------------------------------------------------------------------------------------------------------------------------ | --------------------------------- |
| `&T`        | **Immutable Reference:** ยืมข้อมูลไปเพื่ออ่านอย่างเดียว ไม่สามารถแก้ไขค่าได้ | `let ref_val = &my_string;`     |
| `&mut T`    | **Mutable Reference:** ยืมข้อมูลไปและสามารถแก้ไขค่านั้นได้                                  | `let mut_ref = &mut my_string;` |

### Important Rules

1. ในช่วงเวลาใดเวลาหนึ่ง คุณสามารถมี Mutable Reference (`&mut T`) **ได้เพียง 1 ตัวเท่านั้น** หรือมี Immutable Reference (`&T`) **กี่ตัวก็ได้** แต่ไม่สามารถมีทั้งสองอย่างพร้อมกันได้ (ป้องกัน Data Races)
2. Reference ทุกตัวต้องชี้ไปยังข้อมูลที่มีอยู่จริงเสมอ (Rust จะป้องกันการเกิด "Dangling Pointers" ในขั้นตอนการ Compile อย่างเข้มงวด)

---

---

> **[ส่วนของ Member คนอื่นๆ เริ่มต้นที่นี่]**

---

---

## 6. Runnable Code Examples

> **ข้อกำหนด:** Code ทุกตัวต้อง Compile และ Run ได้จริงก่อนนำมาใส่ในเอกสาร

### Example 1 — `[ชื่อ Example]`

**Purpose:** `[ต้องการสาธิตอะไร]`

```rust
fn main() {
    // Write your runnable Rust code here
}
```

**Expected Output**

```text
[expected output]
```

**Explanation**

`[อธิบาย code ทีละส่วนที่สำคัญ]`

---

### Example 2 — `[ชื่อ Example]`

**Purpose:** `[ต้องการสาธิตอะไร]`

```rust
fn main() {
    // Write your runnable Rust code here
}
```

**Expected Output**

```text
[expected output]
```

**Explanation**

`[อธิบาย code]`

---

## 7. Common Mistakes

### Mistake 1 — `[ชื่อข้อผิดพลาด]`

**Problem**

`[อธิบายปัญหา]`

**Incorrect Code**

```rust
// Incorrect example
```

**Correct Code**

```rust
// Correct example
```

**Why?**

`[อธิบายสาเหตุ]`

---

### Mistake 2 — `[ชื่อข้อผิดพลาด]`

**Problem**

`[อธิบายปัญหา]`

**Incorrect Code**

```rust
// Incorrect example
```

**Correct Code**

```rust
// Correct example
```

**Why?**

`[อธิบายสาเหตุ]`

---

## 8. Exercises

> จัดทำแบบฝึกหัด **2 ข้อ** ที่สอดคล้องกับ Topic และมีระดับความยากเหมาะสม

### Exercise 1 — `[ชื่อโจทย์]`

**Problem**

`[เขียนโจทย์]`

**Hint**

`[คำใบ้]`

**Solution**

```rust
// Solution code
```

**Explanation**

`[อธิบายแนวทางแก้]`

---

### Exercise 2 — `[ชื่อโจทย์]`

**Problem**

`[เขียนโจทย์]`

**Hint**

`[คำใบ้]`

**Solution**

```rust
// Solution code
```

**Explanation**

`[อธิบายแนวทางแก้]`

---

## 9. PPL Perspective

> **ส่วนนี้เป็นหัวใจของรายวิชา Principles of Programming Languages**

วิเคราะห์ Topic นี้ในมุมมองของ Programming Languages

### 9.1 Syntax

References & Borrowing ในภาษา Rust เกี่ยวข้องกับ Syntax ผ่านรูปแบบการสร้างและประกาศ Reference โดยมี Syntax ที่สำคัญ ได้แก่ 
- `&T` สำหรับ Immutable Reference
- `&mut T` สำหรับ Mutable Reference
- `&'a T` สำหรับ Reference ที่มี Lifetime Annotation
- `&'a mut T` สำหรับ Mutable Reference ที่มี Lifetime Annotation 
ตาม Rust Reference รูปแบบทั่วไปของ Reference คือ `&` ตามด้วย Lifetime ที่อาจมีหรือไม่มีก็ได้ ตามด้วย `mut` ที่อาจมีหรือไม่มีก็ได้ และตามด้วย Type นอกจากนี้ Syntax ของ Reference ยังปรากฏใน Function Parameters และ Return Types เช่น `&String` และ `&'a str` ทำให้ลักษณะการเข้าถึงข้อมูลและข้อจำกัดของ Reference สามารถแสดงออกใน Source Code ได้อย่างชัดเจน

**ตัวอย่าง**
```rust
let x = 10;

let r = &x;
```

ในตัวอย่างนี้เป็นการสร้าง Immutable Reference ไปยัง `x`

**ตัวอย่างการ Mutable Reference**
```rust
let mut x = 10;

let r = &mut x;

*r = 20;
```
`&mut` แสดงให้เห็นว่า Reference สามารถใช้แก้ไขค่าของข้อมูลที่ถูก Borrow ได้

**ตัวอย่าง Lifetime**
```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```
`'a`  คือ Lifetime Parameter ที่ใช้ระบุความสัมพันธ์ของ Lifetime ระหว่าง References ไม่ได้หมายความว่าเป็นการกำหนดให้ Reference มีอายุยาวขึ้นเอง

### 9.2 Semantics

Semantics คือการศึกษาความหมายและพฤติกรรมของคำสั่งหรือโครงสร้าง (Construct) โดยอธิบายว่าเมื่อโปรแกรมถูกประมวลผลแล้ว คำสั่งนั้นจะทำงานอย่างไรและให้ผลลัพธ์แบบใด

ใน References & Borrowing สิ่งสำคัญคือ `&` และ `&mut` ไม่ได้เป็นเพียงสัญลักษณ์ทาง Syntax แต่มีความหมายเกี่ยวกับ วิธีการเข้าถึงข้อมูล

**Immutable Borrow**
```rust
let x = 10;

let r1 = &x;

println!("{r1}");
```
สามารถมี Immutable References หลายตัวพร้อมกันได้ เพราะ References เหล่านี้ใช้สำหรับอ่านข้อมูลและไม่ได้อนุญาตให้แก้ไขข้อมูลต้นทาง

**Mutable Borrow**
```rust
let mut x = 10;

let r = &mut x;

*r = 20;

println!("{x}");
```
`&mut x` หมายถึงการ Borrow `x` แบบ Mutable และ Reference นี้สามารถใช้แก้ไขค่าของ `x` ได้
### 9.3 Type System

References & Borrowing เกี่ยวข้องกับ Type System เพราะ Reference ใน Rust มี Type ของตัวเอง เช่น `&T` และ `&mut T` โดย Type เหล่านี้กำหนดลักษณะการเข้าถึงข้อมูลว่าเป็นการอ่านอย่างเดียวหรือสามารถแก้ไขข้อมูลได้ นอกจากนี้ Reference ยังสามารถระบุ Lifetime เช่น `&'a T` เพื่อแสดงความสัมพันธ์ของช่วงอายุระหว่าง References
```rust
let x = 10;
let r: &i32 = &x;
```
- `&x` คือการสร้าง Reference ที่ชี้ไปยัง `x`
- `r: &i32` ระบุว่า `r` มี Type เป็น Reference ไปยังข้อมูลชนิด `i32`
- เนื่องจากเป็น `&i32` จึงไม่สามารถแก้ไขค่า `x` ผ่าน `r` ได้
```rust
let mut x = 10;
let r: &mut i32 = &mut x;
*r = 20;
```
- `mut x` ทำให้ `x` สามารถถูกแก้ไขได้
- `&mut x` สร้าง Mutable Reference ไปยัง `x`
- `r: &mut i32` ระบุ Type ว่าเป็น Mutable Reference
- `*r = 20` คือการ Dereference เพื่อเข้าถึงและเปลี่ยนค่าที่ `r` อ้างถึง
### 9.4 Memory / Resource Management

References & Borrowing มีความสัมพันธ์โดยตรงกับ Ownership และ Lifetime ซึ่งเป็นกลไกสำคัญในการจัดการ Memory ของ Rust โดย Borrowing ทำให้สามารถนำข้อมูลไปใช้งานผ่าน Reference ได้โดยไม่ต้องโอน Ownership ให้กับผู้ที่นำข้อมูลไปใช้
```rust
fn print_length(s: &String) {
    println!("{}", s.len());
}

let text = String::from("Rust");
print_length(&text);
println!("{}", text);
```
- `&String` หมายถึง Function รับ Reference ไปยัง `String` แทนการรับ Ownership
- `s` จึงสามารถใช้ข้อมูลของ `text` ได้ แต่ไม่ได้เป็นเจ้าของข้อมูล
- `&text` คือการ Borrow ตัวแปร `text` เพื่อส่ง Reference เข้า Function
- หลังจากเรียก Function แล้ว `text` ยังสามารถใช้งานได้ เพราะ Ownership ยังคงอยู่ที่ `text`

### 9.5 Abstraction / Other PPL Concepts

References & Borrowing เชื่อมโยงกับแนวคิด PPL อื่น ๆ เช่น Abstraction, Scope และ Ownership โดยเฉพาะ Abstraction ที่ช่วยให้ Function สามารถกำหนดเพียงว่าต้องการ “เข้าถึงข้อมูล” โดยไม่จำเป็นต้องรับผิดชอบ Ownership ของข้อมูลนั้น ส่วน Scope จะกำหนดขอบเขตการใช้งานของ Reference
```rust
let mut x = 10;

{
    let r = &mut x;
    *r = 20;
}

println!("{}", x);
```
- `let mut x = 10` สร้างตัวแปรที่สามารถแก้ไขค่าได้
- `let r = &mut x` สร้าง Mutable Reference ภายใน Scope
- `*r = 20` แก้ไขค่าของ `x` ผ่าน Reference
- เมื่อออกจาก `{ }` การ Borrow ของ `r` สิ้นสุดลง
- จึงสามารถกลับมาใช้ `x` ได้ใน `println!`

ตัวอย่างนี้แสดงความสัมพันธ์ระหว่าง Borrowing และ Scope ได้อย่างชัดเจน

### 9.6 Why Rust?

Rust ใช้ References & Borrowing เพื่อสร้างสมดุลระหว่าง Safety, Reliability และ Performance โดย Compiler สามารถตรวจสอบกฎของ Ownership, Borrowing และ Lifetime ตั้งแต่ Compile Time

- Safety: ช่วยป้องกันการใช้ Reference ที่ไม่ถูกต้อง เช่น Reference ที่ชี้ไปยังข้อมูลที่หมดอายุแล้ว
- Reliability: กำหนดกฎการเข้าถึงข้อมูลอย่างชัดเจน เช่น การควบคุมการใช้ Mutable และ Immutable References
- Performance: สามารถส่งข้อมูลผ่าน Reference ได้โดยไม่จำเป็นต้องคัดลอกข้อมูลหรือโอน Ownership

ดังนั้น References & Borrowing จึงเป็นแนวคิดสำคัญที่ช่วยให้ Rust จัดการ Memory ได้อย่างปลอดภัยและมีประสิทธิภาพ โดยไม่ต้องใช้ Garbage Collector เป็นกลไกหลักของภาษา

---

## 10. Rust vs. Other Language

**Comparison Language:** `Python`

| Aspect               | Rust               | Other Language     |
| -------------------- | ------------------ | ------------------ |
| Syntax               | ใช้ `&` สำหรับ Reference, `&mut` สำหรับ Mutable Reference และ `*` สำหรับ Dereference | มี Reference โดยอัตโนมัติ แต่ไม่มี Syntax สำหรับ Borrowing โดยตรง |
| Semantics / Behavior | Compiler ตรวจสอบ Ownership, Borrowing และ Lifetime ตั้งแต่ Compile Time | Reference สามารถชี้ไปยัง Object เดียวกันได้ และ Memory ถูกจัดการอัตโนมัติ |
| Type System          | Statically Typed มี Reference Type เช่น `&T`, `&mut T` ซึ่งกำหนดสิทธิ์ในการเข้าถึงข้อมูล และสามารถใช้ Lifetime กับ Reference ได้ | Dynamically Typed ตัวแปรไม่จำเป็นต้องประกาศ Type และ Reference ถูกจัดการในระดับ Object |
| Memory Management    | ใช้ Ownership, Borrowing และ Lifetime โดยไม่ต้องใช้ Garbage Collector เป็นกลไกหลัก | ใช้ Automatic Memory Management และ Garbage Collection |
| Safety               | Compiler ช่วยตรวจสอบ Memory Safety และกฎการ Borrowing | จัดการ Memory อัตโนมัติ แต่ Type และข้อผิดพลาดหลายอย่างตรวจพบขณะ Runtime |

### Rust Example

```rust
fn calculate_length(text: &String) -> usize {
    text.len()
}

fn main() {
    let text = String::from("Hello Rust");

    let length = calculate_length(&text);

    println!("Text: {}", text);
    println!("Length: {}", length);
}
```
จุดสำคัญ

- `text` เป็นเจ้าของข้อมูล `String`
- `&text` เป็นการ Borrow ข้อมูลเพื่อส่งเข้า Function
- `&String` หมายถึง Function รับ Reference แทนการรับ Ownership
- หลังจากเรียก `calculate_length()` แล้ว `text` ยังสามารถใช้งานได้ เพราะ Ownership ไม่ได้ถูกโอน
### Python Example

```python
def calculate_length(text):
    return len(text)

text = "Hello Python"

length = calculate_length(text)

print("Text:", text)
print("Length:", length)
```
จุดสำคัญ

- `text` ถูกส่งเข้า Function โดยใช้ระบบ Reference ของ Object ใน Python
- ไม่มีการเขียน `&text` เพื่อระบุการ Borrowing
- Python ไม่ได้บังคับกฎ Ownership และ Borrowing แบบ Rust
- การจัดการ Memory ทำโดยระบบของ Python
### Analysis

*ความแตกต่างสำคัญ:* Rust แยกแนวคิด Ownership กับ Borrowing ออกจากกันอย่างชัดเจน โดย `&text` บอกว่า Function ต้องการเพียงเข้าถึงข้อมูล ไม่ได้เป็นเจ้าของข้อมูล ขณะที่ Python ใช้ระบบ Reference ของ Object โดยไม่มีแนวคิด Ownership/Borrowing ที่ผู้เขียนโปรแกรมต้องระบุใน Syntax

*เหตุผลด้านการออกแบบ:* Rust ถูกออกแบบให้สามารถตรวจสอบ Memory Safety ตั้งแต่ Compile Time โดยไม่ต้องพึ่ง Garbage Collector เป็นกลไกหลัก จึงต้องทำให้ความสัมพันธ์ระหว่างผู้เป็นเจ้าของข้อมูลกับผู้ที่ยืมข้อมูลมีความชัดเจน ส่วน Python เน้น ความเรียบง่ายและความสะดวกในการเขียนโปรแกรม จึงจัดการ Reference และ Memory ให้โดยอัตโนมัติ แม้จะแลกกับการตรวจสอบบางส่วนที่เกิดขึ้นใน Runtime

---
**Comparison Language:** `Java`

| Aspect               | Rust               | Other Language     |
| -------------------- | ------------------ | ------------------ |
| Syntax               | ใช้ `&` สำหรับ Reference, `&mut` สำหรับ Mutable Reference และ `*` สำหรับ Dereference | ใช้ Reference ของ Object โดยตรง แต่ไม่มี Borrowing Syntax แบบ Rust |
| Semantics / Behavior | Compiler ตรวจสอบ Ownership, Borrowing และ Lifetime ตั้งแต่ Compile Time | Reference ใช้เข้าถึง Object และ Object ถูกจัดการโดย Garbage Collector |
| Type System          | Statically Typed และมี Reference Type เช่น `&T`, `&mut T` | Statically Typed ต้องระบุ Type และ Reference จะระบุชนิดของ Object เช่น String หรือ Object |
| Memory Management    | ใช้ Ownership, Borrowing และ Lifetime โดยไม่ต้องใช้ Garbage Collector เป็นกลไกหลัก | ใช้ Garbage Collector จัดการ Memory ของ Object โดยอัตโนมัติ |
| Safety               | Compiler ช่วยตรวจสอบ Memory Safety และกฎการ Borrowing | มี Memory Safety จากการใช้ Reference และ Garbage Collector แต่ไม่มี Borrow Checker แบบ Rust |

### Rust Example

```rust
fn add_suffix(text: &mut String) {
    text.push_str(" Language");
}

fn main() {
    let mut text = String::from("Rust");

    add_suffix(&mut text);

    println!("{}", text);
}
```
จุดสำคัญ

- `String` เป็นข้อมูลที่ `text` เป็นเจ้าของ
- `&mut String` คือ Mutable Reference ที่อนุญาตให้ Function แก้ไขข้อมูล
- `&mut text` เป็นการ Borrow `text` แบบ Mutable
- Function ไม่ได้รับ Ownership ของ `text` จึงไม่ต้องคืน Ownership กลับมา
### Java Example

```Java
public class Main {
    static void addSuffix(StringBuilder text) {
        text.append(" Language");
    }

    public static void main(String[] args) {
        StringBuilder text = new StringBuilder("Java");

        addSuffix(text);

        System.out.println(text);
    }
}
```
จุดสำคัญ

- `text` เป็น Reference ที่ชี้ไปยัง Object `StringBuilder`
- เมื่อส่ง `text` เข้า Method จะมีการส่งค่าของ Reference ไป
- Method สามารถแก้ไข Object ที่ Reference ชี้อยู่ได้
- Memory ของ Object ถูกจัดการโดย Garbage Collector
### Analysis

*ความแตกต่างสำคัญ:* Rust กำหนดสิทธิ์ในการเข้าถึงข้อมูลอย่างชัดเจนผ่าน `&` และ `&mut` และ Compiler ตรวจสอบว่าการ Borrow ไม่ขัดกับกฎของ Ownership ขณะที่ Java ใช้ Reference ที่สามารถอ้างถึง Object และใช้ Garbage Collector จัดการอายุของ Object

*เหตุผลด้านการออกแบบ:* Rust เลือกใช้ Ownership + Borrowing + Lifetime เพื่อให้สามารถควบคุม Memory และตรวจสอบความปลอดภัยได้ตั้งแต่ Compile Time โดยไม่ต้องมี Garbage Collector ส่วน Java ถูกออกแบบให้เน้น ความง่ายในการพัฒนาและการจัดการ Memory อัตโนมัติ จึงใช้ Garbage Collector เพื่อจัดการ Object ที่ไม่ใช้งานแล้ว ทำให้ Programmer ไม่ต้องจัดการ Lifetime ของ Object ด้วยตนเอง

---
**Comparison Language:** `C`

| Aspect               | Rust               | Other Language     |
| -------------------- | ------------------ | ------------------ |
| Syntax               | ใช้ `&` สำหรับ Reference, `&mut` สำหรับ Mutable Reference และ `*` สำหรับ Dereference | ใช้ Pointer `*` และ Address-of `&` ในการเข้าถึง Memory |
| Semantics / Behavior | Compiler ตรวจสอบ Ownership, Borrowing และ Lifetime ตั้งแต่ Compile Time | Pointer สามารถเข้าถึงและแก้ไข Memory ได้โดยตรง |
| Type System          | Statically Typed และมี Reference Type เช่น `&T`, `&mut T` | Statically Typed ต้องระบุ Type ของตัวแปรและ Pointer เช่น `int *p` |
| Memory Management    | ใช้ Ownership, Borrowing และ Lifetime โดยไม่ต้องใช้ Garbage Collector เป็นกลไกหลัก | Programmer จัดการ Memory เอง เช่น `malloc()` / `free()` |
| Safety               | Compiler ช่วยตรวจสอบ Memory Safety และกฎการ Borrowing | มีความเสี่ยงจาก Pointer เช่น Dangling Pointer และ Memory Leak |

### Rust Example

```rust
fn double_value(value: &mut i32) {
    *value *= 2;
}

fn main() {
    let mut number = 10;

    double_value(&mut number);

    println!("Number: {}", number);
}
```
จุดสำคัญ

- `&mut i32` คือ Mutable Reference ที่สามารถแก้ไขข้อมูลได้
- `&mut number` เป็นการ Borrow `number` แบบ Mutable
- `*value` ใช้ Dereference เพื่อเข้าถึงค่าที่ Reference ชี้อยู่
- หลังจาก Function ทำงานเสร็จ `number` ยังคงเป็นเจ้าของข้อมูลและสามารถใช้งานต่อได้
### C Example

```C
#include <stdio.h>

void double_value(int *value) {
    *value *= 2;
}

int main() {
    int number = 10;

    double_value(&number);

    printf("Number: %d\n", number);

    return 0;
}
```
จุดสำคัญ

- `int *value` คือ Pointer ที่เก็บ Address ของตัวแปร
- `&number` ใช้หา Address ของ `number`
- `*value` ใช้ Dereference เพื่อเข้าถึงและแก้ไขข้อมูล
- C ให้ Programmer ควบคุม Pointer และ Memory โดยตรง
### Analysis

*ความแตกต่างสำคัญ:* แม้ Rust และ C จะมี `&` และ `*` ที่ดูคล้ายกัน แต่แนวคิดเบื้องหลังแตกต่างกัน โดย C ใช้ Pointer และ Address เพื่อให้ Programmer ควบคุม Memory ได้โดยตรง ขณะที่ Rust ใช้ Reference ภายใต้กฎ Ownership และ Borrowing

*เหตุผลด้านการออกแบบ:* C ถูกออกแบบโดยให้ความสำคัญกับ การควบคุม Hardware และประสิทธิภาพระดับต่ำ จึงเปิดให้เข้าถึง Memory ได้อย่างอิสระ ส่วน Rust ต้องการรักษาความสามารถในการควบคุม Memory และ Performance แบบภาษา System Programming แต่เพิ่ม Compile-time Safety เข้ามา จึงออกแบบ Ownership และ Borrowing เพื่อให้ Compiler ตรวจสอบการใช้ Reference และลดปัญหา เช่น Dangling Pointer และการเข้าถึงข้อมูลที่ไม่ถูกต้อง


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

`[สิ่งที่รับผิดชอบ]`

**Member 2**

`[สิ่งที่รับผิดชอบ]`

**Member 3**

`[สิ่งที่รับผิดชอบ]`

**Member 4**

`[สิ่งที่รับผิดชอบ]`

> สมาชิกทุกคนต้องสามารถอธิบาย Code ของกลุ่มได้ ไม่ใช่เฉพาะส่วนที่ตนเองเขียน

---

## 12. References

> แนะนำให้มีอย่างน้อย **4 แหล่งอ้างอิง** และควรใช้เอกสารทางการเป็นหลัก

1. `[The Rust Programming Language — Rust Book]`
2. `[Rust by Example / Rust Reference]`
3. `[Official documentation ที่เกี่ยวข้องกับ Topic]`
4. `[แหล่งอ้างอิงเพิ่มเติม]`

---

## 13. AI Usage Declaration

สามารถใช้ AI เป็นเครื่องมือช่วยเรียนรู้และพัฒนาได้ แต่สมาชิกทุกคนต้องเข้าใจและสามารถอธิบายผลงานของกลุ่มได้

| AI Tool                | Purpose                        | How the Result Was Verified        |
| ---------------------- | ------------------------------ | ---------------------------------- |
| `[เช่น ChatGPT]` | `[ใช้เพื่ออะไร]` | `[ตรวจสอบอย่างไร]` |
| `[AI tool]`          | `[ใช้เพื่ออะไร]` | `[ตรวจสอบอย่างไร]` |

### Declaration

- [ ] Code ทุกส่วนที่นำเสนอได้รับการ Compile และทดสอบแล้ว
- [ ] สมาชิกทุกคนสามารถอธิบาย Code ที่นำเสนอได้
- [ ] ตรวจสอบข้อมูลจากแหล่งอ้างอิงที่น่าเชื่อถือแล้ว
- [ ] ระบุการใช้ AI อย่างโปร่งใส

**รายละเอียดการใช้ AI**

`[อธิบายว่าใช้ AI ในขั้นตอนใด และสมาชิกตรวจสอบผลลัพธ์อย่างไร]`

---

## 14. GitHub Contribution

| Member   |           Issues |          Commits |    Pull Requests |     Code Reviews | Contribution               |
| -------- | ---------------: | ---------------: | ---------------: | ---------------: | -------------------------- |
| Member 1 | `[จำนวน]` | `[จำนวน]` | `[จำนวน]` | `[จำนวน]` | `[รายละเอียด]` |
| Member 2 | `[จำนวน]` | `[จำนวน]` | `[จำนวน]` | `[จำนวน]` | `[รายละเอียด]` |
| Member 3 | `[จำนวน]` | `[จำนวน]` | `[จำนวน]` | `[จำนวน]` | `[รายละเอียด]` |
| Member 4 | `[จำนวน]` | `[จำนวน]` | `[จำนวน]` | `[จำนวน]` | `[รายละเอียด]` |

### Teamwork Reflection

**How did your team collaborate?**

`[อธิบายกระบวนการทำงานร่วมกัน]`

**Problems encountered**

`[ปัญหาที่พบ]`

**How did you solve them?**

`[วิธีแก้ปัญหา]`

---

## 15. Final Checklist

- [ ] Learning Objectives ครบ 3–4 ข้อ
- [ ] Key Concepts ครบถ้วน
- [ ] Syntax / Rules
- [ ] Runnable Code Examples
- [ ] Code Compile และ Run ได้จริง
- [ ] Common Mistakes
- [ ] Exercises 2 ข้อ พร้อม Solutions
- [ ] PPL Perspective
- [ ] Rust vs Other Language
- [ ] References อย่างน้อย 4 แหล่ง
- [ ] AI Usage Declaration
- [ ] GitHub Contribution
- [ ] สมาชิกทั้ง 4 คนมีส่วนร่วม
- [ ] สมาชิกทั้ง 4 คนพร้อมนำเสนอคนละ 5 นาที
- [ ] สมาชิกทุกคนสามารถอธิบาย Code ของกลุ่มได้

---

## Submission Information

**Repository:** [github.com/Viseth101/12.References-Borrowing_PL-Group12.git](https://github.com/Viseth101/12.References-Borrowing_PL-Group12.git)

**Chapter Path:** `[เช่น chapters/01-introduction/]`

**Final PR:** `#[PR number]`

**Submitted by:** `[Group 12]`

**Date:** `[YYYY-MM-DD]`
