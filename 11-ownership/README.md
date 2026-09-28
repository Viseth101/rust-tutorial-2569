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

### Example 1 — `Move`

**Purpose:** `การย้าย ownership`

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}", s2);
}
```

**Expected Output**

```text
hello
```

**Explanation**

`ตอนแรก s1 ownership อยู่กับ s1 พอประกาศ s2 = s1 ownership ก็ถูกย้ายไปอยู่ที่ s2 s1ก็จะไม่สามารถใช้ได้`

---

### Example 2 — `Clone`

**Purpose:** `การ clone ค่าเพื่อที่จะนำไปใช้ใน function โดยที่ownershipไม่ถูกย้ายเข้าไป function ด้วย `

```rust
fn main(){
  let s: String = String::from("Hello, world!");
  print_string(s.clone());
  println!("{}", s); 
}


fn print_string(s: String) {
  println!("{}", s);
}
```

**Expected Output**

```
Hello, world!
Hello, world!
```

**Explanation**

`ในตอนที่เราใช้ function print_string แล้วรับparameterไป ownership ก็จะถูกย้ายไปที่ print_string ทำให้ค่า s ใน main ใช้ไม่ได้ เราจึง clone s แล้วค่อยส่งเป็น parameterไปในfunction ค่าที่ส่งก็จะเป็นค่าที่ copy มาแล้วก็จะถูก drop ตอนจบ functionไป ค่า s ใน main ก็จะไม่ถูกแตะ ownership ก็ไม่ถูกย้าย

`

---

## 7. Common Mistakes

### Mistake 1 — `การพยายามใช้ค่าที่ถูกย้าย (Move) ไปแล้ว`

**Problem**

`เมื่อ my_name ถูกส่งไปยังฟังก์ชัน print_name() สิทธิ์ความเป็นเจ้าของ (ownership) ของ String จะถูกย้ายไปยังฟังก์ชันนั้น ดังนั้น my_name จึงไม่สามารถนำมาใช้งานต่อใน main() ได้`


**Incorrect Code**

[View the incorrect code](./code/common-mistake/mistake_1_moved_value_incorrect.rs)

**Correct Code**

[View the correct code](./code/common-mistake/mistake_1_moved_value_correct.rs)

**Why?**

`.clone() จะสร้าง สำเนาแบบ deep copy ของ String จัดสรรหน่วยความจำ heap ใหม่ แต่มีเนื้อหาเดียวกัน ตัว clone นี่แหละที่จะถูกย้ายเข้าไปใน print ส่วน message ตัวเดิมใน main ไม่ถูกแตะต้องเลย จึงยังใช้งานต่อได้หลังจากนั้น`


---

### Mistake 2 — `เข้าใจผิดว่าการ assign คือการ copy ทั้งที่จริงๆ คือการ move`

**Problem**

`มาจากภาษาอย่าง Python, Java หรือ JS การเขียน let s2 = s1; อาจดูเหมือนแค่สร้างตัวแปรตัวที่สองที่ชี้ไปยังข้อมูลเดียวกัน แล้วใช้ได้ทั้งสองชื่อ แต่ใน Rust สำหรับ type ที่ไม่ใช่ Copy นี่คือการ move ไม่ใช่การ copy s1 จะใช้งานไม่ได้ทันทีที่ s2 ถูกสร้างขึ้น`


**Incorrect Code**

[View the incorrect code](./code/common-mistake/mistake_2_confusing_assign_and_copy_incorrect.rs)

**Correct Code**

[View the correct code](./code/common-mistake/mistake_2_confusing_assign_and_copy_correct.rs)

**Why?**

`ใช้ .clone() ถ้าต้องการให้มีเจ้าของสองตัวจริงๆ ที่เป็นอิสระจากกัน หรือใช้แค่ s2 ต่อไป แล้วเลิกพยายามใช้ s1`

---
### Mistake 3 — `Move บางส่วนออก struct (Partial move)`

**Problem**

`การย้าย field เดียวออกจาก struct จะทำให้ struct นั้น "ใช้งานไม่ได้บางส่วน" จะใช้ struct ทั้งก้อน (หรือ field ที่ถูกย้ายไปนั้น) อีกไม่ได้ ถึงแม้ field อื่นๆ จะยังใช้งานได้ปกติก็ตาม จุดนี้มักทำให้คนงงตอนแรกที่เจอ เพราะ error message อาจดูสับสน struct ยัง "มีอยู่" แต่บาง field ในนั้นใช้ไม่ได้แล้ว`


**Incorrect Code**

[View the incorrect code](./code/common-mistake/mistake_3_partial_move_from_struct_incorrect.rs)

**Correct Code**

[View the correct code](./code/common-mistake/mistake_3_partial_move_from_struct_correct.rs)

**Why?**

`clone field นั้นถ้าต้องการใช้ทั้งสองที่ หรือ destructure struct ทั้งหมดแล้วสร้างใหม่ตามที่ต้องการ หรือจัดโครงสร้างโค้ดใหม่ให้การ move เกิดขึ้นเป็นลำดับสุดท้าย`

---
### Mistake 4 — `Move ค่าเข้าไปใน loop แล้วพยายามใช้ซ้ำ`

**Problem**

`การเรียก greet(name) ครั้งแรกจะย้าย name เข้าไปในฟังก์ชัน พอถึงรอบถัดไปของ loop name ก็ไม่มีอยู่แล้ว compiler จะฟ้องว่าการเรียกครั้งที่สองใช้ค่าที่ถูกย้ายไปแล้ว นี่เป็นข้อผิดพลาดที่พบบ่อยมากเวลาแปลงโค้ดแบบ "loop ที่ใช้ตัวแปรซ้ำ" มาจากภาษาอื่น`


**Incorrect Code**

[View the incorrect code](./code/common-mistake/mistake_4_moved_value_in_loop_incorrect.rs)

**Correct Code**

[View the correct code](./code/common-mistake/mistake_4_moved_value_in_loop_correct.rs)

**Why?**

`clone ข้างในลูปถ้าต้องการสำเนาใหม่ทุกรอบ หรือจัดโครงสร้างโค้ดใหม่ให้ฟังก์ชันรับค่าไปแล้ว return กลับมา`

---
### Mistake 5 — `Anti-Pattern: "Clone ทุกอย่าง"`

**Problem**

`แม้จะไม่ใช่ข้อผิดพลาดระดับคอมไพเลอร์ แต่นี่คือข้อผิดพลาดทางพฤติกรรม เมื่อ Borrow Checker แจ้งเตือนข้อผิดพลาด ผู้เริ่มต้นมักจะใส่ .clone() ไว้ในทุกตัวแปรเพียงเพื่อบังคับให้โค้ดสามารถคอมไพล์ผ่าน`


**Incorrect Code**

[View the incorrect (not recommended) code](./code/common-mistake/mistake_5_clone_everything_incorrect.rs)

**Correct Code**

[View the correct (recommended) code](./code/common-mistake/mistake_5_clone_everything_correct.rs)

**Why?**

`การถอยกลับมาทบทวนโครงสร้างโปรแกรมใหม่: พิจารณาว่าตัวแปรใดควรเป็นเจ้าของข้อมูลอย่างแท้จริง และให้ส่วนที่เหลือในโค้ดทำการยืม (Borrow) ไปใช้แทน`

---

## 8. Exercises

> จัดทำแบบฝึกหัด **2 ข้อ** ที่สอดคล้องกับ Topic และมีระดับความยากเหมาะสม

### Exercise 1 — `Clone or Lose It`

**Problem**

[View Problems](./code/Exercise/exercise1_Clone_or_Lose_IT.md)

**Hint**

`describe` รับ `item: String` แบบ by value ดังนั้นการเรียก `describe(item)` จะย้ายความเป็นเจ้าของออกไปจาก `item` ใน `main` ต้องหาวิธีที่ทำให้ `item` ยังใช้งานได้หลังจากนั้น โดยไม่เปลี่ยน signature ของ `describe` มีคำสั่งอะไรที่ช่วยให้คุณส่ง *สำเนา* ไปแทนตัวจริงได้บ้าง?


**Solution**

[View Solution](./code/Exercise/exercise1_solution.rs)

**Explanation**

เนื่องจากโจทย์มีข้อบังคับว่าห้ามเปลี่ยนโครงสร้างของฟังก์ชัน describe (ไม่สามารถเปลี่ยนให้ไปรับค่าแบบยืม หรือ Reference &String ได้) ฟังก์ชันนี้จึง บังคับ ว่าต้องรับสิทธิ์ความเป็นเจ้าของไปเท่านั้น

+วิธีแก้คือการใช้คำสั่ง .clone() เมื่อเราเรียกใช้ describe(item.clone()):

>โปรแกรมจะสร้างสำเนาของข้อความ "Book" ขึ้นมาใหม่ในหน่วยความจำ Heap อย่างสมบูรณ์แบบและแยกขาดจากกัน

>ฟังก์ชัน describe จะรับเอาสิทธิ์ความเป็นเจ้าของของ ตัวสำเนา นี้ไปใช้แทน และทำลายตัวสำเนานั้นทิ้งเมื่อฟังก์ชันทำงานจบ

>ตัวแปร item ต้นฉบับที่อยู่ใน main จะไม่เคยถูกย้ายสิทธิ์หรือถูกแตะต้องเลย มันจึงยังคงใช้งานได้ตามปกติและสามารถนำมาสั่งพิมพ์ในบรรทัดสุดท้ายได้

---

### Exercise 2 — `The Half-Moved Book`

**Problem**

[View Problems](./code/Exercise/exercise2_The_Half_Moved_Book.md)

**Hint**

`book.title` ถูกย้ายเข้าไปใน `make_label(book.title)` หลังจากบรรทัดนั้น `book.title` ยังใช้งานได้อยู่ไหม? แล้ว field อื่นของ `book` (เช่น `book.author`) ยังใช้ได้ปกติหรือเปล่า? นี่เป็นปัญหาแบบเดียวกับการเข้าถึง field ของ struct หลังจากบางส่วนถูกย้ายไปแล้ว ลองคิดดูว่า field ไหนที่ต้องรอดจนถึงหลังจากเรียกฟังก์ชันนั้น แล้วจะทำยังไงให้มันรอด


**Solution**

[View Solution](./code/Exercise/exercise2_solution.rs)

**Explanation**

`ปัญหาคือ **partial move**: `book.title` ถูกย้ายเข้าไปใน `make_label` ดังนั้นหลังจากบรรทัดนั้น `book.title` จะใช้งานต่อใน `println!` ที่อ้างอิงถึง `book.title` อีกครั้งไม่ได้ ส่วน `book.author` ไม่ได้รับผลกระทบเพราะไม่ได้ถูกแตะต้อง`


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
