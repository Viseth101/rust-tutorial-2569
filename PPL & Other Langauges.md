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

| Aspect               | Rust               | Python     |
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

| Aspect               | Rust               | Java     |
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

| Aspect               | Rust               | C     |
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
