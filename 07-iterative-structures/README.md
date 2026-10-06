# Rust Tutorial Project — Principles of Programming Languages

> **กลุ่มที่:** 07
> **Topic No.:** 07
> **Topic Name:** Iterative Structures
> **ประเด็นหลักที่ควรครอบคลุม:** loop, while, for, break, continue, nested loops

---

## 1. Members

| # | Name | Student ID | GitHub Username | Main Responsibility |
|---|---|---|---|---|
| 1 | นายนวภูมิ ปั้นหลวง | 670710133 | `@670710133` | Concept + Short Code Illustration (สรุปแนวคิดหลัก + โค้ดตัวอย่างสั้น) |
| 2 | นายปฏิภาณ นิลวงค์ | 670710134 | `@670710134` | Detailed Code + Live Demo (โค้ดเชิงลึก + สาธิตสด) |
| 3 | นายปรเมทร์ ฟองดา | 670710135 | `@670710135` | Rust vs Other Language + PPL Analysis (เปรียบเทียบภาษา + วิเคราะห์เชิง PPL) |
| 4 | นายปิยวัฒน์ เดียนประไพ | 670710136 | `@670710136` | Exercises + Common Mistakes + Challenge (แบบฝึกหัด + ข้อผิดพลาดที่พบบ่อย + คำถามท้าทาย) |

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

`[เขียนเนื้อหาที่นี่ — ใช้โครงสร้างเดียวกับ rust_tutorial_template.md ฉบับเต็มที่ผู้สอนแจกให้]`

อธิบายว่า Topic นี้คืออะไร มีความสำคัญอย่างไร และใช้แก้ปัญหาอะไรในการเขียนโปรแกรม

`    Iterative Structures คือโครงสร้างควบคุมที่ทำให้โปรแกรมทำคำสั่งเดิมซ้ำตามเงื่อนไขหรือข้อมูลที่กำหนด ใน Rust รูปแบบที่ใช้ทั่วไปคือ loop, while, for และ while let โดย for ทำงานร่วมกับ iterator เพื่อดึงข้อมูลทีละค่า`

`    แนวคิดนี้มาจากความต้องการลดการเขียนคำสั่งซ้ำ ๆ เช่น แทนที่จะเขียน println! หลายครั้ง เราใช้ลูปเพื่อบอกโปรแกรมว่า “ทำคำสั่งนี้กับข้อมูลทุกตัว” หรือ “ทำซ้ำจนกว่าเงื่อนไขจะเป็นเท็จ”`

---

## 4. Key Concepts

### 4.1 `loop`

**คำอธิบาย**
    
`    loop คือการวนซ้ำแบบไม่กำหนดเงื่อนไขสิ้นสุดไว้ที่หัวลูป โปรแกรมจะทำงานต่อไปจนกว่าจะพบ break Rust Reference ระบุว่า loop เป็น infinite loop โดยธรรมชาติ หากไม่มี break ก็จะไม่สิ้นสุดตามปกติ`
    
`    การที่ Rust ออกแบบ loop มา เพื่อให้สามารถกำหนดจุดสิ้นสุดของการวนซ้ำจากภายในกระบวนการทำงานได้อย่างชัดเจน แทนที่จะบังคับให้เงื่อนไขอยู่ที่หัว loop เสมอ นอกจากนี้ Rust อนุญาตให้ loop คืนค่าผ่าน break value ได้ ทำให้ใช้เป็น expression ได้`

loop ช่วยในการแก้ไขปัญหา

`    - งานที่ไม่ทราบจำนวนรอบล่วงหน้า `

`    - โปรแกรมที่ต้องรอเหตุการณ์หรือข้อมูล `

`    - ลูปที่มีเงื่อนไขหยุดหลายจุด `

`    - การเขียนลูปที่ต้องคืนค่าผลลัพธ์ `

**ตัวอย่าง**

```rust
fn main() {
    let mut n = 1;

    loop {
        println!("{n}");

        if n == 3 {
            break;
        }

        n += 1;
    }
}
```

**Explanation**

`    โปรแกรมเริ่มจาก main() และกำหนด n = 1 โดยใช้ mut เพื่อให้สามารถเปลี่ยนค่าได้ จากนั้นเข้าสู่ loop เพื่อทำงานซ้ำ โดยแสดงค่าของ n แล้วตรวจสอบว่า n == 3 หรือไม่ หากยังไม่เท่ากับ 3 จะเพิ่มค่า n ทีละ 1 แล้ววนซ้ำอีกครั้ง เมื่อ n มีค่าเป็น 3 โปรแกรมจะแสดงเลข 3 แล้วทำคำสั่ง break เพื่อออกจาก loop และจบการทำงาน โดยผลลัพธ์คือ 1, 2, 3`

---

### 4.2 `while`

`    while คือการวนซ้ำที่ตรวจสอบเงื่อนไขก่อนทำงานแต่ละรอบ ถ้าเงื่อนไขเป็น true จึงทำงานต่อ แต่ถ้าเป็น false จะจบลูป เหมาะกับสถานการณ์ที่ "เงื่อนไขเป็นตัวกำหนดว่าควรทำต่อหรือไม่" โดยตรง ทำให้โครงสร้างของโปรแกรมอ่านง่าย`

while ช่วยในการแก้ปัญหา

`    - การวนจนกว่าค่าจะถึงขีดจำกัด`

`    - การตรวจสอบสถานะซ้ำ ๆ`

`    - การทำงานที่จำนวนรอบขึ้นอยู่กับเงื่อนไข`

`    - ลดการเขียน loop ร่วมกับ if และ break ที่ซ้ำซ้อน`

**ตัวอย่าง**

```rust

fn main(){
    let mut n = 1;

    while n <= 3 {
        println!("{n}");
        n += 1;
    }
}
```

**Explanation**

`    โปรแกรมเริ่มจาก main() และกำหนดค่า n = 1 โดยใช้ mut เพื่อให้สามารถเปลี่ยนค่าได้ จากนั้นใช้ while ตรวจสอบว่า n <= 3 หรือไม่ หากเป็นจริง โปรแกรมจะแสดงค่า n แล้วเพิ่มค่าขึ้นทีละ 1 จากนั้นวนกลับไปตรวจสอบเงื่อนไขอีกครั้ง เมื่อ n มีค่าเป็น 4 เงื่อนไขเป็นเท็จ จึงหยุดการทำงานของ while และจบโปรแกรม โดยผลลัพธ์คือ 1, 2, 3`

---

### 4.3 `for`

**คำอธิบาย**
    
`    for คือโครงสร้างวนซ้ำที่ใช้สำหรับนำข้อมูลแต่ละตัวในชุดข้อมูล เช่น Array, Vector หรือ Range มาประมวลผลทีละตัว โดย Rust จะนำข้อมูลนั้นมาเป็น Iterator แล้วส่งค่าออกมาทีละตัวจนกว่าข้อมูลจะหมด จึงช่วยให้เราไม่ต้องจัดการจำนวนรอบหรือ index ด้วยตนเอง`
    
`    การที่ Rust ออกแบบ loop มา เพื่อให้สามารถกำหนดจุดสิ้นสุดของการวนซ้ำจากภายในกระบวนการทำงานได้อย่างชัดเจน แทนที่จะบังคับให้เงื่อนไขอยู่ที่หัว loop เสมอ นอกจากนี้ Rust อนุญาตให้ loop คืนค่าผ่าน break value ได้ ทำให้ใช้เป็น expression ได้`

for ใช้แนวคิด iterator เพื่อให้การวนข้อมูล

`    - อ่านง่าย `

`    - ทำงานกับ collection ได้อย่างเป็นระบบ `

`    - ลดการใช้ index ด้วยตนเอง `

`    - ลดความเสี่ยงจากการเข้าถึงตำแหน่งนอกขอบเขต `

`    - ทำงานร่วมกับ ownership และ borrowing ได้อย่างปลอดภัย `

**ตัวอย่าง**

* การใช้ for กับ array
```rust
fn main() {
    let numbers = [10, 20, 30];

    for number in numbers {
        println!("{number}");
    }
}
```
**Explanation**

`    โปรแกรมเริ่มจาก main() และสร้างอาร์เรย์ numbers ที่เก็บค่า 10, 20, 30 จากนั้นใช้ for วนอ่านค่าทีละตัว โดยนำค่ามาเก็บไว้ในตัวแปร number แล้วใช้ println! แสดงค่าบนหน้าจอจนครบทุกตัว ผลลัพธ์คือ 10, 20, 30`

* การใช้ for กับ range
```rust
fn main() {
    for number in 1..=3 {
        println!("{number}");
    }
}
```

**Explanation**

`    โปรแกรมเริ่มจาก main() แล้วใช้ for วนค่าตั้งแต่ 1 ถึง 3 โดย 1..=3 หมายถึงรวมเลข 3 ด้วย ในแต่ละรอบจะนำค่า number มาแสดงผลด้วย println! จึงได้ผลลัพธ์เป็น 1, 2, 3`

---

### 4.4 `break`

`    break เป็นกลไกสำหรับ ยุติ loop ก่อนที่จะวนจนถึงจุดสิ้นสุดตามปกติ และสามารถระบุ label เพื่อออกจาก loop ชั้นนอกได้ โดย break สามารถใช้ได้กับ loop, while และ for`

`    ในบางครั้งโปรแกรมพบผลลัพธ์ที่ต้องการก่อนถึงเงื่อนไขปกติ Rust จึงนำหลักการของ break มาหยุดการทำงานของโปรแกรมทันทีแทนที่จะประมวลผลข้อมูลต่อโดยไม่จำเป็น เพื่อป้องกันการพบเงื่อนไขที่ผิดพลาด หยุดการค้นหาเมื่อพบข้อมูลที่ต้องการแล้ว และลดการทำงานที่ไม่จำเป็น`

**ตัวอย่าง**

```rust

fn main(){
    for n in 1..=10 {
        if n == 5 {
            break;
        }
        println!("{n}");
    }
}
```

**Explanation**

`    โปรแกรมใช้ for วนค่าตั้งแต่ 1 ถึง 10 และตรวจสอบว่า n เท่ากับ 5 หรือไม่ หากยังไม่ถึง 5 จะแสดงค่า n ออกทางหน้าจอ แต่เมื่อ n เท่ากับ 5 จะทำคำสั่ง break เพื่อหยุดการวนซ้ำทันที ดังนั้นผลลัพธ์คือ 1, 2, 3, 4`

---

### 4.5 `continue`

**คำอธิบาย**
    
`    continue ใช้สำหรับ ข้ามการทำงานที่เหลือของรอบปัจจุบัน แล้วส่งการควบคุมกลับไปที่หัวของ loop เพื่อเริ่ม iteration ถัดไป เพื่อให้สามารถข้ามข้อมูลบางรายการโดยไม่ต้องหยุด loop ทั้งหมด`

continue ช่วยในการแก้ไขปัญหา

`    - ข้ามข้อมูลที่ไม่ต้องการประมวลผล `

`    - กรองค่าบางประเภท `

`    - ลดระดับการซ้อนของโค้ด `

`    - ทำให้เงื่อนไขผิดปกติจบเร็ว `

**ตัวอย่าง**

```rust
fn main() {
    for number in 1..=5 {
        if number % 2 == 0 {
            continue;
        }

        println!("{number}");
    }
}
```

**Explanation**

`    โปรแกรมเริ่มจาก main() ใช้ for วนค่าตั้งแต่ 1 ถึง 5 และตรวจสอบว่า number เป็นเลขคู่หรือไม่ด้วย number % 2 == 0 หากเป็นเลขคู่จะใช้ continue เพื่อข้ามรอบนั้นไป แต่ถ้าเป็นเลขคี่จะแสดงค่าบนหน้าจอ ดังนั้นผลลัพธ์คือ 1, 3, 5`

---

### 4.6 `Nested Loops`

`    Nested Loop คือการเขียนลูปไว้ภายในลูปอีกชั้นหนึ่ง โดยทุกครั้งที่ outer loop ทำงาน 1 รอบ inner loop จะทำงานตามที่กำหนด นอกจากนี้ยังสามารถใช้ทั้ง break และ continue ได้ โดยถ้าไม่ระบุ label คำสั่งทั้งสองจะมีผลกับ ลูปชั้นในสุด ที่ครอบคำสั่งนั้นอยู่ หากต้องการควบคุมลูปชั้นนอก ให้ใช้ loop label เช่น 'outer `

`    Nested Loop เหมาะกับข้อมูลสองมิติ เช่น ตาราง, กระดานเกม, matrix, แถว–คอลัมน์ หรือการเปรียบเทียบข้อมูลเป็นคู่`

**ตัวอย่าง**

```rust
fn main() {
    for row in 1..=2 {
        for column in 1..=3 {
            println!("Row : {row}, Column : {column}");
        }
    }
}
```

**Explanation**

`    โปรแกรมใช้ for ซ้อนกัน 2 ชั้น โดย row วนค่าตั้งแต่ 1 ถึง 2 และในแต่ละรอบ column จะวนค่าตั้งแต่ 1 ถึง 3 จากนั้นแสดงค่า row และ column ออกทางหน้าจอ ทำให้แต่ละ row แสดง column ครบทั้ง 3 ค่า ผลลัพธ์คือ 6 บรรทัด ได้แก่ Row 1 Column 1–3 และ Row 2 Column 1–3`

---


## 5. Important Syntax / Rules

| Syntax / Rule | Meaning | Example |
|---|---|---|
| `[loop { ... }]` | `[วนคำสั่งซ้ำอย่างต่อเนื่อง จนกว่าจะใช้ break หยุด]` | `[loop { println!("Hello"); break; }]` |
| `[while condition { ... }]` | `[วนซ้ำตราบใดที่เงื่อนไขเป็น true โดยตรวจสอบเงื่อนไขก่อนทำงานแต่ละรอบ]` | `[while n <= 5 { println!("{n}"); n += 1; }]` |
| `[for pattern in expression { ... }]` | `[วนผ่านค่าจาก iterator โดยดึงค่าทีละตัวจนกว่าจะไม่มีค่าเหลือ]` | `[for n in 1..=5 { println!("{n}"); }]` |

### Important Rules

1. `[loop จะทำงานต่อเนื่อง และหากไม่มี break จะเป็น infinite loop]`
2. `[continue จะยุติ iteration ปัจจุบันและส่งการควบคุมกลับไปยังหัวของ loop]`
3. `[for ทำงานโดยอาศัย IntoIterator และดึงค่าจาก Iterator จน iterator ไม่มีค่าเหลือ]`

---

## 6. Runnable Code Examples

> **ข้อกำหนด:** Code ทุกตัวต้อง Compile และ Run ได้จริงก่อนนำมาใส่ในเอกสาร

### Example 1 — `Counter-Controlled Loop`

**Purpose:** `สาธิตวิธีการใช้งาน For loop ทั้งแบบเดินหน้าและแบบถอยหลัง`
1. เขียนลูปบวกค่าอาเรย์ตำแหน่ง 0-10 รวมตำแหน่ง 10 ด้วย

```rust
fn main(){
    let mut count = 0;
    let mut numbers = Vec::new();
    for i in 0..=10 {
        count += i;
        numbers.push(count);
        println!("numbers = ", numbers[i]);
    }
}
```

**Expected Output**

```text
numbers = 0
numbers = 1
numbers = 3
numbers = 6
numbers = 10
numbers = 15
numbers = 21
numbers = 28
numbers = 36
numbers = 45
numbers = 55
```

**Explanation**

`[อธิบาย code ทีละส่วนที่สำคัญ]`
```text
ตัวแปร count เอาไว้ใช้บวกค่าสะสมในแต่ละตำแหน่ง ส่วนตัวแปร number เป็น vector ที่สามารถเพิ่มค่าได้แบบ Dynamic
for i in 0..=10 เป็นการวนลูปจากตัวที่ 0 ถึงตำแหน่งที่ 10 สาเหตุที่ต้องมีเท่ากับเพราะ เรารวมเลข10ไปด้วย
```

2. เขียนลูปนับถอยหลังจาก 10 ถึง 0 และลูปที่นับทีละ 2 จาก 0 ถึง 20
```rs
fn main(){
    for i in (0..=10).rev(){
        println!("i = {}", i);
    }

    println!("===========================");

    for i in (0..=20).step_by(2){
        println!("i = {}", i);
    }
}
```
**Expected Output**
```text
i = 10
i = 9
i = 8
i = 7
i = 6
i = 5
i = 4
i = 3
i = 2
i = 1
i = 0
===========================
i = 2
i = 4
i = 6
i = 8
i = 10
i = 12
i = 14
i = 16
i = 18
i = 20
```
`[อธิบาย code ทีละส่วนที่สำคัญ]`
```text
for i in (0..=10).rev() rev() ทำหน้าที่วนลูปแบบย้อนกลับ ถ้าไม่มีส่วนนี้ loop นี้จะทำการวนลูปไปด้านหน้าอย่างเดียว
.step_by() ทำหน้าที่นับทีละ จำนวนครั้งที่เราต้อการ
```
---

### Example 2 — `Logically-Controlled Loop`

**Purpose:** `สาธิตวิธีการใช้งาน loop และ while`

1. จำลองโปรแกรม "รับค่าจนกว่าจะได้ค่าที่ติดลบ" และทำการหยุดลูป
```rust
use std::io;
    fn main(){
    let mut number: i32 = 0;

    while number >= 0 {
        let mut msg = String::new();
        println!("Enter a number (negative number to stop):");

        io::stdin()
            .read_line(&mut msg)
            .expect("Failed input");

        number = match msg.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a valid number!");
                0
            }
        };
            if number > 0{
            println!("Input = {}", number);
        }
    }

        println!("Stopped! You entered negative number: {}", number);
    }
```

**Expected Output**

```text
Enter a number (negative number to stop):
5
Input = 5
Enter a number (negative number to stop):
10
Input = 10
Enter a number (negative number to stop):
15
Input = 15
Enter a number (negative number to stop):
20
Input = 20
Enter a number (negative number to stop):
-1
Stopped! You entered negative number: -1
```

**Explanation**

`[อธิบาย code]`
```text
ประกาศตัวแปร number เป็น int  

io::stdin()
    .read_line(&mut msg)
    .expect("Failed input"); ส่วนนี้เราจะทำการ Error Handling เพื่อทำการรับค่าที่เป็น String


number = match msg.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a valid number!");
                0
            }
        };
ส่วนนี้เราจะทำการแปลง String ที่รับเข้ามาให้กลายเป็น int โดยใช้การ Error Handling
match keyword นี้จะทำการเช็คค่าที่แปลงจากข้อความมาแล้ว
0 คือการคืนค่าที่เป็น 0 ออกไป

{} ทำหน้าที่เป็นตัวแทนในปริ้นค่าภายในตัวแปรออกมา
```

### Example 3 — `Iteration`
**Purpose:** `สาธิตวิธีการใช้งาน loop แบบ for each` 
1. เขียนโปรแกรมที่พิมพ์ "Name: Bob", "Name: Carol", "Name: Ted" จากคอลเลกชันของสตริงสามตัว
```rs
fn main(){
    let names = vec!["Bob", "Carol","Ted"];

    for name in names{
        println!("Names --> {}", name);
    }
}
```
**Expected Output**
```text
Names --> Bob
Names --> Carol
Names --> Ted
```
**Explanation**

`[อธิบาย code]`
```text
ตัวแปร names กำหนดให้เป็น Vector เพื่อให้ใสค่าที่เราต้องการเข้าไปได้
loop จะทำการวนลูปตามจำนวนสมาชิกที่อยู่ใน Vector การเขียน loop แบบนี้คล้ายลูปของภาษา python
```

### Example 4 — `Nested Loop`
**Purpose:** `สาธิตวิธีการใช้งาน loop แบบ nested-loop`

1. เขียนโปรแกรมแสดงผล ตารางตัวเลขรูปสี่เหลี่ยมผืนผ้า ขนาด 3 X 4 โดยแต่ละช่องในตารางจะแสดงผลลัพธ์ของการนำ เลขแถวคูณกับเลขคอลัมน์​
```rs
fn main(){​

    let row = 3;​

    let column = 4;​

    for i in 1..=row{​

        for j in 1..=column{​

            print!("{} ", i * j);​

        }​

        println!();​

    }​
}​
```
**Expected Output**
```text
1 2 3 4 
2 4 6 8 
3 6 9 12
```
**Explanation**

`[อธิบาย code]`
```text
เราทำการกำหนดตัวแปร row = 3 และ column = 4 
จากนั้นเราใช้ loop for เพื่อทำการวนลูปตามจำนวน row และ column
การปริ้นค่าเราจะเอาตัว i * j  เพื่อวนรอบตามจำนวนที่คูณออกไป
```

---

## 7. Common Mistakes

### Mistake 1 — `[argument never used]`

**Problem**

`[อธิบายปัญหา]`
```text
ปํญหานี้เกิดจากการเขียน print ที่ผิด format จากตัวภาษา Rust
```

**Incorrect Code**

```rust
// Incorrect example
fn main(){
    let mut count = 0;
    let mut numbers = Vec::new();
    for i in 0..=10 {
        count += i;
        numbers.push(count);
        println!("numbers = ", numbers[i]);
    }
}
```

**Correct Code**

```rust
// Correct example
fn main(){
    let mut count = 0;
    let mut numbers = Vec::new();
    for i in 0..=10 {
        count += i;
        numbers.push(count);
        println!("numbers = {}", numbers[i]);
    }
}
```

**Why?**

`[อธิบายสาเหตุ]`
```text
เกิดจากการสับสนเรื่องของ format ในการปริ้นค่าในตัวแปร ออกมา
```
---

### Mistake 2 — `[unnecessary parentheses around `for` iterator expression AND  invalid left-hand side of assignment]`

**Problem**

`[อธิบายปัญหา]`
```text
เกิดจากการเขียน for ถอยหลังผิด Format และการ Assign ค่า ผิดฝั่ง
```

**Incorrect Code**

```rust
// Incorrect example
fn main(){
    for i in (10..=0){
        println!("i = {}", i);
    }

    println!("===========================");

    for i in (2=..20){
        println!("i = {}", i);
    }
}
```

**Correct Code**

```rust
// Correct example
fn main(){
    for i in (0..=10).rev(){
        println!("i = {}", i);
    }

    println!("===========================");

    for i in (0..=20).step_by(2){
        println!("i = {}", i);
    }
}
```

**Why?**

`[อธิบายสาเหตุ]`
```text
ใน Rust ตัวดำเนินการ Range แบบนับถอยหลังโดยตรงด้วย 10..=0 จะ ไม่ทำงาน เพราะ Range ใน Rust โดยเริ่มต้นจะสมมติว่าค่าเริ่มต้นต้อง น้อยกว่าหรือเท่ากับ ค่าสุดท้ายเสมอ

เขียนเครื่องหมายเทียบช่วงสลับตำแหน่งกัน ใน Rust ตัวดำเนินการนับรวมตัวท้าย คือ ..= ไม่ใช่ =..
```

---

### Mistake 3 — `[mismatched types]`

**Problem**

`[อธิบายปัญหา]`
```text
ปัญหาเกิดจาก ใส่ชนิดข้อมูลไม่ครบหรือใส่ parameter ไม่ครบ เขียนผิด format และ syntax
```

**Incorrect Code**

```rust
// Incorrect example
use std::io;
fn main(){
    let  number: i32 = 0;

    while number >= 0 {
        let  msg = String::new();
        println!("Enter a number (negative number to stop):");

        io::stdin()
            .read_line(msg)
            .expect("Failed input");

        if number > 0{
            println!("Input = {}", number);
        }
    }

    println!("Stopped! You entered negative number: {}", number);
}
    
```

**Correct Code**

```rust
// Correct example
 use std::io;
    fn main(){
    let mut number: i32 = 0;

    while number >= 0 {
        let mut msg = String::new();
        println!("Enter a number (negative number to stop):");

        io::stdin()
            .read_line(&mut msg)
            .expect("Failed input");

        number = match msg.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a valid number!");
                0
            }
        };
            if number > 0{
            println!("Input = {}", number);
        }
    }

        println!("Stopped! You entered negative number: {}", number);
    }
```

**Why?**

`[อธิบายสาเหตุ]`
read_line() ต้องการรับค่าแบบ &mut String เพื่อเขียนข้อมูลลงในบัฟเฟอร์สตริง แต่ในโค้ดส่ง msg ไปตรงๆ ซึ่งเป็นค่าธรรมดา

ตัวแปร number มีค่าเริ่มต้นเป็น 0 และ ไม่เคยถูกอัปเดตค่าเลย ในระหว่างการทำงานของลูป เพราะไม่ได้ดึงข้อมูลจาก msg มาแปลงเป็นตัวเลข ทำให้เงื่อนไข while เป็นจริงตลอดไป ส่งผลให้เกิด Infinite Loop

ตัวแปร number ไม่ได้ประกาศเป็น mut ทำให้อัปเดตค่าไม่ได้
การเรียกใช้ io::stdin() จำเป็นต้องดึงโมดูล std::io เข้ามาก่อน


---

### Mistake 4 — `[Option<&&str>` doesn't implement `std::fmt::Display]`

**Problem**

`[อธิบายปัญหา]`
```text
เกิดจากการใส่ พารามิเตอร์ไม่ครบ เขียนปริ้นผิด Format
```

**Incorrect Code**

```rust
// Incorrect example
fn main(){
    let names = vec!["Bob", "Carol","Ted"];
    for i in 0..3{
         println!("Names --> {}", names.get(i));
    }
}
```

**Correct Code**

```rust
// Correct example
fn main(){
    let names = vec!["Bob", "Carol","Ted"];
    for i in 0..3{
         println!("Names --> {}", names.get(i).unwarp());
    }
}
```

**Why?**

`[อธิบายสาเหตุ]`
```text
เมธอด .get(i) ในภาษา Rust ไม่ได้คืนค่าเป็นสตริงตรงๆ แต่จะคืนค่าเป็นชนิดข้อมูล Option<&str>
{} มีไว้สำหรับพิมพ์ข้อมูลระดับพื้นฐานทั่วไป แต่ Option ไม่ได้อิมพลีเมนต์ Display Trait ไว้ ทำให้ Error
```
---

## 8. Exercises

> จัดทำแบบฝึกหัด **2 ข้อ** ที่สอดคล้องกับ Topic และมีระดับความยากเหมาะสม

### Exercise 1 — `[เกมทายตัวเลข]`

**Problem**

`[เขียนโจทย์]`
```text
โปรแกรมสุ่มเลขลับ (ใช้ค่าคงที่แทนการสุ่มก็ได้ เช่น 37) แล้วให้ผู้เล่นทายจากชุดคำตอบที่กำหนดไว้ล่วงหน้า (เช่น 50, 25, 40, 37)

ข้อกำหนด

1. ทุกครั้งที่ทาย ต้องบอกว่า "มากไป" "น้อยไป" หรือ "ถูกต้อง"
2. ต้องทายอย่างน้อยหนึ่งครั้งเสมอ
3. จำกัดจำนวนครั้งที่ทายได้สูงสุด 5 ครั้ง
4. เมื่อจบเกม ให้แสดงว่าชนะหรือแพ้ และใช้ไปกี่ครั้ง
```

**Hint**

`[คำใบ้]`
1. โจทย์ต้องการ "ทำก่อน แล้วค่อยตรวจเงื่อนไข"
2. เงื่อนไขหยุดมี 2 อย่าง คือ "ทายถูก" หรือ "ครบ 5 ครั้ง" ต้องเช็กทั้งคู่
3. ระวังกรณีชุดคำตอบสั้นกว่า 5 ตัว ถ้าดึงเกินขอบเขตโปรแกรมจะ panic

**Solution**

```rust
// Solution code
use std::cmp::Ordering;

fn main() {
    let secret = 37;
    let guesses = [50, 25, 40, 37];
    let max_tries = 5;
    let limit = max_tries.min(guesses.len()); 

    let mut tries = 0;
    let mut won = false;

    loop {
        let guess = guesses[tries];
        tries += 1;

        match guess.cmp(&secret) {
            Ordering::Greater => println!("ครั้งที่ {}: ทาย {} -> มากไป", tries, guess),
            Ordering::Less => println!("ครั้งที่ {}: ทาย {} -> น้อยไป", tries, guess),
            Ordering::Equal => {
                println!("ครั้งที่ {}: ทาย {} -> ถูกต้อง!", tries, guess);
                won = true;
            }
        }

        if won || tries >= limit {
            break;
        }
    }

    if won {
        println!("ชนะ! ใช้ {} ครั้ง", tries);
    } else {
        println!("แพ้! ใช้ครบ {} ครั้งแล้ว", tries);
    }
}
```

**Explanation**

`[อธิบายแนวทางแก้]`
```text
secret คือเลขลับที่ใช้เทียบ
guesses คืออาเรย์คำตอบที่กำหนดล่วงหน้า (แทนการรับค่าจากผู้เล่นจริง)
max_tries คือจำนวนครั้งสูงสุดที่ทายได้ ตามโจทย์คือ 5
limit คือจำนวนครั้งที่ทายได้จริง โดย .min(...) เลือกค่าที่น้อยกว่าระหว่าง 5 กับความยาวอาเรย์เพื่อกันไม่ให้ดึงคำตอบเกินขอบเขตอาเรย์แล้ว panic

tries นับจำนวนครั้งที่ทายไปแล้ว และใช้เป็นดัชนีดึงคำตอบจากอาเรย์ด้วย
won เป็นธงบอกว่าทายถูกหรือยัง
ทั้งสองต้องมี mut เพราะค่าเปลี่ยนในลูป

loop คือลูปไม่รู้จบที่ไม่มีเงื่อนไขที่หัวลูป จึงทำงาน อย่างน้อยหนึ่งรอบเสมอ แล้วค่อยตรวจเงื่อนไขหยุดที่ท้ายลูป

guess.cmp(&secret) คืนค่า enum Ordering ซึ่งมี 3 ค่าเท่านั้น
match ของ Rust บังคับให้ครอบคลุมทุกกรณี ถ้าลืมกรณีใดคอมไพเลอร์จะไม่ยอมให้คอมไพล์ ต่างจาก switch ของ C ที่ถ้าไม่มี default จะไม่ทำอะไร

เงื่อนไขหยุดลูป ยุดเมื่อ อย่างใดอย่างหนึ่ง เป็นจริง คือทายถูกแล้ว หรือทายครบจำนวนที่กำหนด

สรุปผลหลังลูป
```
---

### Exercise 2 — `[ตารางสูตรคูณ]`

**Problem**

`[เขียนโจทย์]`
สร้างตารางสูตรคูณแม่ 1 ถึง 9 คูณ 1 ถึง 9

ข้อกำหนด
1. ข้ามการแสดงผลลัพธ์ที่เป็นจำนวนคี่ (ใช้ continue)
2. หยุดพิมพ์ทั้งตารางทันทีเมื่อพบผลคูณที่มากกว่า 50 เป็นครั้งแรก (ใช้ labeled break)
3. นับว่าพิมพ์ผลคูณไปทั้งหมดกี่ค่า และแสดงผลรวมของค่าที่พิมพ์

**Hint**

`[คำใบ้]`
1. ต้องใช้ for ซ้อนกันสองชั้น ช่วง 1 ถึง 9 ทั้งคู่
2. ลำดับการเช็กในลูปในสำคัญมาก ลองคิดว่าควรเช็ก "มากกว่า 50" ก่อนหรือหลังเช็ก "เป็นเลขคี่"
3. ผลคูณที่ทำให้หยุดจะไม่ถูกพิมพ์หรือนับ ตรวจให้แน่ใจว่าตรงกับที่โจทย์ต้องการ

**Solution**

```rust
// Solution code
fn main() {
    let mut count = 0;
    let mut sum = 0;

    'outer: for i in 1..=9 {
        for j in 1..=9 {
            let product = i * j;

            if product > 50 {
                break 'outer; 
            }
            if product % 2 != 0 {
                continue; 
            }

            println!("{} x {} = {}", i, j, product);
            count += 1;
            sum += product;
        }
    }

    println!("พิมพ์ทั้งหมด {} ค่า, ผลรวม = {}", count, sum);
}
```

**Explanation**

`[อธิบายแนวทางแก้]`
```text
count นับจำนวนผลคูณที่ถูกพิมพ์ และ sum เก็บผลรวมของค่าที่พิมพ์
ใส่ mut เพราะค่าเปลี่ยนในลูป และประกาศไว้นอกลูปทั้งสองชั้น

1..=9 คือช่วงแบบ รวมค่าปลายทาง

'outer: คือ label ตั้งชื่อให้ลูปนี้เพื่อให้ลูปในอ้างถึงได้

product = i * j คำนวณผลคูณ

ถ้าผลคูณมากกว่า 50 ให้ break 'outer ซึ่งออกจาก ทั้งสองลูปทันที
```
---

## 9. PPL Perspective

> วิเคราะห์เรื่อง **Iterative Structures (โครงสร้างการวนซ้ำ)** ในมุมมองของรายวิชา Principles of Programming Languages

### 9.1 Syntax
ภาษา Rust มีคำสั่งวนซ้ำ 3 รูปแบบหลักให้เลือกใช้ตามความเหมาะสมของงาน:
1. `loop` : การวนลูปแบบไร้ขีดจำกัด (Infinite Loop) วนทำงานไปเรื่อยๆ จนกว่าจะมีคำสั่งหยุด
2. `while` : การวนลูปตามเงื่อนไข (Conditional Loop) ทำงานตราบใดที่เงื่อนไขยังเป็นจริง
3. `for` : การวนลูปผ่านองค์ประกอบในชุดข้อมูล (Iterator Loop)

**จุดเด่นทาง Syntax:** Rust ตัดการใส่เครื่องหมายวงเล็บ `()` ล้อมรอบเงื่อนไขออก ทำให้ไวยากรณ์สะอาด อ่านง่าย และลดความซ้ำซ้อนในโค้ด

### 9.2 Semantics
บล็อกคำสั่ง `loop` ใน Rust มีพฤติกรรมพิเศษคือเป็น **Expression** (บล็อกคำสั่งที่คืนค่าผลลัพธ์ออกมารับไว้ที่ตัวแปรได้) ผ่านคำสั่ง `break value;`
* แตกต่างจากภาษาทั่วไปที่มองลูปเป็นเพียง **Statement** (สั่งให้ทำงานได้อย่างเดียว แต่ส่งค่าคืนออกมาที่ตัวแปรโดยตรงไม่ได้)

### 9.3 Type System
การวนลูปด้วย `for` ใน Rust ทำงานร่วมกับระบบ Type System ผ่าน Trait ที่ชื่อว่า `IntoIterator`
* คอมไพเลอร์จะตรวจสอบและระบุชนิดข้อมูล (Type) ของสมาชิกทุกตัวในลูปอย่างแม่นยำตั้งแต่ช่วง **Compile-time** ช่วยป้องกันข้อผิดพลาดจาก Type Mismatch ก่อนนำโปรแกรมไปรันจริง

### 9.4 Memory / Resource Management
การวนลูปเพื่ออ่านชุดข้อมูล (Collection) ใน Rust ถูกควบคุมอย่างเข้มงวดด้วยกฎ **Ownership & Borrowing**:
* `for item in &collection` : **Immutable Borrow** — ขอ "ยืมอ่าน" ข้อมูลอย่างเดียว (ปลอดภัยที่สุด)
* `for item in &mut collection` : **Mutable Borrow** — ขอ "ยืมเพื่อแก้ไข" ข้อมูลภายในลูป
* `for item in collection` : **Move Ownership** — "ย้ายสิทธิ์ความเป็นเจ้าของ" มายังลูป (เมื่อวนลูปจบ ข้อมูลเดิมจะถูกลบ)

**ประโยชน์:** ป้องกันปัญหาคลาสสิกอย่าง **Iterator Invalidation** (การแอบแก้ไขหรือลบข้อมูลใน Collection ขณะที่ลูปกำลังวนอ่านอยู่) และ **Data Race** ในโปรแกรมแบบ Concurrent

### 9.5 Abstraction / Other PPL Concepts
Rust ใช้แนวคิด **Zero-cost Abstractions** ในการจัดการการวนซ้ำ
* แม้เราจะเขียนโค้ดในรูปแบบระดับสูง (High-level Iterator) ที่อ่านง่าย แต่เมื่อถูก Compile แล้ว จะแปลงเป็น Machine Code ที่มีความเร็วสูงโดยไม่มี Runtime Overhead หรือ Garbage Collector มาคอยดึงประสิทธิภาพโปรแกรมให้ช้าลง

### 9.6 Why Rust?
Rust ออกแบบ Iterative Structures มาเพื่อขจัดข้อผิดพลาดร้ายแรงของผู้เขียนโปรแกรม เช่น **Off-by-one Error** และ **Out-of-bounds Access** ได้อย่างเด็ดขาดในขั้นตอน Compile-time โดยไม่ต้องพึ่งพา Runtime Garbage Collection

---

## 10. Rust vs. Other Language

**Comparison Language:** `Python`

| Aspect (ด้านการเปรียบเทียบ) | Rust | Python |
|---|---|---|
| **Syntax** | ใช้ `loop`, `while`, `for ... in ...` ไม่ต้องมีวงเล็บ `()` ล้อมรอบเงื่อนไข | ใช้ `while`, `for ... in ...` กำหนดบล็อกด้วย Indentation (การย่อหน้า) และมีโครงสร้างพิเศษอย่าง `else` ต่อท้ายลูป |
| **Semantics / Behavior** | `loop` เป็น Expression คืนค่าออกมารับไว้ที่ตัวแปรได้ผ่าน `break val;` | ลูปเป็น Statement ไม่คืนค่าออกมาโดยตรง (ต้องใช้ List Comprehension หรือสร้างตัวแปรรับค่าเอง) |
| **Type System** | Static Typing — ตรวจสอบ Type ของ Iterator และข้อมูลตั้งแต่ช่วง Compile-time | Dynamic Typing / Duck Typing — ตรวจสอบการวนลูป (`__iter__`) ตอน Runtime |
| **Memory Management** | ควบคุมการเข้าถึงด้วย **Ownership & Borrowing** โดยไม่มี Garbage Collector (Zero-cost) | ใช้ **Garbage Collector (GC)** คอยจัดการ Memory มีภาระภายนอก (Runtime Overhead) ในทุกการวนลูป |
| **Safety** | Borrow Checker ป้องกันการแก้ไข Collection ขณะวนอ่านลูปตั้งแต่อยู่ในระดับ Compile-time | หากแก้ไข Collection ระหว่างวนลูป อาจเกิดพฤติกรรมผิดพลาด (Logical Bug) หรือแจ้งเตือน `RuntimeError` |

### Rust Example
```rust
fn main() {
    let numbers = vec![10, 20, 30];

    // 1. วนลูปอ่านค่าอย่างปลอดภัยด้วยการ "ยืม" (&numbers)
    for num in &numbers {
        println!("Number: {}", num);
    }

    // 2. loop ที่เป็น Expression สามารถส่งค่าผลลัพธ์ออกมาเก็บในตัวแปรได้
    let mut count = 0;
    let result = loop {
        count += 1;
        if count == 5 {
            break count * 2; // ส่งค่า 10 ออกมาเก็บไว้ใน result
        }
    };
    println!("Result from loop expression: {}", result);
}
```

### `[Other Language]` Example Python

```python
numbers = [10, 20, 30]

# 1. วนลูปผ่าน List ใน Python (ใช้ Iterable protocol)
for num in numbers:
    print(f"Number: {num}")

# 2. การหาผลลัพธ์จากลูปต้องใช้ตัวแปรภายนอกมารับค่า (ไม่มี loop expression)
count = 0
while True:
    count += 1
    if count == 5:
        result = count * 2
        break
print(f"Result from loop: {result}")

# 3. ตัวอย่างความเสี่ยงของการแอบแก้ไข List ระหว่างวนลูปใน Python (เกิดข้อผิดพลาดตอน Runtime)
for num in numbers:
    numbers.remove(num)  # ข้อมูลจะถูกลบข้ามองค์ประกอบไปเรื่อยๆ โดยที่คอมไพเลอร์ไม่เตือนก่อนรัน
```

### Analysis

`Python ออกแบบการวนลูปโดยเน้นความสะดวกสบายและอ่านง่ายของผู้เขียนโปรแกรม (Developer Ergonomics) ผ่าน Dynamic Typing แต่ต้องแลกมาด้วย Runtime Overhead จากการทำงานของ Garbage Collector และความเสี่ยงที่โปรแกรมจะทำงานผิดพลาดขณะรันหากมีการแก้ไขข้อมูลขณะวนลูป

ในทางกลับกัน ภาษา Rust เลือกใช้ระบบ Borrow Checker และ Zero-cost Abstractions ในการจัดการการวนซ้ำ ทำให้ได้ทั้งความปลอดภัยสูงสุดตั้งแต่ขั้นตอน Compile-time และประสิทธิภาพความเร็วที่เหนือกว่าโดยไม่ต้องพึ่งพา Garbage Collector`

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

`Concept + Short Code Illustration`

**Member 2**

`Detailed Code + Live Demo`

**Member 3**

`Rust, Other Language, PPL Analysis`

**Member 4**

`Exercies, Common Mistakes, Challenge`

> สมาชิกทุกคนต้องสามารถอธิบาย Code ของกลุ่มได้ ไม่ใช่เฉพาะส่วนที่ตนเองเขียน

---

## 12. References

> แนะนำให้มีอย่างน้อย **4 แหล่งอ้างอิง** และควรใช้เอกสารทางการเป็นหลัก

1. `[The Rust Programming Language — Rust Book]`
2. `[Rust by Example / Rust Reference]`
3. `[Official documentation ที่เกี่ยวข้องกับ Topic]`
4. `[https://www.w3schools.com/rust/rust_loops_for.php]`
5. `[https://users.rust-lang.org/t/reverse-for-loops/53856]`
6. `[https://medium.com/@fennsaji/day-1-input-and-output-i-o-in-rust-with-examples-be6f9478d133]`
7. `[https://www.w3schools.com/rust/rust_loops_while.php]`
8. `[https://doc.rust-lang.org/std/error/trait.Error.html#error-source]`
9. `[https://doc.rust-lang.org/reference/expressions/loop-expr.html]`
10. `[https://doc.rust-lang.org/book/ch03-05-control-flow.html]`
11. `[https://mitaa.github.io/rust/doc/book/loops.html]`

---

## 13. AI Usage Declaration

สามารถใช้ AI เป็นเครื่องมือช่วยเรียนรู้และพัฒนาได้ แต่สมาชิกทุกคนต้องเข้าใจและสามารถอธิบายผลงานของกลุ่มได้

| AI Tool | Purpose | How the Result Was Verified |
|---|---|---|
| `[Claude]` | `[คิดโจทย์ challenge และ โจทย์ Example]` | `[https://claude.ai/share/4d50148f-b370-4e44-ab47-62483d2ef7d3]` |
| `[Gemini]` | `[หาข้อผิดพลาดของโปรแกรม]` | `[https://gemini.google.com/app/78ce5cb516c918f1?is_sa=1&is_sa=1&android-min-version=301356232&ios-min-version=322.0&campaign_id=bkws&utm_source=sem&utm_medium=paid-media&utm_campaign=bkws&pt=9008&mt=8&ct=p-growth-sem-bkws&gclsrc=aw.ds&gad_source=1&gad_campaignid=22446690916&gbraid=0AAAAApk5Bhmbyn4lsYV7JyWrTXBtH6zsu&gclid=Cj0KCQjw5vLVBhCiARIsAD56SFLMD9UA7iL7xIayzZd8H7fwbFSSts0Um9YNcMcR3dgtED1-zKyC7_IaAk_iEALw_wcB]` |
|`[Gemini]`|`[PPL Analysis เปรียบเทียบภาษา Rust กับภาษาอื่นๆ]`|`[https://gemini.google.com/share/5fc3e5b9f3a9?skid=17257cad-df3f-4503-b53c-3fc3d5f356ad]`|
| `ChatGPT` | `ใช้ในการศึกษาและเรียบเรียงบทความที่แปลมาจากเว็บไซต์ในการศึกษาเรียนรู้ รวมถึงใช้ในการศึกษา Syntax และรีวิวโค้ด` | `https://chatgpt.com/share/6abfaff4-0bdc-83ec-9bd1-035b2d4f917d` |
| `Perplexity` | `ใช้ศึกษาแนวคิดการเขียนภาษา Rust ให้เข้าใจง่ายขึ้น และยกตัวอย่างโค้ด รวมถึงอธิบายการทำงาน` | `https://www.perplexity.ai/search/d7b03064-4038-4e95-8c80-5afecd70b0f5` |

### Declaration

- [X] Code ทุกส่วนที่นำเสนอได้รับการ Compile และทดสอบแล้ว
- [X] สมาชิกทุกคนสามารถอธิบาย Code ที่นำเสนอได้
- [X] ตรวจสอบข้อมูลจากแหล่งอ้างอิงที่น่าเชื่อถือแล้ว
- [X] ระบุการใช้ AI อย่างโปร่งใส

**รายละเอียดการใช้ AI**

`[อธิบายว่าใช้ AI ในขั้นตอนใด และสมาชิกตรวจสอบผลลัพธ์อย่างไร]`
```text
Claude ใช้ในขั้นตอนการหาโจทย์ Example และคิดโจทย์ challenge ตรวจสอบจากการรันหลายๆเทสเคสแล้วเลือกเอาเทสเคสที่เหมาะสมเอาไปนำเสนอ
ขั้นตอนการหาข้อผิดพลาด เราใช้ Gemini เพื่อหาข้อผิดพลาดของโปรแกรมที่เราจะมานำเสนอในส่วนของ Live Demo เราได้ทำการตรวจสอบจากแหล่งอ้างอิงที่ทางเราได้แนบไป
ขั้นตอนการเปรียบเทียบกับภาษาอื่น ใช้ Gemini เพื่อหาการเปรียบเทียบกับภาษาอื่นที่เข้าใจได้ง่ายๆ และทำการหามุมมองของ PPL ตรวจสอบจากเอกสารที่อาจารย์ได้เอามาสอนในห้องและแหล่งอ้างอิงจากที่เราแนบไปครับ
ChatGPT ใช้ในขั้นตอนการหา concept ตรวจสอบโดยการลองทดลองรันใน Cargo Check / Build และเปรียบเทียบกับ Official Documentation
Perplexity ใช้ในการหา concept ตรวจสอบโดยเช็ตข้อมูลจากหลายแหล่งอ้างอิง และทดสอบโค้ดจริง
```
---

## 14. GitHub Contribution

| Member | Issues | Commits | Pull Requests | Code Reviews | Contribution |
|---|---:|---:|---:|---:|---|
| Member 1 | `0` | `2` | `2` | `2` | `README`, `Update syntax/rules section with loop examples` |
| Member 2 | `0` | `1` | `1` | `1` | `Update: README.md` |
| Member 3 | `0` | `1` | `1` | `1` | `docs: update PPL anaiysis and Rust vs Python comparison` |
| Member 4 | `0` | `3` | `3` | `1` | `SUCCESS: Topic Example, Common mistakes and Challenge`, `Update: details branch main`, `MERGE: origin Tle` , `Update: merge branch everyone push to main`|

### Teamwork Reflection

**How did your team collaborate?**

`[อธิบายกระบวนการทำงานร่วมกัน]`
```text
เราได้ทำการสร้าง branch แยกของแต่ละคน จากนั้นพอทำงานเสร็จตามที่ตัวเองได้รับมอบหมาย จะทำการรวม Branch เข้า main เพื่อเอางานที่เสร็จสมบูรณ์ส่งอาจารย์
```

**Problems encountered**

`[ปัญหาที่พบ]`
```text
ิbranch แต่ละคนที่เอามารวมกันบางเกิดอาการ code ทับกัน บางข้อมูลที่คนอื่นทำไว้บางส่วนหายไปบ้าง
```

**How did you solve them?**

`[วิธีแก้ปัญหา]`
```text
แก้โดยการรวมทีละ branch เข้า main เพื่อป้องกันอาการ code หายไปบางส่วน
```
---

## 15. Final Checklist

- [X] Learning Objectives ครบ 3–4 ข้อ
- [X] Key Concepts ครบถ้วน
- [X] Syntax / Rules
- [X] Runnable Code Examples
- [X] Code Compile และ Run ได้จริง
- [X] Common Mistakes
- [X] Exercises 2 ข้อ พร้อม Solutions
- [X] PPL Perspective
- [X] Rust vs Other Language
- [X] References อย่างน้อย 4 แหล่ง
- [X] AI Usage Declaration
- [X] GitHub Contribution
- [X] สมาชิกทั้ง 4 คนมีส่วนร่วม
- [X] สมาชิกทั้ง 4 คนพร้อมนำเสนอคนละ 5 นาที
- [X] สมาชิกทุกคนสามารถอธิบาย Code ของกลุ่มได้

---

## Submission Information

**Repository:** `[https://github.com/670710136/Iterative-Structures]`

**Chapter Path:** `[https://github.com/soonklang/rust-tutorial-2569/tree/main/07-iterative-structures]`

**Final PR:** `#[30]`

**Submitted by:** `[Group 7]`

**Date:** `[2026-10-02]`
---

*โครงสร้างเอกสารฉบับเต็ม (Key Concepts, Runnable Code Examples, Common Mistakes, Exercises, PPL Perspective, Rust vs Other Language, References, AI Usage Declaration, GitHub Contribution, Final Checklist) ให้ทำต่อจากจุดนี้ตาม Template หลักของวิชา (`rust_tutorial_template.md`) ที่แนบมากับใบมอบหมายงาน*
