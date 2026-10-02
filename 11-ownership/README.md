# Rust Tutorial Project — Principles of Programming Languages

> **สำหรับนักศึกษา:** ใช้ไฟล์นี้เป็น Template สำหรับจัดทำบทเรียน Rust ของกลุ่ม  
> **Topic No.:** `XX`  
> **Topic Name:** `[ชื่อหัวข้อ]`  
> **Group No.:** `XX`

---

## 1. Members

| # | Name | Student ID | GitHub Username | Main Responsibility |
|---|---|---|---|---|
| 1 | `[ชื่อ-นามสกุล]` | `[รหัส]` | `@[username]` | Concept + Code |
| 2 | `[ชื่อ-นามสกุล]` | `[รหัส]` | `@[username]` | Code + Demo |
| 3 | `[ชื่อ-นามสกุล]` | `[รหัส]` | `@[username]` | Rust vs Other Language + PPL |
| 4 | `[ชื่อ-นามสกุล]` | `[รหัส]` | `@[username]` | Exercises + Common Mistakes |

---

## 2. Learning Objectives

หลังจากศึกษา Topic นี้แล้ว ผู้เรียนสามารถ:

1. `ทำความเข้าใจกฎพื้นฐานของระบบ Ownership ในภาษา Rust`
2. `นำแนวคิด Ownership มาประยุกต์ใช้ในการเขียนโปรแกรมจริง`
3. `วิเคราะห์ระบบ Ownership ในมุมมองของวิชาหลักการภาษาคอมพิวเตอร์ (PPL)`
4. `พัฒนาทักษะการแก้ปัญหาผ่านการระบุ อธิบาย และแก้ไขข้อผิดพลาดจากคอมไพเลอร์ที่เกี่ยวข้องกับ Ownership`

---

## 3. Introduction

อธิบายว่า Topic นี้คืออะไร มีความสำคัญอย่างไร และใช้แก้ปัญหาอะไรในการเขียนโปรแกรม

`[เขียนเนื้อหาที่นี่]`

---

## 4. Key Concepts

### 4.1 `[Concept 1]`

**คำอธิบาย**

`[อธิบายแนวคิด]`

**ตัวอย่าง**

```rust
fn main() {
    println!("Hello, Rust!");
}
```

**Explanation**

`[อธิบายว่า code ทำงานอย่างไร]`

---

### 4.2 `[Concept 2]`

`[อธิบายแนวคิด]`

```rust
// Rust code
```

---

### 4.3 `[Concept 3]`

`[อธิบายแนวคิด]`

```rust
// Rust code
```

---

### 4.4 `[Concept 4 — ถ้ามี]`

`[อธิบายแนวคิด]`

```rust
// Rust code
```

---

### 4.5 `[Concept 5 — ถ้ามี]`

`[อธิบายแนวคิด]`

```rust
// Rust code
```

---

## 5. Important Syntax / Rules

| Syntax / Rule | Meaning | Example |
|---|---|---|
| `[syntax/rule]` | `[ความหมาย]` | `[ตัวอย่าง]` |
| `[syntax/rule]` | `[ความหมาย]` | `[ตัวอย่าง]` |
| `[syntax/rule]` | `[ความหมาย]` | `[ตัวอย่าง]` |

### Important Rules

1. `[กฎสำคัญข้อที่ 1]`
2. `[กฎสำคัญข้อที่ 2]`
3. `[กฎสำคัญข้อที่ 3]`

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

`Introduction + Key Concept + Important Syntax/Rule`

**Member 2**

`Runable Code Example`

**Member 3**

`PPL Perspective + Compare Rust with other languages`

**Member 4**

`Common Mistakes + Exercise`

> สมาชิกทุกคนต้องสามารถอธิบาย Code ของกลุ่มได้ ไม่ใช่เฉพาะส่วนที่ตนเองเขียน

---

## 12. References

> แนะนำให้มีอย่างน้อย **4 แหล่งอ้างอิง** และควรใช้เอกสารทางการเป็นหลัก

1. `The Rust Programming Language: https://doc.rust-lang.org/book/`
2. `Rust by Example : https://doc.rust-lang.org/rust-by-example/`
3. `W3School: https://www.w3schools.com/rust/`
4. `The Rust Programming Language Book: https://www.scs.stanford.edu/~zyedidia/docs/rust/rust_book.pdf page 82-96`

---

## 13. AI Usage Declaration

สามารถใช้ AI เป็นเครื่องมือช่วยเรียนรู้และพัฒนาได้ แต่สมาชิกทุกคนต้องเข้าใจและสามารถอธิบายผลงานของกลุ่มได้

| AI Tool | Purpose | How the Result Was Verified |
|---|---|---|
| `ChatGPT` | `translate from English to Thai` | `Human Verification` |
| `Gemini` | `find reference and document` | `Use the official website` |
| `Claude` | `Code verification and mistake correction` | `run and compile the code` |

### Declaration

- [✔] Code ทุกส่วนที่นำเสนอได้รับการ Compile และทดสอบแล้ว
- [✔] สมาชิกทุกคนสามารถอธิบาย Code ที่นำเสนอได้
- [✔] ตรวจสอบข้อมูลจากแหล่งอ้างอิงที่น่าเชื่อถือแล้ว
- [✔] ระบุการใช้ AI อย่างโปร่งใส

**รายละเอียดการใช้ AI**

`ใช้ Generative AI เป็นผู้ช่วยสนับสนุนในขั้นตอนการค้นคว้าและพัฒนาโปรเจกต์นี้ โดยมีการใช้ ChatGPT สำหรับการแปลเนื้อหาจากภาษาอังกฤษเป็นภาษาไทย (ผ่านการตรวจสอบความถูกต้องด้วยตนเอง) ใช้ Gemini ในการค้นหาแหล่งอ้างอิงและเอกสารประกอบ (ตรวจสอบความถูกต้องเทียบกับเอกสารอ้างอิงบนเว็บไซต์ทางการ) และใช้ Claude ในการวิเคราะห์โค้ดรวมถึงแก้ไขข้อผิดพลาด (ตรวจสอบความถูกต้องผ่านการรันและคอมไพล์โค้ดด้วย Rust compiler) ทั้งนี้ เนื้อหา แหล่งอ้างอิง และตรรกะของโค้ดในผลงานชิ้นสุดท้ายทั้งหมด ได้รับการตรวจสอบและยืนยันความถูกต้องโดยผู้จัดทำเองทั้งสิ้น`

---

## 14. GitHub Contribution

| Member | Issues | Commits | Pull Requests | Code Reviews | Contribution |
|---|---:|---:|---:|---:|---|
| Member 1 | `0` | `3` | `1` | `1` | `Introduction + Key Concept + Important Syntax/Rule` |
| Member 2 | `0` | `1` | `1` | `0` | `Runable Code Example` |
| Member 3 | `0` | `1` | `1` | `0` | `PPL Perspective + Compare Rust with other languages` |
| Member 4 | `0` | `6` | `2` | `0` | `Common Mistakes + Exercise + others` |

### Teamwork Reflection

**How did your team collaborate?**

`ทีมของเราทำงานร่วมกันโดยใช้ระบบควบคุมเวอร์ชัน (Version Control) และแอปพลิเคชันสื่อสารควบคู่กันเพื่อให้การทำงานมีประสิทธิภาพสูงสุด เราใช้ Git และ GitHub สำหรับการจัดการโค้ดจากศูนย์กลาง ซึ่งช่วยให้เราสามารถควบคุมเวอร์ชันของงาน แบ่งกันพัฒนาใน Branch ต่างๆ ได้พร้อมกัน และรีวิวโค้ดของทีมได้ สำหรับการสื่อสารในแต่ละวันและการประสานงานที่ต้องการความรวดเร็ว เราใช้กลุ่มใน LINE เพื่อพูดคุยเกี่ยวกับความคืบหน้าของงาน อัปเดตข้อมูล และนัดหมายเวลาทำงาน นอกจากนี้ สิ่งสำคัญคือพวกเรายังมีการแลกเปลี่ยนความรู้ โดยผลัดกันอธิบายและสอนเนื้อหางานในส่วนที่แต่ละคนรับผิดชอบให้เพื่อนร่วมทีมฟัง เพื่อให้ทุกคนมีความเข้าใจในภาพรวมของโปรเจกต์ไปพร้อมๆ กันอย่างแท้จริง`

**Problems encountered**

`ไม่คุ้นเคยกับการใช้ Git และ GitHub ทำให้เกิดปัญหาทางเทคนิค เช่น โค้ดทับซ้อนกัน (Merge Conflict), pull request และความสับสนในการจัดการ Branch`

**How did you solve them?**

`ศึกษาคำสั่งพื้นฐานที่จำเป็นของ Git เราได้เรียนรู้วิธีการ Pull ดึงข้อมูล อัปเดตงาน (Commit) และการแก้ไขปัญหา Conflict ในเครื่องของตัวเองให้เรียบร้อยก่อนที่จะ Push งานขึ้นไปยัง Repository หลัก`

---

## 15. Final Checklist

- [✔] Learning Objectives ครบ 3–4 ข้อ
- [✔] Key Concepts ครบถ้วน
- [✔] Syntax / Rules
- [✔] Runnable Code Examples
- [✔] Code Compile และ Run ได้จริง
- [✔] Common Mistakes
- [✔] Exercises 2 ข้อ พร้อม Solutions
- [✔] PPL Perspective
- [✔] Rust vs Other Language
- [✔] References อย่างน้อย 4 แหล่ง
- [✔] AI Usage Declaration
- [✔] GitHub Contribution
- [✔] สมาชิกทั้ง 4 คนมีส่วนร่วม
- [✔] สมาชิกทั้ง 4 คนพร้อมนำเสนอคนละ 5 นาที
- [✔] สมาชิกทุกคนสามารถอธิบาย Code ของกลุ่มได้

---

## Submission Information

**Repository:** `https://github.com/sotheasokea/rust-tutorial-2569`

**Chapter Path:** [View Repository Path](/11-ownership)

**Final PR:** `5`

**Submitted by:** `Group 11`

**Date:** `2026-10-02`
