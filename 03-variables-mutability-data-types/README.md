# Rust Tutorial Project — Principles of Programming Languages

> **Topic No.:** `3`
> **Topic Name:** `Variables, Mutability & Data Types`
> **Group No.:** `3`

---

## 1. Members

| # | Name | Student ID | GitHub Username | Main Responsibility |
|---|---|---|---|---|
| 1 | นาธาน ศรีนาคาร | 660710606 | `@660710606` | Concept + Code |
| 2 | ภัทรพล ก่อมงคลกุล | 660710613 | `@660710613` | Code + Demo |
| 3 | พีรดนษ์ นิกพงศ์ | 660710624 | `@660710624` | Rust vs Other Language + PPL |
| 4 | รชต กระเช้าเพีชร์ | 660710626 | `@660710626` | Exercises + Common Mistakes |

---

## 2. Learning Objectives

หลังจากศึกษา Topic นี้แล้ว ผู้เรียนสามารถ:

1. อธิบายความแตกต่างระหว่าง `let`, `let mut`, `const` และ shadowing ได้อย่างถูกต้อง
2. เลือกใช้ data type พื้นฐานของ Rust (scalar และ compound types) และเขียนโปรแกรมที่ compile และ run ได้จริง
3. วิเคราะห์ได้ว่าทำไม Rust จึงกำหนดให้ตัวแปรเป็น immutable โดยค่าเริ่มต้น และ type system ของ Rust ช่วยป้องกันข้อผิดพลาดอย่างไร
4. เปรียบเทียบการประกาศตัวแปรและ type system ของ Rust กับ Python ได้อย่างมีเหตุผล

---

## 3. Introduction

ตัวแปร (variable) คือชื่อที่ผูก (bind) กับค่าในหน่วยความจำ ส่วน data type เป็นตัวกำหนดว่าค่านั้นเก็บอะไร ใช้พื้นที่เท่าไร และทำ operation อะไรกับมันได้บ้าง

ใน Rust การประกาศตัวแปรมีแนวคิดที่ต่างจากหลายภาษา คือ **ตัวแปรเป็น immutable โดยค่าเริ่มต้น** หากต้องการเปลี่ยนค่าต้องระบุ `mut` อย่างชัดเจน และ Rust เป็นภาษา **statically typed** ที่ compiler ต้องรู้ type ของทุกตัวแปรตอน compile time (แม้จะอนุมาน type ให้ได้ในหลายกรณี) แนวคิดเหล่านี้ช่วยให้อ่านโค้ดแล้วเดาพฤติกรรมได้ง่ายขึ้น ลดบั๊กจากการเปลี่ยนค่าโดยไม่ตั้งใจ และเป็นรากฐานของระบบ ownership ที่จะเรียนในหัวข้อถัดไป

---

## 4. Key Concepts

### 4.1 Variables และ Immutability

**คำอธิบาย**

ประกาศตัวแปรด้วยคีย์เวิร์ด `let` ตัวแปรที่ประกาศแบบปกติจะ **เปลี่ยนค่าไม่ได้** หลังกำหนดค่าแล้ว หากพยายามกำหนดค่าซ้ำ compiler จะแจ้ง error ตั้งแต่ตอน compile

**ตัวอย่าง**

```rust
fn main() {
    let x = 5;
    println!("x = {x}");
    // x = 6; // ❌ compile error: cannot assign twice to immutable variable
}
```

**Explanation**

`let x = 5;` สร้างตัวแปร `x` ที่ compiler อนุมาน type เป็น `i32` ให้อัตโนมัติ เมื่อมี `x = 6;` จะ compile ไม่ผ่านเพราะ `x` เป็น immutable

---

### 4.2 Mutability (`mut`)

**คำอธิบาย**

เพิ่ม `mut` หลัง `let` เพื่อให้เปลี่ยนค่าตัวแปรได้ แต่ **type ต้องคงเดิม** จะเปลี่ยนจากตัวเลขเป็นข้อความไม่ได้

```rust
fn main() {
    let mut count = 0;
    count += 1;
    count += 1;
    println!("count = {count}");
}
```

---

### 4.3 Constants (`const`)

**คำอธิบาย**

`const` คือค่าที่เปลี่ยนไม่ได้เสมอ ต่างจาก `let` ดังนี้

- ใช้ `mut` กับ `const` ไม่ได้
- **ต้องระบุ type** เสมอ
- ค่าต้องเป็น constant expression ที่คำนวณได้ตอน compile time
- ประกาศได้ในทุก scope รวมถึง global scope
- ตามธรรมเนียมตั้งชื่อด้วย `SCREAMING_SNAKE_CASE`

```rust
const MAX_POINTS: u32 = 100_000;

fn main() {
    println!("MAX_POINTS = {MAX_POINTS}");
}
```

---

### 4.4 Shadowing

**คำอธิบาย**

เราสามารถประกาศตัวแปรชื่อเดิมด้วย `let` ซ้ำได้ ตัวแปรใหม่จะ "บัง" (shadow) ตัวเดิม ต่างจาก `mut` ตรงที่เป็นการ **สร้างตัวแปรใหม่** จึงเปลี่ยน type ได้ และเมื่อออกจาก scope ของ block ด้านใน ตัวแปรเดิมจะกลับมาใช้งานได้อีก

```rust
fn main() {
    let spaces = "   ";        // &str
    let spaces = spaces.len(); // usize (type เปลี่ยนได้)
    println!("spaces = {spaces}");

    let y = 10;
    {
        let y = y * 2; // shadow เฉพาะใน block นี้
        println!("inner y = {y}");
    }
    println!("outer y = {y}");
}
```

| | `mut` | Shadowing |
|---|---|---|
| สร้างตัวแปรใหม่ | ไม่ (แก้ค่าเดิม) | ใช่ |
| เปลี่ยน type ได้ | ไม่ได้ | ได้ |
| ต้องใช้ `let` | ไม่ (ใช้ `=` ได้เลย) | ใช่ |

---

### 4.5 Data Types

Rust เป็นภาษา statically typed จึงต้องรู้ type ของทุกค่าตอน compile time แบ่งเป็น 2 กลุ่มใหญ่

**Scalar Types** (ค่าเดี่ยว)

| Type | รายละเอียด | ตัวอย่าง |
|---|---|---|
| Integer | signed `i8 i16 i32 i64 i128 isize`, unsigned `u8 u16 u32 u64 u128 usize` (ค่าเริ่มต้นคือ `i32`) | `42`, `0xff`, `1_000`, `b'A'` |
| Floating-point | `f32`, `f64` (ค่าเริ่มต้นคือ `f64`) | `3.5`, `2.5_f32` |
| Boolean | `bool` มี 2 ค่าคือ `true`, `false` | `true` |
| Character | `char` ขนาด 4 ไบต์ แทน Unicode scalar value | `'a'`, `'🦀'` |

**Compound Types** (รวมหลายค่า)

| Type | รายละเอียด | ตัวอย่าง |
|---|---|---|
| Tuple | รวมค่าหลาย type ในขนาดคงที่ | `(500, 6.4, 'R')` |
| Array | ค่า type เดียวกัน **ความยาวคงที่** เก็บบน stack | `[1, 2, 3]`, `[0; 3]` |

**Integer overflow:** ใน debug build จะ panic ส่วน release build จะ wrap around หากต้องการควบคุมพฤติกรรมให้ใช้เมธอดอย่าง `wrapping_add`, `checked_add`, `saturating_add`

---

## 5. Important Syntax / Rules

| Syntax / Rule | Meaning | Example |
|---|---|---|
| `let x = v;` | ประกาศตัวแปร immutable | `let x = 5;` |
| `let mut x = v;` | ประกาศตัวแปร mutable | `let mut n = 0;` |
| `const NAME: T = v;` | ค่าคงที่ ต้องระบุ type | `const MAX: u32 = 10;` |
| `let x: T = v;` | ระบุ type ชัดเจน | `let a: u8 = 255;` |
| shadowing | ประกาศชื่อเดิมซ้ำด้วย `let` | `let x = x + 1;` |
| `(a, b)` / `t.0` | สร้าง tuple / เข้าถึงสมาชิกด้วย index | `t.0` |
| `[T; N]` / `a[i]` | type ของ array / เข้าถึงสมาชิก | `[i32; 5]` |
| `as` | แปลง type ด้วยตัวเอง (explicit cast) | `x as f64` |

### Important Rules

1. ตัวแปรเป็น immutable โดยค่าเริ่มต้น ต้องใช้ `mut` หากต้องการเปลี่ยนค่า
2. ตัวแปร `mut` เปลี่ยนค่าได้แต่เปลี่ยน type ไม่ได้ ส่วน shadowing เปลี่ยน type ได้เพราะเป็นตัวแปรใหม่
3. Rust **ไม่แปลง type ให้โดยอัตโนมัติ** (no implicit conversion) ต้องใช้ `as` หรือ `From`/`Into` เอง
4. ค่าที่ compiler อนุมาน type ไม่ได้ ต้องระบุ type annotation
5. การเข้าถึง array นอกช่วง index จะ panic ตอน runtime (Rust ไม่อ่านหน่วยความจำนอก array)

---

## 6. Runnable Code Examples

> **หมายเหตุ:** ควรทดสอบโค้ดทุกตัวด้วย `cargo run` หรือ [Rust Playground](https://play.rust-lang.org/) ก่อนส่ง

### Example 1 — `let`, `mut`, `const` และ Shadowing (ใบเสร็จร้านค้า)

**Purpose:** สาธิตความต่างของ immutable, mutable, constant และ shadowing ผ่านการคำนวณยอดซื้อสินค้า

```rust
const VAT_RATE: f64 = 0.07;

fn main() {
    // 1) immutable: ราคาต่อชิ้นกำหนดแล้วเปลี่ยนไม่ได้
    let unit_price = 50.0;
    println!("ราคาต่อชิ้น = {unit_price}");

    // 2) mutable: จำนวนสินค้าเพิ่มได้
    let mut quantity = 0;
    quantity += 2;
    quantity += 3;
    println!("จำนวน = {quantity}");

    // 3) constant: อัตราภาษีมูลค่าเพิ่ม
    println!("VAT_RATE = {VAT_RATE}");

    // 4) shadowing เปลี่ยน type ได้ (i32 -> f64)
    let quantity = quantity as f64;
    let total = unit_price * quantity;
    println!("ยอดก่อนภาษี = {total}");

    // 5) shadowing ใน block
    {
        let total = total * (1.0 + VAT_RATE);
        println!("ยอดรวมภาษี (ใน block) = {total:.2}");
    }
    println!("ยอดก่อนภาษี (นอก block) = {total}");
}
```

**Expected Output**

```text
ราคาต่อชิ้น = 50
จำนวน = 5
VAT_RATE = 0.07
ยอดก่อนภาษี = 250
ยอดรวมภาษี (ใน block) = 267.50
ยอดก่อนภาษี (นอก block) = 250
```

**Explanation**

- `unit_price` เป็น immutable จึงอ่านได้อย่างเดียว (ค่า `50.0` เป็น `f64` จึงแสดงผลเป็น `50`)
- `quantity` ประกาศด้วย `mut` จึงบวกค่าได้ เมื่อบวก 2 และ 3 ค่าจึงเป็น 5
- `VAT_RATE` เป็น `const` ที่อยู่ระดับ global และมี type กำกับ
- `quantity` ถูก shadow จาก `i32` เป็น `f64` ด้วย `as f64` เพื่อนำไปคูณกับ `unit_price` ซึ่งทำได้เพราะเป็นตัวแปรใหม่ (Rust ไม่แปลง type ให้อัตโนมัติ)
- `total` ใน block ด้านในเป็นตัวแปรคนละตัวกับ `total` ด้านนอก (250 × 1.07 = 267.50) เมื่อจบ block ค่า `total` ด้านนอกยังเป็น 250

---

### Example 2 — Data Types

**Purpose:** สาธิต scalar types, tuple, array และการจัดการ integer overflow อย่างปลอดภัย

```rust
fn main() {
    // Scalar types
    let a: i32 = -42;
    let b: u8 = 255;
    let c: i64 = 9_000_000_000;
    let d = 0xff; // hexadecimal, type เป็น i32
    let e: f64 = 3.5;
    let f = 2.5_f32;
    let is_ok: bool = true;
    let ch: char = '🦀';

    println!("i32: {a}");
    println!("u8 max: {b}");
    println!("i64: {c}");
    println!("hex 0xff = {d}");
    println!("f64: {e}, f32: {f}");
    println!("bool: {is_ok}, char: {ch}");
    println!("size of char = {} bytes", std::mem::size_of::<char>());

    // Tuple
    let t: (i32, f64, char) = (500, 6.4, 'R');
    let (x, y, z) = t; // destructuring
    println!("tuple = {:?}, t.0 = {}", t, t.0);
    println!("x={x}, y={y}, z={z}");

    // Array
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let zeros = [0; 3];
    println!("array = {:?}, len = {}", arr, arr.len());
    println!("zeros = {:?}", zeros);

    // Integer overflow แบบควบคุมได้
    println!("255 wrapping_add 1 = {}", b.wrapping_add(1));
    println!("255 checked_add 1 = {:?}", b.checked_add(1));
}
```

**Expected Output**

```text
i32: -42
u8 max: 255
i64: 9000000000
hex 0xff = 255
f64: 3.5, f32: 2.5
bool: true, char: 🦀
size of char = 4 bytes
tuple = (500, 6.4, 'R'), t.0 = 500
x=500, y=6.4, z=R
array = [1, 2, 3, 4, 5], len = 5
zeros = [0, 0, 0]
255 wrapping_add 1 = 0
255 checked_add 1 = None
```

**Explanation**

- `u8` เก็บได้ 0–255 จึงบวก 1 แล้วเกินช่วง `wrapping_add` วนกลับเป็น 0 ส่วน `checked_add` คืน `None` เพื่อบอกว่า overflow
- `char` ใช้ 4 ไบต์ จึงเก็บอีโมจิและตัวอักษรไทยได้
- `{:?}` คือ Debug formatting ใช้พิมพ์ tuple และ array
- `[0; 3]` คือการสร้าง array ที่มีค่า 0 จำนวน 3 ตัว

---

## 7. Common Mistakes

### Mistake 1 — กำหนดค่าซ้ำให้ตัวแปร immutable

**Problem**

ประกาศตัวแปรด้วย `let` แล้วพยายามเปลี่ยนค่าภายหลัง

**Incorrect Code**

```rust
fn main() {
    let x = 5;
    x = 6; // ❌ error[E0384]: cannot assign twice to immutable variable `x`
    println!("{x}");
}
```

**Correct Code**

```rust
fn main() {
    let mut x = 5;
    x = 6;
    println!("{x}");
}
```

**Why?**

`let` ทำให้ตัวแปรเป็น immutable ถ้าต้องการเปลี่ยนค่าต้องประกาศด้วย `let mut` ซึ่งเป็นการประกาศเจตนาให้ผู้อ่านโค้ดรู้ว่าค่านี้จะเปลี่ยน

---

### Mistake 2 — ใช้ `mut` เพื่อเปลี่ยน type ของตัวแปร

**Problem**

เข้าใจว่า `mut` เปลี่ยนได้ทุกอย่างรวมถึง type

**Incorrect Code**

```rust
fn main() {
    let mut spaces = "   ";
    spaces = spaces.len(); // ❌ error[E0308]: mismatched types (expected &str, found usize)
    println!("{spaces}");
}
```

**Correct Code**

```rust
fn main() {
    let spaces = "   ";
    let spaces = spaces.len(); // ✅ shadowing
    println!("{spaces}");
}
```

**Why?**

`mut` เปลี่ยนได้เฉพาะ "ค่า" แต่ type ของตัวแปรถูกกำหนดไว้ตั้งแต่ประกาศ หากต้องการเปลี่ยน type ให้ใช้ shadowing

---

### Mistake 3 — คำนวณข้าม type โดยไม่แปลงเอง

**Problem**

นำตัวเลขต่าง type มาบวกกันตรง ๆ แบบที่ภาษาอื่นแปลงให้อัตโนมัติ

**Incorrect Code**

```rust
fn main() {
    let a: i32 = 5;
    let b: i64 = 10;
    let total = a + b; // ❌ error[E0308]: mismatched types
    println!("{total}");
}
```

**Correct Code**

```rust
fn main() {
    let a: i32 = 5;
    let b: i64 = 10;
    let total = a as i64 + b; // หรือ i64::from(a) + b
    println!("{total}");
}
```

**Why?**

Rust ไม่มี implicit numeric conversion เพื่อกันข้อมูลเสียหรือ overflow โดยไม่รู้ตัว ผู้เขียนต้องแปลง type อย่างชัดเจนเอง (`as` หรือ `From`/`Into`)

---

## 8. Exercises

### Exercise 1 — คำนวณราคาสินค้าด้วย Shadowing

**Problem**

กำหนดราคาสินค้าเป็นข้อความ `"1200"` ให้แปลงเป็น `f64` แล้วลดราคา 10% จากนั้นบวกภาษีมูลค่าเพิ่ม 7% และแสดงราคาสุทธิทศนิยม 2 ตำแหน่ง โดยใช้ **ชื่อตัวแปร `price` เพียงชื่อเดียว** (ใช้ shadowing) และไม่ใช้ `mut`

**Hint**

ใช้ `.parse()` พร้อมระบุ type เป็น `f64` ลดราคา 10% คือคูณ `0.9` และบวกภาษี 7% คือคูณ `1.07` ใช้ `{:.2}` เพื่อกำหนดทศนิยม

**Solution**

```rust
fn main() {
    let price = "1200";
    let price: f64 = price.parse().unwrap();
    let price = price * 0.9;  // ลดราคา 10%
    let price = price * 1.07; // บวกภาษี 7%
    println!("ราคาสุทธิ = {price:.2} บาท");
}
```

**Expected Output**

```text
ราคาสุทธิ = 1155.60 บาท
```

**Explanation**

`price` ถูก shadow 3 ครั้ง คือ `&str` → `f64` (ราคาเต็ม) → `f64` (หลังลดราคา) → `f64` (หลังบวกภาษี) ทำให้ใช้ชื่อเดียวได้โดยไม่ต้องใช้ `mut` และตัวแปรทุกตัวยังเป็น immutable ราคาหลังลด 10% คือ 1080 และบวกภาษี 7% เป็น 1155.60

---

### Exercise 2 — หาอุณหภูมิต่ำสุดและสูงสุดด้วย Array, `mut` และ Tuple

**Problem**

มี array อุณหภูมิ 6 วัน `[31, 28, 35, 30, 33, 27]` ให้หาค่าต่ำสุดและสูงสุด เก็บผลลัพธ์ไว้ใน tuple `(i32, i32)` แล้วแสดงผล พร้อมหาช่วงอุณหภูมิ (สูงสุด − ต่ำสุด)

**Hint**

กำหนดตัวแปร `mut` สำหรับค่าต่ำสุดและสูงสุดเริ่มจากสมาชิกตัวแรก `temps[0]` แล้วใช้ลูป `for` เทียบค่าทีละตัว

**Solution**

```rust
fn main() {
    let temps: [i32; 6] = [31, 28, 35, 30, 33, 27];

    let mut min = temps[0];
    let mut max = temps[0];
    for t in temps {
        if t < min {
            min = t;
        }
        if t > max {
            max = t;
        }
    }

    let range: (i32, i32) = (min, max);
    println!("min = {}, max = {}", range.0, range.1);
    println!("ช่วงอุณหภูมิ = {}", range.1 - range.0);
}
```

**Expected Output**

```text
min = 27, max = 35
ช่วงอุณหภูมิ = 8
```

**Explanation**

`min` และ `max` ต้องเป็น `mut` เพราะค่าเปลี่ยนระหว่างลูป การเข้าถึง `temps[0]` เป็นการอ่านสมาชิก array ด้วย index (Rust ตรวจ bounds ตอน runtime) tuple `(i32, i32)` ใช้รวมผลลัพธ์สองค่าไว้ด้วยกัน และเข้าถึงด้วย `range.0` กับ `range.1`

---

## 9. PPL Perspective

### 9.1 Syntax

Rust ใช้ keyword `let` สำหรับการประกาศตัวแปร (variable declaration) โดย `mut` เป็น modifier ที่ต้องเขียนชัดเจน ส่วน type annotation เขียนหลังชื่อตัวแปรด้วย `:` (เช่น `let x: i32`) ซึ่งเป็นรูปแบบเดียวกับ Kotlin, Swift และ TypeScript ต่างจาก C/Java ที่เขียน type นำหน้า (`int x`) ค่า literal มี syntax เฉพาะ เช่น `1_000`, `0xff`, `2.5_f32`, `b'A'`

### 9.2 Semantics

`let` คือ **name binding** ผูกชื่อเข้ากับค่า ไม่ใช่เพียงการจองพื้นที่เก็บข้อมูลเหมือนในมุมมองแบบเดิม ความหมายของ immutability คือ binding นั้นห้ามกำหนดค่าใหม่ ส่วน shadowing คือการสร้าง binding ใหม่ที่ซ้อนทับชื่อเดิมใน scope เดียวกันหรือ scope ย่อย เมื่อ block ภายในจบ binding ใหม่จะหมดอายุและ binding เดิมกลับมามองเห็นอีกครั้ง ซึ่งสัมพันธ์กับแนวคิด **scope และ lexical binding**

### 9.3 Type System

- **Statically typed:** ตรวจ type ตอน compile time
- **Strongly typed:** ไม่มี implicit coercion ระหว่าง numeric types
- **Type inference:** compiler อนุมาน type จากบริบทด้วยอัลกอริทึมในตระกูล Hindley–Milner (เช่น `let x = 5;` ได้ `i32`) แต่ต้องมี annotation เมื่อกำกวม เช่น `.parse()`
- **Default types:** integer ใช้ `i32`, floating-point ใช้ `f64`
- **Compound types:** tuple (product type) และ array ที่ความยาวเป็นส่วนหนึ่งของ type (`[i32; 5]` กับ `[i32; 3]` เป็นคนละ type)

### 9.4 Memory / Resource Management

- ค่า scalar, tuple และ array ที่ขนาดแน่นอนจะเก็บบน **stack** ซึ่ง compiler รู้ขนาดตอน compile
- scalar types ทั้งหมดเป็น `Copy` การกำหนดค่าให้ตัวแปรอื่นจึงเป็นการ copy ไม่ใช่ move
- `const` ถูก inline เข้าไปในจุดที่ใช้ตอน compile (ไม่มีตำแหน่งในหน่วยความจำคงที่)
- Array bounds ถูกตรวจสอบ (bounds checking) ตอน runtime จึงเข้าถึงหน่วยความจำนอกช่วงไม่ได้ ต่างจาก C
- เมื่อตัวแปรออกจาก scope ค่าจะถูกคืนทรัพยากรโดยอัตโนมัติ (scope-based resource management) ซึ่งเป็นพื้นฐานของระบบ ownership

### 9.5 Abstraction / Other PPL Concepts

- **Binding และ Scope:** shadowing แสดงให้เห็นการซ้อนกันของ scope
- **Immutability by default:** ใกล้เคียงกับแนวคิด functional programming ที่ลด side effect
- **Explicit over implicit:** `mut`, `as` และ type annotation บังคับให้เจตนาของโปรแกรมเมอร์ชัดเจนในโค้ด

### 9.6 Why Rust?

เมื่อค่าไม่เปลี่ยนโดยปริยาย การวิเคราะห์โค้ดทั้งโดยคนและโดย compiler ง่ายขึ้น ลดบั๊กจากการแก้ค่าโดยไม่ตั้งใจ และเป็นพื้นฐานให้ Rust ป้องกัน data race ได้ในระบบ ownership และ borrowing (ตัวแปร `mut` ที่ถูกอ้างอิงพร้อมกันหลายที่จะถูกจำกัดโดย borrow checker) ขณะเดียวกัน static typing กับ no implicit conversion ก็ช่วยจับข้อผิดพลาดตั้งแต่ compile time โดยไม่เสีย performance ตอน run

---

## 10. Rust vs. Other Languages

**Comparison Languages:** Python, Java, C

| **Aspect** | **Rust** | **Python** | **Java** | **C** |
|---|---|---|---|---|
| **Syntax** | `let x: i32 = 5;` ต้องมี `let` | `x = 5` กำหนดค่าได้ทันที | `int x = 5;` ต้องระบุชนิดข้อมูล | `int x = 5;` ต้องระบุชนิดข้อมูล |
| **Semantics / Behavior** | ตัวแปร immutable เป็นค่าเริ่มต้น ต้อง `mut` | ตัวแปรเปลี่ยนค่าได้เสมอ (rebind) | ตัวแปรเปลี่ยนค่าได้ เว้นแต่ใช้ `final` | ตัวแปรเปลี่ยนค่าได้เป็นค่าเริ่มต้น |
| **Type System** | Static, strong, มี type inference | Dynamic, strong (type ผูกกับค่า ไม่ใช่ตัวแปร) | Static, strong, มี type checking ตอน compile | Static, มี implicit conversions ได้หลายกรณี |
| **Memory Management** | Ownership, borrowing, scope-based, ไม่มี GC | Garbage collection + reference counting | Garbage Collector (GC) | Manual memory management (`malloc` / `free`) |
| **Safety** | ตรวจ type, ownership และ borrowing ตอน compile | Type error ส่วนใหญ่พบตอน runtime และมี GC ช่วยจัดการ memory | Type checking ตอน compile และมี GC ช่วยลด memory errors | ผู้พัฒนาต้องระวัง pointer, buffer overflow และ memory leak |

### Rust Example

```rust
fn main() {
    let x = 5;
    // x = 6;       // ❌ compile error
    // x = "hello"; // ❌ compile error
    let x = "hello"; // ✅ shadowing
    println!("{x}");
}
```

### Python Example

```python
x = 5
x = 6        # ได้ ไม่มี error
x = "hello"  # ได้ เพราะ type ผูกกับค่า ไม่ใช่ตัวแปร
print(x)
```

### Java Example

```java
public class Main {
    public static void main(String[] args) {
        int x = 5;
        x = 6; // ✅ เปลี่ยนค่าได้
        // x = "hello"; // ❌ compile error
        System.out.println(x);
    }
}
```

### C Example

```c
#include <stdio.h>

int main() {
    int x = 5;
    x = 6; // ✅ เปลี่ยนค่าได้
    // x = "hello"; // ❌ type mismatch
    printf("%d\n", x);
    return 0;
}
```

### Analysis

Python ผูกชื่อกับ object และให้ rebind ชื่อไปยังค่า type ใดก็ได้ในทุกเวลา ทำให้เขียนเร็วแต่ข้อผิดพลาดเรื่อง type ไปปรากฏตอน runtime (ต้องพึ่ง type hint และเครื่องมืออย่าง mypy ซึ่งไม่ได้บังคับ)

Java เป็นภาษา statically typed เช่นเดียวกับ Rust และ C โดย type ของตัวแปรต้องสอดคล้องกับการประกาศ เช่น `int x = 5;` และ compiler สามารถตรวจสอบ type error ได้ก่อน run โดย Java ใช้ Garbage Collector เพื่อจัดการ memory ให้โดยอัตโนมัติ

C เป็นภาษา statically typed และให้ผู้พัฒนาควบคุม memory และ hardware ได้ละเอียดมาก แต่การจัดการ memory เป็นความรับผิดชอบของผู้พัฒนาเอง จึงต้องระวัง pointer, memory leak และ buffer overflow

Rust เลือกให้ compiler ตรวจสอบ type, ownership และ borrowing ล่วงหน้า และให้ผู้เขียนระบุเจตนาด้วย `mut` หรือ shadowing อย่างชัดเจน แลกกับการเขียนโค้ดที่เคร่งครัดกว่า ผลที่ได้คือ performance สูงโดยไม่ต้องมี garbage collector และมี memory safety สูงกว่า C ในขณะที่ยังคงแนวทาง system programming ได้

---

## 11. Teach Your Topic

การนำเสนอมีสมาชิก **4 คน คนละประมาณ 5 นาที**

| Member | Responsibility | Time |
|---|---|---:|
| Member 1 | Concept (`let`, `mut`, `const`, shadowing) + Short Code Illustration | 5 min |
| Member 2 | Data Types + Live Demo (Example 1–2) | 5 min |
| Member 3 | Rust vs Python + PPL Analysis | 5 min |
| Member 4 | Exercises + Common Mistakes + Challenge | 5 min |

### Individual Contribution

**Member 1**

นาธาน ศรีนาคาร — **Concept + Short Code Illustration**  
รับผิดชอบอธิบายแนวคิดหลักของหัวข้อ ได้แก่ `let`, `mut`, `const` และ shadowing พร้อมยกตัวอย่างโค้ดสั้น ๆ ประกอบการอธิบาย และนำเสนอส่วน Concept

**Member 2**

ภัทรพล ก่อมงคลกุล — **Detailed Code + Live Demo**  
รับผิดชอบโค้ดตัวอย่างแบบละเอียด (Example 1–2) ครอบคลุมตัวแปร, mutability และ data types พร้อมสาธิตการ compile และรันโค้ดสดระหว่างนำเสนอ

**Member 3**

พีรดนษ์ นิกพงศ์ — **Rust, Other Language, PPL Analysis**  
รับผิดชอบการเปรียบเทียบ Rust กับ Python และการวิเคราะห์ตามหลัก Principles of Programming Languages (syntax, semantics, type system, memory management) และนำเสนอส่วนนี้

**Member 4**

รชต กระเช้าเพีชร์ — **Exercises, Common Mistakes, Challenge**  
รับผิดชอบแบบฝึกหัด 2 ข้อพร้อมเฉลย, ข้อผิดพลาดที่พบบ่อยพร้อมตัวอย่างโค้ดที่ผิดและโค้ดที่ถูกต้อง และโจทย์ท้าทายให้ผู้ฟังลองตอบระหว่างนำเสนอ

> สมาชิกทุกคนต้องสามารถอธิบาย Code ของกลุ่มได้ ไม่ใช่เฉพาะส่วนที่ตนเองเขียน

---

## 12. References

1. The Rust Programming Language — Variables and Mutability: https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html
2. The Rust Programming Language — Data Types: https://doc.rust-lang.org/book/ch03-02-data-types.html
3. Rust by Example — Variable Bindings / Primitives: https://doc.rust-lang.org/rust-by-example/
4. The Rust Standard Library — Primitive Types: https://doc.rust-lang.org/std/#primitives
5. The Rust Reference: https://doc.rust-lang.org/reference/

---

## 13. AI Usage Declaration

| AI Tool | Purpose | How the Result Was Verified |
|---|---|---|
| `NotebookLM` | `ใช้เพื่อค้นหาข้อมูลและเปรียบเทียบข้อมูลเชิงppl` | `ตรวจสอบความถูกต้องโดยเทียบกับไฟล์เอกสารที่ใช้เรียนในรายวิชาPrincipleProgramLanguageเป็นหลัก` |
| `Claude` | `เขียนโค๊ดเปรียบเทียบระหว่างRustกับPython` | `ช่วยเช็คsyntaxและlogicการเขียนcodeทั้งสองภาษา` |

### Declaration

- [x] Code ทุกส่วนที่นำเสนอได้รับการ Compile และทดสอบแล้ว
- [x] สมาชิกทุกคนสามารถอธิบาย Code ที่นำเสนอได้
- [x] ตรวจสอบข้อมูลจากแหล่งอ้างอิงที่น่าเชื่อถือแล้ว
- [x] ระบุการใช้ AI อย่างโปร่งใส

**รายละเอียดการใช้ AI**

นำเอกสารที่เรียนไปให้ NotebookLM สรุป และนำมาเป็นเกณฑ์ในการวิเคราะห์ภาษาเชิง PPL และใช้ Claude ช่วยเช็ค syntax ของแต่ละภาษา

---

## 14. GitHub Contribution

| Member | Issues | Commits | Pull Requests | Code Reviews | Contribution |
|---|---:|---:|---:|---:|---|
| Member 1 | `0` | `1` | `1` | `1` | `เขียนหัวข้อ Introduction และ Key Concepts, เปิด PR ส่วน Concept, review PR ของ Member 2` |
| Member 2 | `0` | `1` | `1` | `1` | `เขียน Example 1–2 และทดสอบโค้ดด้วย cargo run, review PR ของ Member 3` |
| Member 3 | `0` | `4` | `3` | `1` | `เขียนหัวข้อ PPL Perspective และ Rust vs Python, review PR ของ Member 4` |
| Member 4 | `0` | `1` | `1` | `1` | `เขียน Common Mistakes และ Exercises, review PR ของ Member 1` |

### Teamwork Reflection

**How did your team collaborate?**

`เราได้นัดกันมาทำและแบ่งหน้าที่ตามของแต่ละคนและทำการรวมงานเพื่อนเอาversionที่สมบูรณ์ส่ง`

**Problems encountered**

`บางทีเพื่อนเอาไปแก้แล้วcodeบางส่วนที่เคยทำขาดหาย`

**How did you solve them?**

`ปรึกษาและคอยupdateงานกันตลอดเวลา`

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

**Repository:** https://github.com/soonklang/rust-tutorial-2569

**Chapter Path:** `03-variables-mutability-data-types/`

**Final PR:** ``

**Submitted by:** `Group 03`

**Date:** `2026-10-03`
