# Rust Tutorial Project — Principles of Programming Languages

> **สำหรับนักศึกษา:** ใช้ไฟล์นี้เป็น Template สำหรับจัดทำบทเรียน Rust ของกลุ่ม  
> **Topic No.:** `11`  
> **Topic Name:** `Ownership`  
> **Group No.:** `11`
> 
> **ประเด็นหลักที่ควรครอบคลุม:** ownership rules, move, copy, scope, memory management

---

## 1. Members

| # | Name | Student ID | GitHub Username | Main Responsibility |
|---|---|---|---|---|
| 1 | `สิริญญาธร ปุณกะบุตร` | `670710151` | `@670710151` | Concept + Code |
| 2 | `อังกฤษ ถ้ำสุวรรณ` | `670710152` | `@670710152` | Code + Demo |
| 3 | `ภูริณัฐ สุวรรณสังโส` | `670710153` | `@670710153` | Rust vs Other Language + PPL |
| 4 | `Sothea Sokea` | `670710258` | `@sotheasokea` | Exercises + Common Mistakes |

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

1.`Topic นี้คืออะไร`<br>
--> `ระบบ (set of rules) ที่ Rust ใช้จัดการหน่วยความจำ (memory management) โดยไม่ต้องมี Garbage Collector`<br>
2.`ทำไมถึงสำคัญ`<br>
--> `Ownership เป็น แนวคิดที่เป็นเอกลักษณ์ที่สุด ของ Rust และเป็นรากฐานของฟีเจอร์อื่นเกือบทั้งหมดในภาษา (borrowing, lifetimes, smart pointers ล้วนต่อยอดจากแนวคิดนี้)`<br>
-`ปลอดภัยเท่าภาษาที่มี Garbage Collector แต่เร็วเท่าภาษาระดับต่ำ`<br>
-`ตรวจจับ bug ตั้งแต่ compile time`<br>
-`ไม่มี runtime overhead`<br>
3.`ใช้แก้ปัญหาอะไรในการเขียนโปรแกรม`<br>
--> `ช่วยแก้ปัญหาความปลอดภัยของหน่วยความจำที่พบบ่อยในการเขียนโปรแกรม ได้แก่ `<br>
-`dangling pointer (การเข้าถึงหน่วยความจำที่ถูกคืนไปแล้ว) `<br>
-`double free (การคืนหน่วยความจำซ้ำ) `<br>
-`memory leak (การลืมคืนหน่วยความจำ)`<br>
`โดยไม่ต้องแลกกับ performance ของโปรแกรม ทำให้ Rust สามารถให้ทั้งความปลอดภัยและความเร็วไปพร้อมกันได้`<br>

---

## 4. Key Concepts

### 4.1 `Ownership คืออะไร (กฎพื้นฐาน 3 ข้อ)`

**คำอธิบาย**

`Ownership คือระบบจัดการหน่วยความจำของ Rust โดยไม่ใช้ Garbage Collector `<br>
`มีกฎ 3 ข้อ: `<br>
`(1) ทุกๆค่า ใน Rust จะมี "เจ้าของ" (Owner) เสมอ `<br>
`(2) เมื่อ owner หลุด scope ค่านั้นถูก drop ทันที โดยอัตโนมัติ`<br>
`(3) มี owner ได้เพียง "คนเดียว" เท่านั้นในเวลาเดียวกัน`<br>

**ตัวอย่าง**

```rust
fn main() {
    let s = String::from("hello");
    println!("{}", s);
} // s หลุด scope ที่นี่ -> ถูก drop อัตโนมัติ
```

**Explanation**

`ตัวแปร s เป็นเจ้าของค่า "hello" บน heap เมื่อโค้ดมาถึงปิดวงเล็บ } ซึ่งเป็นจุดที่ s หลุดออกจาก scope`<br>
`Rust จะเรียก drop() ให้อัตโนมัติเพื่อคืนหน่วยความจำ โดยไม่ต้องเขียน free() เอง`<br>

---

### 4.2 `Move Semantics`

`Move คือการ "ย้าย" ความเป็นเจ้าของ (ownership) จากตัวแปรเดิมไปยังตัวแปรใหม่`<br>
`Rust ออกแบบให้ตัวแปรเดิมใช้งานต่อไม่ได้ทันทีหลัง move เพื่อป้องกันปัญหา double free`<br>
`ช่วยแก้ปัญหาการที่สองตัวแปรชี้ไปยังข้อมูล heap เดียวกัน แล้วพยายาม drop ข้อมูลซ้ำตอน scope จบ`<br>
`ตัวแปรที่ถูก move แล้ว compiler จะบล็อกไม่ให้ใช้ต่อ (compile error ทันที)`<br>

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;  // ownership ย้ายจาก s1 ไป s2
    println!("{}", s2);
}
```

**Explanation**

`หลังบรรทัด let s2 = s1; ความเป็นเจ้าของถูกย้ายจาก s1 ไปยัง s2 ถ้าพยายามใช้ s1 ต่อ `<br>
`เช่น println!("{}", s1) จะเกิด compile error ทันที เพราะ Rust ไม่ยอมให้มีสอง owner ชี้ไปยังข้อมูลก้อนเดียวกัน` <br>
`ป้องกันปัญหาที่ทั้งสองตัวแปรจะพยายาม drop ข้อมูลเดียวกันซ้ำ`<br>


---

### 4.3 `Copy & Clone`

`Copy Trait: type พื้นฐานบน stack (i32, bool, char, f64) จะถูก copy อัตโนมัติแทนการ move เพราะคัดลอกแบบcost น้อย `<br>
`Clone: สำหรับข้อมูลบน heap (เช่น String) ต้องเรียก .clone() เพื่อคัดลอกข้อมูลจริงแบบ deep copy อย่างชัดเจน`<br>
`ช่วยแก้ปัญหากรณีต้องการใช้ตัวแปรสองตัวพร้อมกัน โดยไม่ทำให้ตัวแปรเดิมถูก move ทิ้ง`<br>

```rust
fn main() {
    let x = 5;
    let y = x;              // copy อัตโนมัติ (stack)
    let s1 = String::from("hello");
    let s2 = s1.clone();    // ต้องเรียก clone เอง (heap)
    println!("{} {} {} {}", x, y, s1, s2);
}
```

**Explanation**

`เนื่องจาก i32 มีขนาดคงที่และอยู่บน stack การคัดลอกค่ามีต้นทุนต่ำมาก Rust จึงอนุญาตให้ x และ y ใช้งานได้พร้อมกันโดยไม่ error` <br>
`s1.clone() คัดลอกข้อมูลบน heap ทั้งหมดไปสร้างเป็นก้อนใหม่ให้ s2 ทำให้ s1 และ s2 ต่างมีข้อมูลของตัวเองแยกกันคนละก้อน จึงใช้งานได้พร้อมกันทั้งคู่โดยไม่เกิด error`<br>


---

### 4.4 `Scope`

`Scope คือขอบเขตของตัวแปรในโปรแกรม ตั้งแต่จุดที่ถูกประกาศจนถึงจุดที่ปิดวงเล็บ {}`<br>
`Rust ผูก ownership เข้ากับ scope โดยตรง ตัวแปรมีผลใช้งานได้เฉพาะภายใน scope ของมันเท่านั้น`<br>
`ช่วยแก้ปัญหาการจัดการหน่วยความจำแบบ Manual เพราะไม่ต้องกำหนดจุดคืนหน่วยความจำเอง(free)`<br>
`เมื่อออกจาก scope ตัวแปรที่เป็น owner จะถูก drop ทันทีโดยอัตโนมัติ`<br>

```rust
fn main() {
    {
        let s = String::from("hello"); // s เริ่มมีผล
        println!("{}", s);
    } // s หลุด scope ที่นี่ -> ถูก drop
    // println!("{}", s); //  error: s ไม่มีผลแล้วนอก scope
}
```
**Explanation**

`ตัวแปร s มีผลใช้งานได้เฉพาะภายใน block {} ที่ประกาศเท่านั้น เมื่อโปรแกรมมาถึง } ซึ่งเป็นจุดสิ้นสุด scope ของ s`<br>
`Rust จะ drop ค่านั้นให้อัตโนมัติทันที ถ้าเรียกใช้ s นอก scope จะเกิด compile error เพราะ ownership ผูกติดกับ scope โดยตรง`<br>

---

### 4.5 `Memory Management`

`Memory Management คือการจัดการหน่วยความจำที่โปรแกรมขอใช้ (allocate) และคืน (deallocate) ให้ระบบ`<br>
`Rust ไม่ใช้ Garbage Collector และไม่ให้เขียน free() เอง แต่ผูกการคืนหน่วยความจำเข้ากับ Ownership + Scope โดยตรง`<br>
`ช่วยแก้ปัญหา runtime overhead ของภาษาที่มี GC และปัญหาความผิดพลาดจากการจัดการเองแบบ C/C++`<br>
`compiler แทรกการเรียก drop() ให้อัตโนมัติตอน compile time ทำให้ปลอดภัยโดยไม่มี cost ตอน runtime`<br>

```rust
fn main() {
    let s = String::from("hello"); // ขอหน่วยความจำบน heap
    println!("{}", s);
} // ออกจาก scope -> Rust คืนหน่วยความจำอัตโนมัติ ไม่ต้องเขียน free()
```
**Explanation**

`เมื่อ main() จบการทำงาน Rust ไม่ต้องรอ Garbage Collector และไม่ต้องเขียน free() เอง`<br>
`compiler จะแทรกการเรียก drop() ให้อัตโนมัติตอน compile time ทันทีที่ owner หลุด scope ทำให้คืนหน่วยความจำได้แน่นอน ไม่มี runtime overhead และไม่เสี่ยง memory leak`<br>

---

## 5. Important Syntax / Rules

| Syntax / Rule | Meaning | Example |
|---|---|---|
| `let x = y;` | `Assign ค่า — ถ้า y เป็น type บน heap (เช่น String) จะเกิด move; ถ้าเป็น type ที่มี Copy trait จะ copy อัตโนมัติ` | `let s2 = s1;` |
| `.clone()` | `คัดลอกข้อมูลบน heap แบบ deep copy ทำให้ทั้งสองตัวแปรใช้งานได้พร้อมกัน` | `let s2 = s1.clone();` |
| `{ }` | `กำหนด scope ของตัวแปร — เมื่อปิด block ตัวแปรที่เป็น owner ภายในจะถูก drop อัตโนมัติ` | `{ let s = String::from("hi"); }` |
| `drop()` | `กฟังก์ชันที่ Rust เรียกอัตโนมัติเมื่อ owner หลุด scope เพื่อคืนหน่วยความจำ (ไม่ต้องเรียกเอง)` | `เรียกอัตโนมัติตอนปิด }` |

### Important Rules

1. `ทุกๆค่า ใน Rust จะมี "เจ้าของ" (Owner) เสมอ`
2. `เมื่อ owner หลุด scope ค่านั้นถูก drop ทันที โดยอัตโนมัติ`
3. `มี owner ได้เพียง "คนเดียว" เท่านั้นในเวลาเดียวกัน`



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

`[Topic นี้เกี่ยวข้องกับ syntax อย่างไร]`

### 9.2 Semantics

`[คำสั่ง/construct เหล่านี้มีความหมายหรือพฤติกรรมอย่างไร]`

### 9.3 Type System

`[เกี่ยวข้องกับ type system อย่างไร ถ้ามี]`

### 9.4 Memory / Resource Management

`[เกี่ยวข้องกับ memory หรือ resource management อย่างไร ถ้ามี]`

### 9.5 Abstraction / Other PPL Concepts

`[อธิบาย abstraction, scope, binding, paradigm หรือแนวคิด PPL อื่นที่เกี่ยวข้อง]`

### 9.6 Why Rust?

`[Rust ใช้แนวคิดนี้เพื่อเพิ่ม safety, reliability หรือ performance อย่างไร]`

---

## 10. Rust vs. Other Language

**Comparison Language:** `[Python / C / C++ / Java / Kotlin / ...]`

| Aspect | Rust | Other Language |
|---|---|---|
| Syntax | `[อธิบาย]` | `[อธิบาย]` |
| Semantics / Behavior | `[อธิบาย]` | `[อธิบาย]` |
| Type System | `[อธิบาย]` | `[อธิบาย]` |
| Memory Management | `[อธิบาย]` | `[อธิบาย]` |
| Safety | `[อธิบาย]` | `[อธิบาย]` |

### Rust Example

```rust
// Rust code
```

### `[Other Language]` Example

```python
# Other language code
```

### Analysis

`[อธิบายความแตกต่างที่สำคัญ และเหตุผลด้านการออกแบบภาษา]`

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

| AI Tool | Purpose | How the Result Was Verified |
|---|---|---|
| `[เช่น ChatGPT]` | `[ใช้เพื่ออะไร]` | `[ตรวจสอบอย่างไร]` |
| `[AI tool]` | `[ใช้เพื่ออะไร]` | `[ตรวจสอบอย่างไร]` |

### Declaration

- [ ] Code ทุกส่วนที่นำเสนอได้รับการ Compile และทดสอบแล้ว
- [ ] สมาชิกทุกคนสามารถอธิบาย Code ที่นำเสนอได้
- [ ] ตรวจสอบข้อมูลจากแหล่งอ้างอิงที่น่าเชื่อถือแล้ว
- [ ] ระบุการใช้ AI อย่างโปร่งใส

**รายละเอียดการใช้ AI**

`[อธิบายว่าใช้ AI ในขั้นตอนใด และสมาชิกตรวจสอบผลลัพธ์อย่างไร]`

---

## 14. GitHub Contribution

| Member | Issues | Commits | Pull Requests | Code Reviews | Contribution |
|---|---:|---:|---:|---:|---|
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

**Repository:** `[GitHub repository URL]`

**Chapter Path:** `[เช่น chapters/01-introduction/]`

**Final PR:** `#[PR number]`

**Submitted by:** `[Group XX]`

**Date:** `[YYYY-MM-DD]`
