# Rust Tutorial Project — Principles of Programming Languages

> **กลุ่มที่:** 14
> **Topic No.:** 14
> **Topic Name:** Error Handling: Option & Result
> **ประเด็นหลักที่ควรครอบคลุม:** Option, Result, Some/None, Ok/Err, error propagation, ?

---

## 1. Members

| # | Name | Student ID | GitHub Username | Main Responsibility |
|---|---|---|---|---|
| 1 | นายกันต์ธร บุตรเบ้า | 670710619 | `@[670710619]` | Concept + Short Code Illustration (สรุปแนวคิดหลัก + โค้ดตัวอย่างสั้น) |
| 2 | นางสาวฉันทณัฏฐ วิชพันธุ์ | 670710620 | `@[670710620]` | Detailed Code + Live Demo (โค้ดเชิงลึก + สาธิตสด) |
| 3 | นางสาวณัฐกฤตา บุญมี | 670710621 | `@[670710621]` | Rust vs Other Language + PPL Analysis (เปรียบเทียบภาษา + วิเคราะห์เชิง PPL) |
| 4 | นายณัฐวีร์ บุญยินดี | 670710622 | `@[670710622]` | Exercises + Common Mistakes + Challenge (แบบฝึกหัด + ข้อผิดพลาดที่พบบ่อย + คำถามท้าทาย) |

> แก้ไข GitHub Username ของแต่ละคนให้ตรงกับบัญชีจริงก่อนเริ่มทำงาน (ผู้สอนจะใช้คอลัมน์นี้เชิญเป็น collaborator ของ repository)

---

## 2. Learning Objectives

หลังจากศึกษา Topic นี้แล้ว ผู้เรียนสามารถ:

1. อธิบายแนวคิดการใช้งานของ Option,Result,match,Error Propagation และ ? operator ได้
2. สามารถนำความรู้ไปเขียนโปรแกรมโ้วยภาษา Rust ที่มีการรับจัดการกับการไม่มีข้อมูลที่ต้องการหรือ ความล้มเหลวได้
3. สามารถเข้าใจและวิเคราะห์พฤติกรรม จุดเด่น กฎของภาษาในการรับมือและจัดการ Error ในภาษา Rust เพื่อเขียนโปรแกรมได้อย่างถูกต้องและปลอดภัย
4. เปรียบเทียบภาษา Rust กับ Java, Python, Swift และเห็นความแตกต่างที่สำคัญของแต่ละภาษาที่นำมาเปรียบเทียบกันได้

---

## 3. Introduction

&nbsp;&nbsp;&nbsp;&nbsp;ในการพัตนาโปรแรมต่างๆนั้น ผู้เขียนต้องคำนึงถึงความเป็นไปได้ที่การทำงานของโปรแกรมต้องเจอกับสถานะที่ทำให้โปรแกรมทำงานผิดพลาดไปในทางที่เราไม่ต้องการ 
เช่น การค้นหาไฟล์ไม่ได้ หาข้อมูลที่ต้องการไม่ได้ หรือบัคที่ทำให้ไม่สามารถดำเนินการได้<br>
จึงมีความสำคัญที่โปรแกรมต้องสามารถชี้ถึงความผิดพลาดนั้นได้ และมีวิธีการในการรับมือเพื่อคงการทำงานต่อไป

ซึ่ง Rust เป็นภาษาที่มีแนวคิดที่ว่า ทำให้ความเป็นไปได้ในการล้มเหลวของผลลัพธ์ต่างๆเป็นชนิดข้อมูลที่บ่งบอกว่าการทำงานนั้นสำเร็จหรือไม่ 
ส่งผลให้ผู้เขียนเห็นได้ชัดเจนจาก return type ของ function ที่ใช้งาน และสามารถตัดสินใจที่รับมือกับปัญหาที่อาจเกิดขึ้นได้อย่างไร

ตัวอย่างเช่น หากฟังก์ชันมี return type เป็น `Option<T>`<br>
`fn function(a: i32) -> Option<i32>`<br>
หมายความว่าฟังก์ชันจะมีค่าเป็นชนิด i32 หรืออาจจะไม่มีค่าให้ส่งกลับมา

&nbsp;&nbsp;&nbsp;&nbsp;Rust แยกข้อผิดพลาดเป็น 2 แบบหลัก `Recoverable Error`และ `Unrecoverable Error` <br>
* `Recoverable Error` เช่น "File not found" เป็นการทำงานผิดพลาดที่เราสามารถตรวจสอบ และออกแบบให้มีรับมือกับความผิดพลาดนี้เพื่อให้โปรแกรมทำงานต่อไป<br>
* `Unrecoverable Error` เช่น การเข้าถึงตำแหน่ง array นอกเหนือจากที่สร้างไว้ ซึ่งถือว่าเป็นสิ่งที่อันตรายที่จะทำให้งานต่อไป เราจึงอาจต้องการให้มีการหยุดการทำงานของโปรแกรม โดยการเรียก `panic!`

rust มีชนิดข้อมูลที่สำคัญ 2 ชนิดในการรับมือกับสถานการณ์ที่อาจเจอข้อมูบที่ต้องการ หรือต้องผบเจอกับข้อผิดพลาด คือ `Option` และ `Result`<br>
* `Option` ใช้สำหรับรับมือกับความเป็นไปได้ที่เกิดขึ้นจากข้อมูลที่ต้องการไม่มีอยู่ และสามารถส่งที่บ่งบอกว่าไม่มีข้อมูลกลับไปได้<br>
* `Result` ใช้รับมือกับปัญหาที่เกิดขึ้นจากการทำงานที่ล้มเหลว และมีการส่งข้อมูลเกี่ยวกับความล้มเหลวนั้นมาได้<br>

---

## 4. Key Concepts

### 4.1 `Option`

**คำอธิบาย**

`Option<T>` คือ enum type ที่มี 2 variant: `Some(T)` , `None`<br>
ใช้ในการรับมือการความเป็นไปได้ที่จะไม่มีข้อมูลที่ต้องการในการคืนค่าออกมา เช่น ฟังก์ชันที่ค้นหาข้อมูลเพื่อส่งกลับไป อาจจะพบหรือไม่พบข้อมูลที่ต้องการก็ได้<br>
ซึ่งจะไม่ได้มองว่าฟังค์ชันนั้นทำงานล้มเหลว แต่การหาไม่เจอเป็นผลลัพธ์ที่เกิดขึ้นได้ตามปกติ

**ตัวอย่าง**

```rust
fn get_user(id: u32) -> Option<String> {
    if id == 1 {
        return Some("John".to_string());
    } else {
        return None;
    }
}

fn main() {
    let _result = get_user(1);
}
```

**Explanation**

&nbsp;&nbsp;&nbsp;&nbsp;ตัวอย่างฟังก์ชัน `get_user()` มี return type เป็น `Option<String>` ทำให้ผลลัพธ์มีได้ 2แบบ คือ <br>
`Some(String)` หรือ `None`

`Some(T)` ใช้แทนค่าผลลัพธ์ที่หาได้โดยคืนค่า Some(T)<br>
และมี `T` เป็น generic type ในการใส่ผลลัพธ์ชนิดนั้น จากตัวอย่างคือ Some(`“John”.to_string()`)

`None` บ่งบอกว่าไม่มีข้อมูลที่ต้องการส่งกลับมา ฟังก์ชันจะส่ง None กลับไป จึงไม่มีข้อมูลอยู่ด้านในเหนือม Some()<br>
จากฟังก์ชันตัวอย่าง หาก ID ไม่ใช่ 1 ฟังก์ชันจะส่งคืน None

---

### 4.2 `Result`
**คำอธิบาย**

`Result<T,E>` คือ enum type ที่มี 2 variant: `Ok(T)` , `Err(E)`<br>
ใช้รับมือกับความเป็นไปได้ที่จะมีการล้มเหลวจากการทำงานต่างๆ เช่น ไม่สามารถอ่านไฟล์ได้เนื่องจากไฟล์นั้นไม่มีอยู่<br>
ซึ่งเราสามารถส่งข้อมูลผลลัพธ์ที่เกี่ยวกับข้อผิดหลาดนั้นได้กลับไปได้ด้วย<br>


**ตัวอย่าง**

```rust
fn divide(a: i32,b: i32) -> Result<i32,String> {
    if b == 0 {
        return Err("Fail: denominator cannot be zero!".to_string())
    } else {
        return Ok(a / b);
    }
}

fn main() {
    let _varaint_return = divide(10,2);
}
```

**Explanation**

&nbsp;&nbsp;&nbsp;&nbsp;ตัวอย่างฟังก์ชัน `divide()` มี return type เป็น `Result<i32, String>` ทำให้การคืนค่าได้ 2แบบ คือ<br> 
`Ok(i32)` หรือ `Err(String)`

`Ok(T)` ใช้ในเหตุการณ์ที่การทำงานนั้นสำเร็จ เช่น การเจอไฟล์ที่ต้องการ<br>
จะถูกส่งกลับไปพร้อมกับ `T` (generic type) ใส่ไว้ด้านใน<br>
โดย T จะแทนข้อมูลที่เราต้องการส่งกลับไปด้วย ในกรณีที่การทำงานสำเร็จ<br>
จากตัวอย่าง หากตัวหาร b ไม่เป็น 0 จะส่งกลับค่า `Ok(a/b)`

`Err(E)` ใช้ในสถานะการที่การทำงานนั้นล้มเหลว เช่น ไม่สามารถหาไฟล์นั้นได้ หรือ ไม่สามารถสร้างไฟล์ใหม่ได้<br>
จะถูกส่งกลับไปพร้อมกับ `E` (generic type) ด้านใน<br>
โดย E จะแทนด้วยข้อมูลที่สามารถอธิบายเกี่ยวความล้มเหลวจากการนำเนินการนั้นได้<br>
จากตัวอย่าง หากตัวหาร b เป็น 0 จะส่งกลับค่า `Err("Fail: ..."to_string())`

---

### 4.3 `Match`

&nbsp;&nbsp;&nbsp;&nbsp;เมื่อได้รับค่าคืนจากฟังก์ชันเป็นข้อมูลชนิด `enum` เราสามารถเรียกใช้ `match` ในการตรวจสอบว่าค่าที่ได้มานั้นเป็น variant ใด
จากนั้นกำหนดให้เกิดการทำงานตาม variant ที่ได้คืนมา

```rust
fn main() {
    let variant: Result<String, String> = Result::Ok("hello world!".to_string());

    match variant {
        Ok(value) => println!("Result: {value}"),
        Err(_error) => println!("Error: something wrong"),
    }
}
```

**Explanation**

&nbsp;&nbsp;&nbsp;&nbsp;เมื่อ `divide()` คืนค่ามา `match` จะทำการตรวจสอบ และเลือกทำงานที่ตรงกับแต่ละกรณี<br>
`match` เป็นตัวช่วยการตัดสินใจในการดำเนินการในแต่ละกรณี

ในกรณีที่ผลลัพธ์เป็น `Ok(value)` โปรแกรมจะทำงานตามที่ `Ok` กำหนดไว้ และสามารถดึง `value` ด้านในมาใช้งานได้ด้วย<br>
ส่วนกรณีที่ผลลัพธ์เป็น `Err(error)` จะทำงานตามที่กำหนดไว้สำหรับ `Err` โดยมีข้อมูล `error` เป็นอธิบายเกี่ยวความล้มเหลวที่สามารถใช้พิจารณาได้


---

### 4.4 `Error Propagation`

&nbsp;&nbsp;&nbsp;&nbsp;เมื่อเกิดข้อผิดพลาด ฟังก์ชันนั้นจำเป็นต้องแก้ไข้ปัญหานั้นเองเสมอไป หรือหากฟังก์ชันไม่มีข้อมูลหรือ context ที่จะตัดสินใจว่าควรจะจัดการการทำงานต่ออย่างไร
ฟังก์ชันนั้นก็สามารถส่งคืนผลลัพธ์เพื่อบ่งบอกถึงการล้มเหลวนั้นกลับไป เพื่อให้ฟังช์ที่เรียกใช้ที่อาจมีข้อมูลมากกว่าตัดสินใจการทำงานต่อ<br>
เช่น การคืนค่า Err(error) ฟังก์ชันที่ได้รับค่า จะต้องกำหนดการทำงานโดยใช้ข้อมูล error ตัดสินใจการทำงานต่ออย่างไร


```rust
fn calculate() -> Result<i32, String> {
    match divide(10,0) { //ฟังก์ชันจาก 4.2
        Ok(value) => {
            println!("success!!");
            return Ok(value);
        },
        Err(error) => return Err(error),
        
    }
}

fn main() {
    match calculate() {
        Ok(value) => println!("Result:{value}"),
        Err(error) => println!("Error:{error}"),
    }
}
```

**Explanation**

&nbsp;&nbsp;&nbsp;&nbsp;`calculate()` ไม่ได้ตัดสินใจจัดการ Err เอง แต่รับค่า Result จาก divide() และส่งต่อไปยัง `maiท` ที่เรียกใช้งานและเป็นตัวจัดการกับผลลัพธ์เอง


---

### 4.5 `? operator`

&nbsp;&nbsp;&nbsp;&nbsp;`?` Operator มีการทำงานใกล้เคียงกับ `match`<br>
โดยตรวจสอบหากค่าที่คืนจาก Result เป็น `Ok(T)` จะเป็นการดึงนำค่า `T` ออกมาใช้งานต่อในฟังก์ชันเดิม และยังคงดำเนินการต่อไปในฟังก์ชันเดิม <br>
กลับกันหาก ? คืนค่าเป็น `Err` ฟังก์ชันนั้จะทำการส่ง`Err(error)`กลับออกจากฟังก์ชันปัจจุบันทันที

```rust
fn  calculator() ->  Result<i32,String> {
    let value = divide(10,2)? ; //ฟังก์ชันจาก 4.2

    println!("Result is Ok, and continue this path");
    println!("value: {}", value);

    return Ok(value + 100)
}

fn main() {
    match calculator() {
        Ok(value) => println!("{value}"),
        Err(error) => println!("{error}"),
    }
}
```
**Explanation**

`let value = divide(10,2)?` ตรวจสอบผลลัพธ์ที่ได้รับกลับมาจาก `divide()`<br>
หากผลลับได้เป็น `Ok(5)` ? จะกำหนดให้ค่า `value` เท่ากับ `5` ที่อยู่ด้านใน `Ok` และทำคำสั้งต่อไปในฟังก์ชันเดิม แล้วจากนั้น return `Ok(value + 100)` ในท้ายที่สุด<br>
แต่ในกรณีของ `Err(error)` ? จะส่ง `Err(error)` กลับจากฟังก์ชันปัจจุบันทันทีโดยไม่การทำงานต่อในฟังก์ชันนั้นเลย


---

## 5. Important Syntax / Rules

| Syntax / Rule | Meaning | Example |
|---|---|---|
| `Option<T>` | `ใช้ในการแทนที่ค่า ซึงจะมีค่าหรือไม่มีค่าก็ได้` | `let x: Option<i32> = Some(10)` |
|`Some(value)`|`แสดงว่า Option มีค่า`|`Some(10)`|
|`None`|`แสดงว่า Option ไม่มีค่า`|`let x: Option<i32> = None`|
| `Result<T, E>` | `ใช้ในการแทนที่ผลลัพธ์ที่สำเร็จหรือไม่สำเร็จก็ได้` | `let result: Result<i32, Err> = Ok(10)` |
|`Ok(value)`|`แสดงว่าผลลัพธ์สำเร็จ `|`OK(10)`|
|`Err(error)`|`แสดงว่าผลลัพธ์ไม่สำเร็จ`|`Err("Invalid input"`|
|`match`|`ใช้ตอนแยกหรือจัดการแต่ละกรณีของ Option/Result`|`match result {Ok(x),Err(e),}`|
| `let value =  function()?;` | ` ถ้า Ok/Some ให้ทำงานต่อไป แต่ถ้า Err/None ให้ส่งค่ากลับจาก function ` | `let x = get_number()?;` |
|`unwrap()`|`ดึงค่าจาก Some/Ok แต่ถ้า None/Err จะเกิด  panic`|`let x = Some(10).unwrap();`|


### Important Rules

1. `Option จัดการ 2 กรณีได้แก่ Some ,None`
2. `Result จัดการ 2 กรณี ได้แก่ Ok,Err`
3. `? สำหรับ Error/ Value Propagation และ Function ต้องมี return type ที่รองรับ propagate เช่น Result , Option `
4. `unwrap สามารถทำให้ program panic ได้หากผลลัพธ์ที่ได้ Error `

---

## 6. Runnable Code Examples

> **ข้อกำหนด:** Code ทุกตัวต้อง Compile และ Run ได้จริงก่อนนำมาใส่ในเอกสาร

### Example 1 — `[Panic]`

**Purpose:** `แสดงการทำงานของ panic`

```rust
fn cause_panic(value: i32){
    if value == 0{
        panic!("Panic! The value was zero, which is not allowed here");
    }
    println!("Value{} is fine.",value);
}
fn main() {
    cause_panic(0); 
}
```
**Expected Output**

```text
Panic! The value was zero, which is not allowed here
```

**Explanation**

`ส่วนแรกเป็นการสร้าง function ขึ้นมาสำหรับเช็คค่า integer ถ้าค่าที่รับมาตรงตามเงื่อนไข if value == 0 ดังเช่น code ตัวอย่างที่เป็น function main ที่มีการส่งค่า 0 เข้าไปใน function cause_panic ทำให้ output เกิด panic `

---

### Example 2 — `[Option]`

**Purpose:** `แสดงการใช้งานของ option `

```rust
fn find_first_a(text: &str) -> Option<usize>{
    text.find('a') 
}
fn main() {
    match find_first_a("Hello World!"){
        Some(index) => println!("The first 'a' is at index {}", index),
        None => println!("No 'a' found in the text."),
    }
}
```
**Expected Output**

```text
No 'a' found in the text.
```

**Explanation**

`อันดับแรกก็จะทำการสร้าง function ในการหาตัว a ตัวแรกโดยให้มี return type เป็น option จากนั้นใน main เราก็ทำการสร้าง match ขึ้นมาเพื่อทำการเช็คข้อความที่ใส่เข้าไปในฟังก์ชัน ซึ่งข้อความตามตัวอย่างจะไม่มีตัว a เลย ทำให้ได้ None แทน ดังนั้นข้อความที่ได้จึงเป็น No 'a' found in the text. `

---

### Example 3 — `[Result]`

**Purpose:** `แสดงการทำงานของ result`

```rust
#[derive(Debug)]
struct DivisionError{
    message: String,
}
fn divide(numerator: f64, denominator: f64) -> Result<f64, DivisionError>{
    if denominator == 0.0 {
        Err(DivisionError{
            message: "Cannot divide by Zero ".to_string(),
        })
    }else {
        Ok(numerator / denominator)
    }
}
fn main() {
    println!("{:?}",divide(5.0,2.0));
    println!("{:?}",divide(5.0,0.0));
}
```

**Expected Output**

```text
Ok(2.5)
Err(DivisionError { message: "Cannot divide by Zero " })
```

**Explanation**

`ส่วนแรกเราก็จะสร้าง ประเภทของ DivisionError สำหรับเป็นประเภท error จากนั้นสร้างfunction divide ที่มี return เป็น Result และรับพารามิเตอร์ 2 ตัว เป็น float ทั้งคู่ โดยในฟังก์ชันจะมีเงื่อนไขหาก denominator ที่รับมาเป็น 0.0 จะได้ error ซึ่ง error คือตัว DivisionError ที่เราสร้างไว้ตอนแรก และทำการระบุข้อความที่เราต้องการให้ขึ้นเมื่อตัวหารที่เราใส่เป็น 0.0 ดังตัวอย่างในส่วนของ main ที่เราเรียกใช้ค่าที่ตัวหารเป็น 2.0 และ 0.0  `

---
### Example 4 — `[Unwrap]`

**Purpose:** `การใ้ช unwrap`

```rust
use std::fs::File;
fn main(){
    let f = File::open("hello.txt").unwrap();
} 
```

**Expected Output**

```text
called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }
```

**Explanation**

`unwrap ทำงานเหมือน match ซึ่งค่า f จะมี data type เป็น file โดย unwrap สามารถเป็นค่าสำเร็จ (Ok()) หรือ ไม่สำเร็จ(Err)ได้ กรณีไม่สำเร็จจะเกิด panic จาก code ตัวอย่างจะได้ output Err เนื่องจากไม่พบไฟล์ hello.txt `

---
### Example 5 — `[Expect]`

**Purpose:** `แสดงการใช้งาน expect`

```rust
use std::fs::File;
fn main(){
     let f = File::open("hello.txt").expect("Failed to open it ");
} 
```

**Expected Output**

```text
Failed to open it : Os { code: 2, kind: NotFound, message: "No such file or directory" }
```

**Explanation**

`การใช้ expect สามารถระบุข้อความที่เราต้องการลงเป็นข้อความที่ต้องการ error อย่าง code ตัวอย่างที่เราให้เปิดไฟล์ หาก error ก็ให้ขึ้นข้อความว่าเปิดไม่ได้ `

---

### Example 6 — `[? Operator และ Error Propagation]`

**Purpose:** `แสดงการใช้ ? และ  Error Propagation`

```rust
#[derive(Debug)]
struct DivisionError{
    message: String,
}
fn divide(numerator: f64, denominator: f64) -> Result<f64, DivisionError>{
    if denominator == 0.0 {
        Err(DivisionError{
            message: "Cannot divide by Zero ".to_string(),
        })
    }else {
        Ok(numerator / denominator)
    }
}
fn calculate_division_then_add_one(num: f64, den: f64)->Result<f64,DivisionError>{
    let result = divide(num, den)?;
    Ok(result +1.0)
}
fn main(){
    println!("{:?}", calculate_division_then_add_one(5.0,2.0));
    println!("{:?}", calculate_division_then_add_one(5.0,0.0));
}
```

**Expected Output**

```text
Ok(3.5)
Err(DivisionError { message: "Cannot divide by Zero " })
```

**Explanation**

`เราจะใช้ ? แทนการเขียน match ซึ่งถ้า function divide คืนค่าออกมาเป็น Ok result ใน function calculate_division_then_add_one จะถูกบวกเพิ่ม 1.0 แล้วคืนค่าส่งกลับไปให้ main แต่ถ้าคืนค่าออกมาเป็น Error บรรทัด Ok ใน  function calculate_division_then_add_one ก็จะไม่ถูกทำงาน และจะส่งค่า Error ที่ได้จาก  function divide กลับไปที่ function main แทน  `

---

## 7. Common Mistakes

### Mistake 1 — Using .unwrap() in production code

**Problem**

`การใช้ production code นั้นถ้าใช้ .unwrap() แล้วเจอ Err จะทำให้ thread นั้นเกิดการ panic ซึ่งถ้าการ panic เกิดขึ้นที่ web server จะทำให้การจัดการคำขอ(Request Handling)หยุดทำงาน และ หากเกิดข้อผิดพลาดร้ายแรงในงานแบบ asynchronous อาจทำให้งานหยุดทำงานโดยไม่แจ้งให้ทราบล่วงหน้า`

**Incorrect Code**

```rust
fn read_port(input: &str) -> u16 {
    input.trim().parse().unwrap() // panics if input is not a valid number
}

fn main() {
    let config = "abc";

    let port = read_port(config);
    println!("Starting server on port {}", port);
}
```

**Correct Code**

```rust
use std::num::ParseIntError;

fn read_port(input: &str) -> Result<u16, ParseIntError> {
    let port: u16 = input.trim().parse()?; // returns Err to the caller instead of panicking
    Ok(port)
}

fn main() {
    let config = "abc";

    match read_port(config) {
        Ok(port) => println!("Starting server on port {}", port),
        Err(e) => println!("Invalid config: {}", e),
    }
}
```

**Why?**

`เพราะ .unwrap() ตรวจเจอข้อผิดพลาดแล้ว panic จะทำการ crash โปรแกรม แต่ถ้าเราใช้ ? แทนนั้น Errจะถูกส่งกลับไปให้ caller เลือกวิธีจัดการ`

---

### Mistake 2 — Matching on error strings instead of error variant

**Problem**

`Developer บางคนมักเช็กข้อผิดพลาดด้วยการดูข้อความ เช่น err.contains("not found") ซึ่งเปราะบางมาก เพราะข้อความ error เปลี่ยนได้ทุกเมื่อ เช่น แก้คำ แก้ภาษา หรือเปลี่ยนรูปแบบ พอข้อความเปลี่ยน โค้ดยังคอมไพล์ผ่านตามปกติ แต่เงื่อนไขที่เช็กไว้จะไม่ทำงานอีกต่อไปโดยไม่มีอะไรเตือนเลย`

**Incorrect Code**

```rust
fn find_user(id: u32) -> Result<String, String> {
    if id == 1 {
        Ok("Alice".to_string())
    } else {
        Err("user not found".to_string())
    }
}

fn main() {
    match find_user(2) {
        Ok(name) => println!("Hello {}", name),
        Err(err) => {
            if err.contains("not found") {
                println!("Creating a new user...");
            } else {
                println!("Something else went wrong");
            }
        }
    }
}
```

**Correct Code**

```rust
enum AppError {
    UserNotFound,
    DatabaseDown,
}

fn find_user(id: u32) -> Result<String, AppError> {
    match id {
        1 => Ok("Alice".to_string()),
        99 => Err(AppError::DatabaseDown),
        _ => Err(AppError::UserNotFound),
    }
}

fn main() {
    let id = 2; // try 1 (found), 2 (not found), 99 (database down)

    match find_user(id) {
        Ok(name) => println!("Hello {}", name),
        Err(AppError::UserNotFound) => println!("Creating a new user..."),
        Err(AppError::DatabaseDown) => println!("Try again later"),
    }
}
```

**Why?**

`การเช็ก error ด้วย String ทำให้โค้ดต้องไปพึ่งพาข้อความที่เขียนไว้ ซึ่งคอมไพเลอร์ไม่ได้ช่วยตรวจสอบตรงนี้ ถ้ามีการเปลี่ยนข้อความจาก user not found เป็น no such user โค้ดที่ใช้ตรวจจับ error ก็อาจไม่ทำงานโดยที่เราไม่รู้ตัว แต่ถ้าใช้ enum เราสามารถกำหนดประเภทของ error ไว้ชัดเจน ทำให้ Rust สามารถตรวจสอบผ่านระบบ type ได้
`

---

## 8. Exercises

> จัดทำแบบฝึกหัด **2 ข้อ** ที่สอดคล้องกับ Topic และมีระดับความยากเหมาะสม

### Exercise 1 — หาเลขคู่ตัวแรก (Option)

**Problem**

เขียนฟังก์ชัน `first_even` ที่รับ slice ของ `i32` แล้วคืนค่าเลขคู่ตัวแรกที่พบ
โดยมี return type เป็น `Option<i32>` (ถ้าไม่มีเลขคู่ให้คืน `None`)
จากนั้นใน `main` ให้ใช้ `match` แสดงผลทั้งสองกรณี

**Hint**

ใช้ loop ตรวจทีละตัว ถ้า `n % 2 == 0` ให้ `return Some(n)` และถ้าจบ loop แล้วยังไม่เจอให้คืน `None`

**Solution**

```rust
fn first_even(numbers: &[i32]) -> Option<i32> {
    for &n in numbers {
        if n % 2 == 0 {
            return Some(n);
        }
    }
    None
}

fn main() {
    match first_even(&[1, 3, 4, 7]) {
        Some(n) => println!("First even number: {}", n),
        None => println!("No even number found."),
    }

    match first_even(&[1, 3, 5]) {
        Some(n) => println!("First even number: {}", n),
        None => println!("No even number found."),
    }
}
```

**Expected Output**

```text
First even number: 4
No even number found.
```

**Explanation**

ฟังก์ชันวนตรวจตัวเลขทีละตัว ถ้าเจอเลขคู่จะคืน `Some(n)` ทันที แต่ถ้าวนจนครบแล้วไม่เจอ จะคืน `None`
ใน `main` เราใช้ `match` จัดการทั้ง 2 กรณี (`Some` / `None`) ซึ่ง Rust บังคับให้เราจัดการครบทุกกรณี จึงไม่มีโอกาสลืมเช็กกรณีที่ไม่มีค่า

---

### Exercise 2 — บวกเลขจากข้อความ (Result และ ?)

**Problem**

เขียนฟังก์ชัน `add_strings` ที่รับข้อความ 2 ตัว (`&str`) แปลงเป็น `i32` แล้วคืนผลบวก
โดยมี return type เป็น `Result<i32, ParseIntError>` และต้องใช้ `?` ในการส่ง error กลับ (ห้ามใช้ `unwrap`)

**Hint**

ใช้ `.parse::<i32>()?` กับข้อความแต่ละตัว ถ้าแปลงไม่ได้ `?` จะ return `Err` ออกจากฟังก์ชันให้อัตโนมัติ
อย่าลืม `use std::num::ParseIntError;`

**Solution**

```rust
use std::num::ParseIntError;

fn add_strings(a: &str, b: &str) -> Result<i32, ParseIntError> {
    let x = a.trim().parse::<i32>()?;
    let y = b.trim().parse::<i32>()?;
    Ok(x + y)
}

fn main() {
    println!("{:?}", add_strings("10", "20"));
    println!("{:?}", add_strings("10", "abc"));
}
```

**Expected Output**

```text
Ok(30)
Err(ParseIntError { kind: InvalidDigit })
```

**Explanation**

`parse::<i32>()` คืนค่าเป็น `Result` ถ้าแปลงสำเร็จ `?` จะดึงค่าออกมาใช้ต่อ แต่ถ้าล้มเหลว (เช่น `"abc"`)
`?` จะ return `Err` กลับไปให้ผู้เรียกทันที โดยบรรทัด `Ok(x + y)` จะไม่ถูกรัน
วิธีนี้ปลอดภัยกว่า `unwrap` ที่จะทำให้โปรแกรม panic (ตรงกับ Common Mistake ข้อ 1)

---

## 9. PPL Perspective

ส่วนนี้เป็นการวิเคราะห์หัวข้อ Error Handling ของภาษา Rust ผ่านมุมมอง PPL โดยแบ่งออกเป็น 6 ประเด็น ได้แก่ Syntax, Semantics, Type System, Memory / Resource Management, Abstraction / Other PPL Concepts และ Why Rust? 

### 9.1 Syntax

&emsp; ภาษา Rust ใช้แนวคิดการคืนค่าผลลัพธ์ (Return Values) ในการจัดการข้อผิดพลาดด้วยชนิดข้อมูล `Option` และ `Result` โดยจะไม่มีการใช้โครงสร้างไวยากรณ์เฉพาะสำหรับการโยน Exception (เช่น คีย์เวิร์ด `try`, `catch`, `throw`) แต่จะประยุกต์ใช้โครงสร้างไวยากรณ์พื้นฐานร่วมกับ Pattern Matching และ `?` operator ดังนี้

- **การนิยามโครงสร้างข้อมูลด้วย Enum และ Generic Parameters (`<T, E>`)**

  โดยใช้ไวยากรณ์ Enum ร่วมกับ Generic Parameters (`<T, E>`) เพื่อสร้างประเภทของข้อมูลที่ยืดหยุ่น รองรับข้อมูลชนิดใดก็ได้ สำหรับใช้แทนกรณีการทำงานที่สำเร็จและกรณีที่เกิดข้อผิดพลาด

- **การควบคุมทิศทางโปรแกรมด้วย Pattern Matching**

  โดยใช้ไวยากรณ์ `match` และ `if let` เป็นโครงสร้างหลักในการควบคุมทิศทางโปรแกรม เพื่อตรวจสอบกรณีที่เกิดข้อผิดพลาด และแกะค่าข้อมูลออกจาก `Option` และ `Result`

- **การจัดการและส่งต่อข้อผิดพลาดด้วย `?` Operator**

  โดยใช้เครื่องหมาย `?` เพื่อลดรูปโค้ดในการตรวจสอบ `Result` หรือ `Option` แทนการเขียนคำสั่ง `match` ที่ยาว โดยทำงานแบบ Short-circuiting ซึ่งจะส่งคืนข้อผิดพลาดกลับไปยังฟังก์ชันที่เรียกใช้งานทันทีโดยอัตโนมัติ

- **การดึงข้อมูลด้วย `unwrap()` และ `expect()`**

  เป็นเมธอดสำหรับดึงค่าข้อมูลข้างในของ `Option` และ `Result` ออกมาใช้งาน โดยจะทำให้โปรแกรมหยุดทำงานทันที หรือเรียกว่า panic ถ้าค่าที่ได้ไม่ใช่ค่าที่ประมวลผลสำเร็จ หรือเป็นค่า `None`/`Err`

  และ `expect()` จะแตกต่างกับ `unwrap()` ตรงที่สามารถระบุข้อความอธิบายสาเหตุของการหยุดทำงานเพิ่มเติมได้

- **การสั่งหยุดโปรแกรมด้วยคำสั่ง `panic!`**

  เป็นคำสั่งที่ใช้สั่งหยุดการทำงานของโปรแกรมทันที เมื่อเจอข้อผิดพลาดร้ายแรงที่ไม่สามารถแก้ไขหรือประมวลผลต่อได้ เช่น การหารด้วยศูนย์ (Division by Zero), การเข้าถึงข้อมูลเกินขอบเขตของอาร์เรย์ (Index Out of Bounds)


### 9.2 Semantics

- **การมอง Error เป็นค่าข้อมูลทั่วไป** นั่นคือ ภาษา Rust มองว่าข้อผิดพลาดคือค่าข้อมูลธรรมดาตัวหนึ่งที่ส่งกลับมาจากฟังก์ชันผ่าน `Result<T, E>` เหมือนกับการคืนค่าข้อมูลปกติทั่วไป ซึ่งต่างจากระบบ Exception ในภาษาอื่นตรงที่จะไม่เกิดการกระโดดข้ามลำดับการทำงานไปมา

- **การหยุดทำงานทันทีเมื่อเจอ Error ด้วย ? Operator หรือเรียกว่า Short-Circuiting** นั่นคือ `? Operator`  ทำหน้าที่เช็กผลลัพธ์ตอนที่โปรแกรมทำงานทันที โดยกรณีสำเร็จ (`Ok` หรือ `Some`) จะแกะค่าออกมาให้เรานำไปประมวลผลต่อในบรรทัดนั้นตามปกติ กรณีล้มเหลว (`Err` หรือ `None`) จะหยุดทำงานคำสั่งที่เหลือในฟังก์ชันปัจจุบันทันที แล้วส่งค่าความผิดพลาดนั้นกลับไปให้ผู้เรียกใช้งาน

- **เมื่อเกิด `panic!`** ระบบรันไทม์จะเปลี่ยนจากการส่งคืนค่าตามปกติไปเป็นการสั่งให้หน่วยการทำงานปัจจุบันหยุดทำงานทันที และจะเริ่มกระบวนการถอยย้อน Stack เพื่อทำลายตัวแปรและคืนทรัพยากร ก่อนปิดการทำงานลง

### 9.3 Type System

- **การแยกชนิดข้อมูลเพื่อขจัดปัญหา Null** นั่นคือ ในภาษา Rust จะทำการแยกความแตกต่างระหว่าง `T` (ชนิดข้อมูลที่มีค่าเสมอ) และ `Option<T>` (ชนิดข้อมูลที่อาจเป็นค่าว่าง) ออกจากกันอย่างเด็ดขาด โดยคอมไพเลอร์จะบังคับให้แกะค่าออกจาก Option<T> ก่อนส่งให้ฟังก์ชันที่ต้องการ T เสมอ ช่วยกำจัดปัญหา Null Pointer Exception ได้อย่างเด็ดขาดตั้งแต่ขั้นตอนการคอมไพล์

- **การประยุกต์ใช้ชนิดข้อมูลผลรวม (Sum Types)** นั่นคือ ภาษา Rust นิยาม `Option<T>` และ `Result<T, E>` ตามโครงสร้างชนิดข้อมูลแบบ Sum Type เพื่อบังคับให้ตัวแปรจะอยู่ในสถานะใดสถานะหนึ่งได้เพียงอย่างเดียวในขณะนั้น ได้แก่ `Option<T>` มีรูปแบบที่เป็นไปได้ 2 รูปแบบคือ `Some` หรือ `None` และ `Result<T, E>` มีรูปแบบที่เป็นไปได้ 2 ทางคือ `Ok` หรือ `Err`

- **การบังคับจัดการผลลัพธ์ด้วยแอตทริบิวต์ `#[must_use]`** นั่นคือ โครงสร้าง `Option` และ `Result` ถูกกำกับด้วยแอตทริบิวต์ `#[must_use]` ซึ่งจะบังคับให้ผู้เขียนโปรแกรมต้องนำค่าผลลัพธ์ที่คืนมาจากฟังก์ชันไปจัดการต่อเสมอ ถ้ามีการเรียกใช้ฟังก์ชันแล้วปล่อยทิ้งไว้โดยไม่นำไปจัดการต่อ คอมไพเลอร์จะทำการแจ้งเตือนทันทีในขั้นตอนการคอมไพล์

### 9.4 Memory / Resource Management

- **ภาษา Rust ประยุกต์ใช้ระบบ Ownership** ทำให้ค่าข้อมูลที่ถูกห่อหุ้มอยู่ภายใน `Option` หรือ `Result` จะถูกคืนพื้นที่หน่วยความจำ (Deallocate) อัตโนมัติทันทีเมื่อตัวแปรนั้นหมดขอบเขตการทำงาน

- **มี Null Pointer Optimization (NPO)** ซึ่งเป็นกลไกที่คอมไพเลอร์ของ Rust ใช้ปรับแต่งโครงสร้างข้อมูลประเภท `Option<T>` ในกรณีที่ `T` เป็นตัวชี้ตำแหน่งหน่วยความจำ ให้มีขนาดเท่ากับ Pointer ปกติ โดยจะไม่เสียพื้นที่หน่วยความจำเพิ่มเติมสำหรับเก็บสถานะ `None` หรือค่าว่าง

- **ตัวแปรประเภท Enum ใน Rust มีพฤติกรรมเป็น Value Type** โดยโครงสร้างข้อมูลจะถูกจัดเก็บ ประมวลผล และส่งผ่านบน Stack โดยตรง (ยกเว้นสมาชิกภายในจะมีการอ้างอิงไปยัง Heap)

- **เมื่อโปรแกรมเกิด `panic!`** รันไทม์ของ Rust จะไล่ทำลายตัวแปรใน Stack ออกทีละตัวโดยอัตโนมัติผ่านการทำงานของ `Drop Trait` ทำให้มั่นใจได้ว่าหน่วยความจำและทรัพยากรต่าง ๆ จะถูกคืนให้ระบบทั้งหมด

### 9.5 Abstraction / Other PPL Concepts

- **รูปแบบการเขียนโปรแกรม (Paradigm):** เป็นการรวมข้อดีระหว่างการเขียนโปรแกรมแบบ Functional และการเขียนโปรแกรมแบบ Imperative เข้าด้วยกัน โดยใช้แนวคิด Errors as Values ซึ่งเป็นการมองข้อผิดพลาดเป็นค่าข้อมูลธรรมดาที่ส่งกลับจากฟังก์ชัน มาทำงานร่วมกับตัว `? Operator` ที่ทำหน้าที่ส่งคืนข้อผิดพลาดกลับทันทีเมื่อเกิดปัญหา ทำให้สามารถเขียนโค้ดเรียงลำดับขั้นตอนจากบนลงล่างได้อย่างสั้นกระชับ อ่านง่ายและตรงไปตรงมาในรูปแบบ Imperative

- **การจำแนกประเภทข้อผิดพลาด** โดย Rust แบ่งการจัดการข้อผิดพลาดออกเป็น 2 ระดับ ได้แก่

  - **ข้อผิดพลาดที่จัดการได้ (Recoverable Errors)** เป็นข้อผิดพลาดปกติทั่วไปที่ระบบคาดการณ์ไว้อยู่แล้วว่าเกิดขึ้นได้ เช่น การหาไฟล์ไม่เจอ หรือการที่ผู้ใช้ป้อนข้อมูลผิดรูปแบบ โดย Rust จะส่งผลลัพธ์กลับมาเป็นค่าข้อมูลผ่าน `Result<T, E>` หรือ `Option<T>` เพื่อให้สามารถเขียนโปรแกรมเพื่อจัดการข้อผิดพลาดนั้นได้

  - **ข้อผิดพลาดที่จัดการไม่ได้ (Unrecoverable Errors)** เป็นข้อผิดพลาดร้ายแรงจาก Bug ของโปรแกรม เช่น การหารด้วยศูนย์ (Division by Zero), การเข้าถึงข้อมูลเกินขอบเขตของอาร์เรย์ (Index Out of Bounds) ซึ่งเป็นจุดที่ระบบทำงานผิดปกติและไม่ควรฝืนประมวลผลต่อ โดย Rust จะสั่งหยุดการทำงานทันทีด้วยคำสั่ง `panic!` เพื่อป้องกันข้อมูลเสียหาย

- **การซ่อนรายละเอียดข้อมูล (Data Abstraction)** โดยภาษา Rust ซ่อนค่าข้อมูลและสถานะข้อผิดพลาดไว้ภายในโครงสร้าง `Option` หรือ `Result` โดยไม่ให้เข้าถึงตำแหน่งหน่วยความจำโดยตรง แต่จะบังคับให้เข้าถึงและใช้งานผ่านอินเทอร์เฟซที่ภาษากำหนดไว้เท่านั้น เช่น การใช้ Pattern Matching หรือเมธอดสำหรับการแกะค่า

- **ขอบเขตการทำงานและการผูกค่าตัวแปร (Scope & Binding)** โดยการแกะค่าข้อมูลด้วยโครงสร้าง Pattern Matching ได้แก่ `match` หรือ `if let Some(data) = result` ภาษา Rust จะสร้างตัวแปรใหม่ (data) เพื่อทำ Binding กับค่าภายในทันที โดยตัวแปรนี้จะมีขอบเขตการทำงานอยู่เฉพาะภายในบล็อก `{ }` ของเงื่อนไขนั้น ๆ เท่านั้น และจะถูกทำลายทันทีเมื่อออกจากบล็อก

### 9.6 Why Rust?

### **ความปลอดภัย (Safety)**

- **กำจัดปัญหา Null Pointer Exception** นั่นคือ ภาษา Rust ไม่รองรับการกำหนดค่า null ให้กับตัวแปรทั่วไป แต่จะบังคับให้ใช้ `Option<T>` ในการจัดการค่าว่าง จึงช่วยกำจัดปัญหา Null Pointer Exception ได้อย่างเด็ดขาด

- **ป้องกันการนำข้อมูลที่ไม่ถูกต้องไปใช้งาน** โดยคอมไพเลอร์จะบังคับให้ตรวจสอบสถานะและแกะค่าออกจาก `Option` หรือ `Result` ก่อนนำข้อมูลภายใน (T) ไปใช้งานเสมอ ช่วยป้องกันการนำข้อมูลที่ไม่ถูกต้องหรือข้อผิดพลาดไปใช้โดยที่เราไม่ตั้งใจ

- **ป้องกันความผิดพลาดของระบบด้วย `panic!`** โดย Rust จะสั่งให้โปรแกรมหยุดทำงานทันที (`panic!`) เมื่อเกิด Bug ร้ายแรง เพื่อไม่ให้โปรแกรมฝืนทำงานต่อด้วยข้อมูลที่ผิดพลาด

### **ความน่าเชื่อถือ (Reliability)**

- **การจัดการข้อผิดพลาดใน Rust ทำงานเหมือนกับการคืนค่าจากฟังก์ชันตามปกติ** จะไม่มีการข้ามขั้นตอนการทำงานไปที่บล็อกอื่นแบบกะทันหัน ทำให้การทำงานเป็นไปตามลำดับจากบนลงล่าง สามารถอ่านและตรวจสอบได้ง่าย

- **ป้องกันการละเลยข้อผิดพลาดด้วย `#[must_use]`** นั่นคือ โครงสร้าง `Result` และ `Option` มี attribute `#[must_use]` กำกับไว้ หากมีการเรียกฟังก์ชันที่คืนค่าเหล่านี้โดยไม่นำผลลัพธ์ไปจัดการต่อ คอมไพเลอร์จะทำการแจ้งเตือนทันที ช่วยป้องกันไม่ให้ข้อผิดพลาดถูกมองข้ามโดยที่เราไม่ตั้งใจ

### **ประสิทธิภาพ (Performance)**

- **ไม่มี Runtime Overhead บน Heap** โดยการส่งผ่านและคืนค่า `Result` และ `Option` ทำงานผ่านการคืนค่าโครงสร้างข้อมูลแบบ Enum บน Stack Frame เหมือนกับการคืนค่าฟังก์ชันทั่วไป จึงไม่มีภาระในการจองพื้นที่หน่วยความจำแบบไดนามิก และไม่ทำให้เกิด Overhead บน Heap Memory

---

## 10. Rust vs. Other Language

<p><strong>เปรียบเทียบภาษา Rust กับภาษา Java, Python และ Swift โดยแบ่งการเปรียบเทียบออกเป็น 3 ตาราง ดังนี้</strong></p>

<h3>ตารางที่ 1: เปรียบเทียบ Rust และ Java</h3>

<table>
<thead>
<tr>
<th>Aspect</th>
<th>Rust</th>
<th>Java</th>
</tr>
</thead>

<tbody>

<tr>
<td valign="top"><strong>Syntax</strong></td>

<td valign="top">
<ul>
<li><strong>จัดการข้อผิดพลาดด้วย <code>Option&lt;T&gt;</code> และ <code>Result&lt;T, E&gt;</code></strong> โดย <code>Option&lt;T&gt;</code> ใช้จัดการกรณีที่ไม่มีข้อมูล (ค่าว่าง) ส่วน <code>Result&lt;T, E&gt;</code> ใช้จัดการกรณีที่เกิดข้อผิดพลาดซึ่งสามารถแก้ไขได้</li>

<li><strong>กำหนดโครงสร้างด้วย <code>Enum</code> และ รูปแบบตัวเลือกย่อย (<code>Variant</code>)</strong> ได้แก่ <code>Some/None</code> สำหรับ <code>Option</code> และ <code>Ok/Err</code> สำหรับ <code>Result</code></li>

<li><strong>แกะค่าข้อมูลด้วย <code>Pattern Matching</code></strong> โดยใช้ไวยากรณ์ <code>match</code> และ <code>if let</code></li>

<li><strong>ส่งต่อข้อผิดพลาดอย่างรวดเร็วด้วย <code>? Operator</code></strong></li>

<li><strong>เมธอดดึงค่าข้อมูล <code>unwrap()</code> และ <code>expect()</code></strong> ใช้แกะเอาค่าข้างใน <code>Option</code> หรือ <code>Result</code> ออกมาใช้งาน หากเจอข้อผิดพลาด (<code>None/Err</code>) จะสั่งหยุดโปรแกรมทันที (<code>panic!</code>) โดย <code>expect()</code> สามารถใส่ข้อความอธิบายสาเหตุเพิ่มเติมได้</li>

<li><strong>คำสั่ง <code>panic!</code></strong> เป็นการสั่งหยุดโปรแกรมทันที ใช้เมื่อเจอข้อผิดพลาดร้ายแรงที่ไม่สามารถแก้ไขหรือประมวลผลต่อได้</li>

<td valign="top">
<ul>
<li>
<strong>จัดการข้อผิดพลาดด้วยโครงสร้างบล็อก <code>try-catch-finally</code></strong> โดย
<ul>
<li><code>try:</code> เป็นบล็อกสำหรับใส่คำสั่งที่มีโอกาสเกิดข้อผิดพลาด</li>
<li><code>catch:</code> เป็นบล็อกดักจับและจัดการ <code>Exception</code> ตามลำดับชั้น <code>Polymorphism</code> (ต้องเรียงจากคลาสลูกไปคลาสแม่)</li>
<li><code>finally:</code> บล็อกที่ได้รับการประมวลผลเสมอ ไม่ว่าจะเกิด <code>Exception</code> หรือไม่ก็ตาม</li>
</ul>
</li>

<li><strong>ระบุข้อผิดพลาดบนส่วนหัวเมธอด</strong> ใช้คำสั่ง <code>throws</code> เพื่อบอกว่าเมธอดนั้นมีโอกาสโยน <code>Exception</code> ออกไปให้ผู้เรียกใช้งานต้องจัดการต่อ</li>
</ul>
</td>
</tr>

<tr>
<td valign="top"><strong>Semantics / Behavior</strong></td>

<td valign="top">
<ul>
<li><strong>มอง <code>Error</code> เป็นค่าข้อมูลปกติ</strong> ที่ถูกส่งคืนจากฟังก์ชันเหมือนการคืนค่าทั่วไปผ่านโครงสร้าง <code>Result&lt;T, E&gt;</code></li>

<li><strong><code>?</code> Operator ทำงานแบบ <code>Short-circuiting</code></strong> โดยหากประมวลผลแล้วเกิดข้อผิดพลาด ระบบจะทำการคืนค่าข้อผิดพลาดกลับไปยังฟังก์ชันผู้เรียกทันทีโดยอัตโนมัติ</li>

<li><strong>คำสั่ง <code>panic!</code></strong> จะสั่งหยุดหน่วยการทำงานปัจจุบันทันทีเมื่อเกิดข้อผิดพลาดร้ายแรง โดยระบบจะถอยย้อน <code>Stack</code> เพื่อเคลียร์ตัวแปรและคืนทรัพยากร ก่อนปิดการทำงานลง</li>
</ul>
</td>

<td valign="top">
<ul>
<li><strong>เมื่อคำสั่ง <code>throw</code> ทำงาน</strong> ระบบรันไทม์จะหยุดการประมวลผลในขอบเขตปัจจุบันทันที และเริ่มกระบวนการถอยย้อน <code>Stack</code> โดยการยกเลิก <code>Stack Frame</code> จะไล่ย้อนกลับไปตามลำดับการเรียกใช้งาน เพื่อค้นหาบล็อก <code>catch</code> ที่มีชนิดข้อมูลตรงกับวัตถุ <code>Exception</code> นั้นมาจัดการ</li>
</ul>
</td>
</tr>

<tr>
<td valign="top"><strong>Type System</strong></td>

<td valign="top">
<ul>
<li><strong>การแยกชนิดข้อมูล <code>T</code> และ <code>Option&lt;T&gt;</code></strong> โดยเป็นการแยก <code>T</code> (ชนิดข้อมูลที่มีค่าเสมอ) และ <code>Option&lt;T&gt;</code> (ชนิดข้อมูลที่อาจเป็นค่าว่าง) ออกจากกันอย่างเด็ดขาด ช่วยตัดปัญหา <strong>Null Pointer Exception</strong> ออกไปได้ตั้งแต่ขั้นตอนคอมไพล์</li>

<li><strong>ตัวแปรจะอยู่ในสถานะใดสถานะหนึ่งได้เพียงอย่างเดียวในขณะนั้น</strong> เช่น <code>Option</code> (<code>Some</code> หรือ <code>None</code>) และ <code>Result</code> (<code>Ok</code> หรือ <code>Err</code>)</li>

<li><strong>มีแอตทริบิวต์ <code>#[must_use]</code></strong> บังคับให้ผู้เขียนโปรแกรมต้องนำผลลัพธ์ไปจัดการต่อเสมอ หากละเลยคอมไพเลอร์จะแจ้งเตือนทันที</li>
</ul>
</td>

<td valign="top">
<ul>
<li>
<strong>เป็นลำดับชั้นชนิดข้อมูล</strong> โดยระบบ <code>Type</code> ใน Java มีคลาสสูงสุดคือ <code>java.lang.Throwable</code> ซึ่งถูกแบ่งออกเป็น 2 ประเภทหลักคือ
<ul>
<li><code>java.lang.Error</code> (ข้อผิดพลาดรุนแรงในระดับ JVM ที่โปรแกรมไม่ควรจัดการ)</li>
<li><code>java.lang.Exception</code> (ข้อผิดพลาดจากลอจิกหรือปัจจัยภายนอกที่โปรแกรมสามารถดักจับได้)</li>
</ul>
</li>

<li><strong>มี <code>Checked Exceptions</code></strong> ซึ่งเป็น Exceptions ประเภทที่บังคับให้ตรวจสอบหรือจัดการตอนคอมไพล์</li>

<li><strong>การดักจับด้วย <code>Polymorphism</code></strong> นั่นคือ บล็อก <code>catch (Exception e)</code> สามารถดักจับ Exception ย่อยทุกตัวที่สืบทอดมาจากคลาส <code>Exception</code> ได้ทันที</li>
</ul>
</td>
</tr>

<tr>
<td valign="top"><strong>Memory Management</strong></td>

<td valign="top">
<ul>
<li><strong>คืนทรัพยากรอัตโนมัติด้วยระบบ <code>Ownership</code></strong> ทันทีที่ <code>Option</code> หรือ <code>Result</code> หมดขอบเขตการทำงาน (<code>Scope</code>)</li>

<li><strong>มี <code>Null Pointer Optimization (NPO)</code></strong></li>

<li><strong>ไม่มีการสร้าง <code>Overhead</code> บน <code>Heap</code></strong> เพราะโครงสร้าง <code>Enum</code> มีพฤติกรรมเป็น <code>Value Type</code> ซึ่งข้อมูลจะถูกประมวลผลและส่งผ่านบน <code>Stack</code></li>
</ul>
</td>

<td valign="top">
<ul>
<li><strong>สร้าง Overhead บน Heap</strong> โดยคำสั่ง <code>throw</code> บังคับให้ JVM สร้างออบเจกต์ <code>Exception</code> บน <code>Heap Memory</code> พร้อมกับบันทึก <code>Stack Trace</code> ทำให้กินหน่วยความจำและเกิด <code>Runtime Overhead</code></li>

<li><strong>คืนพื้นที่ด้วย Garbage Collector (GC)</strong> โดยเมื่อจัดการ <code>Exception</code> ในบล็อก <code>catch</code> เสร็จและไม่มีการใช้งานต่อ ระบบ <code>GC</code> จะเข้ามาล้างออบเจกต์ออกจาก <code>Heap Memory</code> และคืนพื้นที่ให้อัตโนมัติ</li>
</ul>
</td>
</tr>

<tr>
<td valign="top"><strong>Safety</strong></td>

<td valign="top">
<ul>
<li><strong>การันตีความปลอดภัยตั้งแต่ขั้นตอนคอมไพล์</strong> โดยคอมไพเลอร์จะบังคับให้โปรแกรมเมอร์ต้องเขียนโค้ดรองรับทุกกรณีของข้อผิดพลาดที่อาจเกิดขึ้นขณะรันโปรแกรม (<code>Runtime</code>) ไว้ล่วงหน้าเสมอ ทำให้โปรแกรมไม่ Crash โดยที่ไม่ได้คาดคิด</li>

<li><strong>การันตีการตรวจสอบครบทุกกรณี</strong> โดยคอมไพเลอร์บังคับให้ต้องเขียนโค้ดรองรับทุก <code>Variant</code> ของ <code>Enum</code> (<code>Ok/Err</code> หรือ <code>Some/None</code>) ให้ครบถ้วน เพื่อป้องกันไม่ให้มีเงื่อนไขตกหล่น</li>

<li><strong>การันตีว่า Error จะไม่ถูกละเลย</strong> โดยคอมไพเลอร์บังคับให้ต้องจัดการค่า <code>Result</code> ที่คืนกลับมาเสมอ ไม่สามารถเรียกฟังก์ชันแล้วปล่อยผ่านไปเฉย ๆ ได้</li>

<li><strong>ป้องกันความผิดพลาดของระบบด้วย <code>panic!</code></strong> โดยจะสั่งหยุดโปรแกรมทันทีเมื่อเกิด Bug ร้ายแรง เพื่อการันตีว่าระบบจะไม่ฝืนทำงานต่อด้วยข้อมูลที่ผิดพลาด</li>
</ul>
</td>

<td valign="top">
<ul>
<li><strong>มีความเสี่ยงที่โปรแกรมจะเกิดการทำงานผิดพลาดและล่มลงในขณะที่ทำงานอยู่</strong> ถ้าไม่ได้เขียนบล็อก <code>catch</code> ดักจับไว้ให้ครอบคลุม Error นั้น</li>

<li><strong>เสี่ยงเกิด <code>NullPointerException</code> ได้ง่าย</strong> เพราะคอมไพเลอร์บังคับตรวจจับเฉพาะ <code>Checked Exceptions</code> เท่านั้น</li>
</ul>
</td>
</tr>

</tbody>
</table>


<h3>ตารางที่ 2: เปรียบเทียบ Rust และ Python</h3>

<table>
<thead>
<tr>
<th>Aspect</th>
<th>Rust</th>
<th>Python</th>
</tr>
</thead>

<tbody>

<tr>
<td valign="top"><strong>Syntax</strong></td>

<td valign="top">
<ul>
<li><strong>จัดการข้อผิดพลาดด้วย <code>Option&lt;T&gt;</code> และ <code>Result&lt;T, E&gt;</code></strong> โดย <code>Option&lt;T&gt;</code> ใช้จัดการกรณีที่ไม่มีข้อมูล (ค่าว่าง) ส่วน <code>Result&lt;T, E&gt;</code> ใช้จัดการกรณีที่เกิดข้อผิดพลาดซึ่งสามารถแก้ไขได้</li>

<li><strong>กำหนดโครงสร้างด้วย <code>Enum</code> และ รูปแบบตัวเลือกย่อย (<code>Variant</code>)</strong> ได้แก่ <code>Some/None</code> สำหรับ <code>Option</code> และ <code>Ok/Err</code> สำหรับ <code>Result</code></li>

<li><strong>แกะค่าข้อมูลด้วย <code>Pattern Matching</code></strong> โดยใช้ไวยากรณ์ <code>match</code> และ <code>if let</code></li>

<li><strong>ส่งต่อข้อผิดพลาดอย่างรวดเร็วด้วย <code>? Operator</code></strong></li>

<li><strong>เมธอดดึงค่าข้อมูล <code>unwrap()</code> และ <code>expect()</code></strong> ใช้แกะเอาค่าข้างใน <code>Option</code> หรือ <code>Result</code> ออกมาใช้งาน หากเจอข้อผิดพลาด (<code>None/Err</code>) จะสั่งหยุดโปรแกรมทันที (<code>panic!</code>) โดย <code>expect()</code> สามารถใส่ข้อความอธิบายสาเหตุเพิ่มเติมได้</li>

<li><strong>คำสั่ง <code>panic!</code></strong> เป็นการสั่งหยุดโปรแกรมทันที ใช้เมื่อเจอข้อผิดพลาดร้ายแรงที่ไม่สามารถแก้ไขหรือประมวลผลต่อได้</li>

<td valign="top">
<ul>
<li>
<strong>จัดการข้อผิดพลาดด้วยโครงสร้างบล็อก <code>try-except-else-finally</code></strong> โดย
<ul>
<li><code>try:</code> เป็นบล็อกสำหรับใส่คำสั่งที่มีโอกาสเกิดข้อผิดพลาด</li>
<li><code>except:</code> เป็นบล็อกสำหรับระบุประเภท <code>Exception</code> ที่ต้องการดักจับเพื่อจัดการ</li>
<li><code>else:</code> เป็นบล็อกที่จะประมวลผลเฉพาะเมื่อไม่พบ <code>Exception</code> ใดๆ ในบล็อก <code>try</code></li>
<li><code>finally:</code> เป็นบล็อกที่จะประมวลผลเสมอ ไม่ว่าจะเกิด <code>Exception</code> หรือไม่ก็ตาม</li>
</ul>
</li>

<li><strong>การส่ง Exception ออกไป</strong> จะใช้คำสั่ง <code>raise</code> เมื่อเกิดเงื่อนไขที่ผิดปกติ</li>

<li><strong>การตรวจสอบค่าว่าง</strong> จะตรวจสอบตัวแปรที่อาจเป็นค่าว่างด้วยไวยากรณ์ <code>is None</code></li>
</ul>
</td>
</tr>

<tr>
<td valign="top"><strong>Semantics / Behavior</strong></td>

<td valign="top">
<ul>
<li><strong>มอง <code>Error</code> เป็นค่าข้อมูลปกติ</strong> ที่ถูกส่งคืนจากฟังก์ชันเหมือนการคืนค่าทั่วไปผ่านโครงสร้าง <code>Result&lt;T, E&gt;</code></li>

<li><strong><code>?</code> Operator ทำงานแบบ <code>Short-circuiting</code></strong> โดยหากประมวลผลแล้วเกิดข้อผิดพลาด ระบบจะทำการคืนค่าข้อผิดพลาดกลับไปยังฟังก์ชันผู้เรียกทันทีโดยอัตโนมัติ</li>

<li><strong>คำสั่ง <code>panic!</code></strong> จะสั่งหยุดหน่วยการทำงานปัจจุบันทันทีเมื่อเกิดข้อผิดพลาดร้ายแรง โดยระบบจะถอยย้อน <code>Stack</code> เพื่อเคลียร์ตัวแปรและคืนทรัพยากร ก่อนปิดการทำงานลง</li>
</ul>
</td>

<td valign="top">
<ul>
<li><strong>เมื่อคำสั่ง <code>raise</code> ทำงาน</strong> จะหยุดการประมวลผลบรรทัดปัจจุบันทันที และถอยย้อน <code>Stack</code> เพื่อค้นหาบล็อก <code>except</code> ที่จับคู่กับ <code>Exception</code> ชนิดนั้นๆ ทำให้ทิศทางการทำงานกระโดดข้ามไปมาและอ่านทำความเข้าใจได้ยากจากหน้าไวยากรณ์</li>

<li><strong>สนับสนุนการเขียนโปรแกรมตามลำดับลอจิกปกติไปก่อน</strong> แล้วค่อยใช้บล็อก <code>try-except</code> ดักจับข้อผิดพลาดที่เกิดขึ้นขณะทำงาน (<code>Runtime</code>)</li>
</ul>
</td>
</tr>

<tr>
<td valign="top"><strong>Type System</strong></td>

<td valign="top">
<ul>
<li><strong>การแยกชนิดข้อมูล <code>T</code> และ <code>Option&lt;T&gt;</code></strong> โดยเป็นการแยก <code>T</code> (ชนิดข้อมูลที่มีค่าเสมอ) และ <code>Option&lt;T&gt;</code> (ชนิดข้อมูลที่อาจเป็นค่าว่าง) ออกจากกันอย่างเด็ดขาด ช่วยตัดปัญหา <strong>Null Pointer Exception</strong> ออกไปได้ตั้งแต่ขั้นตอนคอมไพล์</li>

<li><strong>ตัวแปรจะอยู่ในสถานะใดสถานะหนึ่งได้เพียงอย่างเดียวในขณะนั้น</strong> เช่น <code>Option</code> (<code>Some</code> หรือ <code>None</code>) และ <code>Result</code> (<code>Ok</code> หรือ <code>Err</code>)</li>

<li><strong>มีแอตทริบิวต์ <code>#[must_use]</code></strong> บังคับให้ผู้เขียนโปรแกรมต้องนำผลลัพธ์ไปจัดการต่อเสมอ หากละเลยคอมไพเลอร์จะแจ้งเตือนทันที</li>
</ul>
</td>

<td valign="top">
<ul>
<li><strong>ตรวจสอบชนิดข้อผิดพลาดแบบไดนามิก</strong> โดยจะทำการประเมินและจับคู่ประเภทของข้อผิดพลาดขณะรันโปรแกรม (<code>Runtime</code>) เท่านั้น โดยไม่มีกลไกการบังคับตรวจสอบในขั้นตอนคอมไพล์ (<code>Compile-Time</code>)</li>

<li><strong>เป็นลำดับชั้นคลาสสืบทอด</strong> โดยข้อผิดพลาดทั้งหมดใน Python เป็นออบเจกต์ที่สืบทอดมาจากคลาสสูงสุดคือ <code>BaseException</code> โดยมีคลาสย่อยหลักคือ <code>Exception</code> เป็นคลาสสำหรับจัดการข้อผิดพลาดทั่วไปของโปรแกรม</li>

<li><strong>การประยุกต์ใช้ Subtype Polymorphism</strong> โดยโครงสร้างการดักจับข้อผิดพลาด สามารถรับและจัดการ <code>Exception</code> คลาสลูกทุกประเภทที่สืบทอดมาจากคลาส <code>Exception</code> ได้โดยอัตโนมัติ</li>
</ul>
</td>
</tr>

<tr>
<td valign="top"><strong>Memory Management</strong></td>

<td valign="top">
<ul>
<li><strong>คืนทรัพยากรอัตโนมัติด้วยระบบ <code>Ownership</code></strong> ทันทีที่ <code>Option</code> หรือ <code>Result</code> หมดขอบเขตการทำงาน (<code>Scope</code>)</li>

<li><strong>มี <code>Null Pointer Optimization (NPO)</code></strong></li>

<li><strong>ไม่มีการสร้าง <code>Overhead</code> บน <code>Heap</code></strong> เพราะโครงสร้าง <code>Enum</code> มีพฤติกรรมเป็น <code>Value Type</code> ซึ่งข้อมูลจะถูกประมวลผลและส่งผ่านบน <code>Stack</code></li>
</ul>
</td>

<td valign="top">
<ul>
<li><strong>สร้าง Overhead บน Heap สูง</strong> โดยคำสั่ง <code>raise</code> จะจองพื้นที่บน <code>Heap Memory</code> และสร้างออบเจกต์ <code>Traceback</code> เพื่อเก็บบันทึก <code>Stack Frame</code> ทำให้เกิด <code>Runtime Overhead</code> สูง</li>

<li><strong>คืนหน่วยความจำอัตโนมัติ</strong> โดยใน Python 3 จะทำการลบตัวแปรอ้างอิงข้อผิดพลาด (เทียบเท่าการสั่ง <code>del e</code> เบื้องหลัง) ให้อัตโนมัติทันทีที่ประมวลผลหลุดออกจากบล็อก <code>except</code></li>
</ul>
</td>
</tr>

<tr>
<td valign="top"><strong>Safety</strong></td>

<td valign="top">
<ul>
<li><strong>การันตีความปลอดภัยตั้งแต่ขั้นตอนคอมไพล์</strong> โดยคอมไพเลอร์จะบังคับให้โปรแกรมเมอร์ต้องเขียนโค้ดรองรับทุกกรณีของข้อผิดพลาดที่อาจเกิดขึ้นขณะรันโปรแกรม (<code>Runtime</code>) ไว้ล่วงหน้าเสมอ ทำให้โปรแกรมไม่ Crash โดยที่ไม่ได้คาดคิด</li>

<li><strong>การันตีการตรวจสอบครบทุกกรณี</strong> โดยคอมไพเลอร์บังคับให้ต้องเขียนโค้ดรองรับทุก <code>Variant</code> ของ <code>Enum</code> (<code>Ok/Err</code> หรือ <code>Some/None</code>) ให้ครบถ้วน เพื่อป้องกันไม่ให้มีเงื่อนไขตกหล่น</li>

<li><strong>การันตีว่า Error จะไม่ถูกละเลย</strong> โดยคอมไพเลอร์บังคับให้ต้องจัดการค่า <code>Result</code> ที่คืนกลับมาเสมอ ไม่สามารถเรียกฟังก์ชันแล้วปล่อยผ่านไปเฉย ๆ ได้</li>

<li><strong>ป้องกันความผิดพลาดของระบบด้วย <code>panic!</code></strong> โดยจะสั่งหยุดโปรแกรมทันทีเมื่อเกิด Bug ร้ายแรง เพื่อการันตีว่าระบบจะไม่ฝืนทำงานต่อด้วยข้อมูลที่ผิดพลาด</li>
</ul>
</td>

<td valign="top">
<ul>
<li><strong>มีความเสี่ยงที่โปรแกรมจะเกิดการทำงานผิดพลาดและล่มลงในขณะที่ทำงานอยู่</strong> ถ้าไม่ได้เขียนบล็อก <code>except</code> ดักจับไว้ให้ครอบคลุม <code>Error</code> นั้น</li>
</ul>
</td>
</tr>

</tbody>
</table>


<h3>ตารางที่ 3: เปรียบเทียบ Rust และ Swift</h3>

<table>
<thead>
<tr>
<th>Aspect</th>
<th>Rust</th>
<th>Swift</th>
</tr>
</thead>

<tbody>

<tr>
<td valign="top"><strong>Syntax</strong></td>

<td valign="top">
<ul>
<li><strong>จัดการข้อผิดพลาดด้วย <code>Option&lt;T&gt;</code> และ <code>Result&lt;T, E&gt;</code></strong> โดย <code>Option&lt;T&gt;</code> ใช้จัดการกรณีที่ไม่มีข้อมูล (ค่าว่าง) ส่วน <code>Result&lt;T, E&gt;</code> ใช้จัดการกรณีที่เกิดข้อผิดพลาดซึ่งสามารถแก้ไขได้</li>

<li><strong>กำหนดโครงสร้างด้วย <code>Enum</code> และ รูปแบบตัวเลือกย่อย (<code>Variant</code>)</strong> ได้แก่ <code>Some/None</code> สำหรับ <code>Option</code> และ <code>Ok/Err</code> สำหรับ <code>Result</code></li>

<li><strong>แกะค่าข้อมูลด้วย <code>Pattern Matching</code></strong> โดยใช้ไวยากรณ์ <code>match</code> และ <code>if let</code></li>

<li><strong>ส่งต่อข้อผิดพลาดอย่างรวดเร็วด้วย <code>? Operator</code></strong></li>

<li><strong>เมธอดดึงค่าข้อมูล <code>unwrap()</code> และ <code>expect()</code></strong> ใช้แกะเอาค่าข้างใน <code>Option</code> หรือ <code>Result</code> ออกมาใช้งาน หากเจอข้อผิดพลาด (<code>None/Err</code>) จะสั่งหยุดโปรแกรมทันที (<code>panic!</code>) โดย <code>expect()</code> สามารถใส่ข้อความอธิบายสาเหตุเพิ่มเติมได้</li>

<li><strong>คำสั่ง <code>panic!</code></strong> เป็นการสั่งหยุดโปรแกรมทันที ใช้เมื่อเจอข้อผิดพลาดร้ายแรงที่ไม่สามารถแก้ไขหรือประมวลผลต่อได้</li>

<td valign="top">
<ul>
<li>
<strong>ใช้ <code>throws</code> สำหรับประกาศฟังก์ชันที่อาจโยนข้อผิดพลาด และใช้โครงสร้างบล็อก <code>do-catch</code> ในการดักจับและจัดการข้อผิดพลาด</strong>
</li>

<li>
<strong>บังคับใส่คีย์เวิร์ดนำหน้าฟังก์ชันที่ประกาศ <code>throws</code> เสมอ</strong> เพื่อเน้นจุดที่อาจจะเกิด Error ให้เห็นชัดเจนในโค้ด โดยแบ่งออกเป็น 3 รูปแบบ ได้แก่
<ul>
<li><code>try:</code> ใช้เรียกฟังก์ชันตามปกติภายในบล็อก <code>do-catch</code> (หรือส่งต่อ Error ไปยังฟังก์ชันอื่น)</li>
<li><code>try?:</code> ใช้เปลี่ยนข้อผิดพลาดให้กลายเป็นค่า <code>nil</code> (แปลงผลลัพธ์เป็น <code>Optional&lt;T&gt;</code>)</li>
<li><code>try!:</code> เป็นการบังคับแกะค่าผลลัพธ์ออกมาทันทีเมื่อมั่นใจว่าไม่มีทางเกิด Error ขึ้นแน่นอน</li>
</ul>
</li>

<li>
<strong>ไวยากรณ์สั้นสำหรับจัดการค่าว่าง (<code>nil</code>)</strong> โดยไม่ต้องใช้โครงสร้าง Error Handling แบบเต็มรูปแบบ ได้แก่ <code>if let</code> / <code>guard let</code>, <code>?.</code> และ <code>??</code>
</li>
</ul>
</td>
</tr>

<tr>
<td valign="top"><strong>Semantics / Behavior</strong></td>

<td valign="top">
<ul>
<li><strong>มอง <code>Error</code> เป็นค่าข้อมูลปกติ</strong> ที่ถูกส่งคืนจากฟังก์ชันเหมือนการคืนค่าทั่วไปผ่านโครงสร้าง <code>Result&lt;T, E&gt;</code></li>

<li><strong><code>?</code> Operator ทำงานแบบ <code>Short-circuiting</code></strong> โดยหากประมวลผลแล้วเกิดข้อผิดพลาด ระบบจะทำการคืนค่าข้อผิดพลาดกลับไปยังฟังก์ชันผู้เรียกทันทีโดยอัตโนมัติ</li>

<li><strong>คำสั่ง <code>panic!</code></strong> จะสั่งหยุดหน่วยการทำงานปัจจุบันทันทีเมื่อเกิดข้อผิดพลาดร้ายแรง โดยระบบจะถอยย้อน <code>Stack</code> เพื่อเคลียร์ตัวแปรและคืนทรัพยากร ก่อนปิดการทำงานลง</li>
</ul>
</td>

<td valign="top">
<ul>
<li><strong>ส่งคืนค่าข้อผิดพลาดในรูปแบบ <code>Value Passing</code></strong> คล้ายกลไกการคืนค่า <code>Result</code> ในภาษา Rust โดยไม่ต้องผ่านกระบวนการถอยย้อน <code>Stack</code></li>

<li><strong>เกิด Short-circuit เมื่อมีข้อผิดพลาด</strong> โดยเมื่อคำสั่ง <code>try</code> เกิด Error ระบบจะตัดลำดับการทำงานแล้วกระโดดไปยังบล็อก <code>catch</code> ที่ดักรับอยู่ใกล้ที่สุด</li>

<li><strong>แกะค่าด้วย Pattern Matching ในบล็อก <code>catch</code></strong> ทำให้ดักจับประเภทข้อผิดพลาด (ที่มักนิยามด้วย <code>Enum</code>) และแกะค่าข้อมูลภายในมาใช้งานได้อย่างแม่นยำและครบทุกกรณี</li>
</ul>
</td>
</tr>

<tr>
<td valign="top"><strong>Type System</strong></td>

<td valign="top">
<ul>
<li><strong>การแยกชนิดข้อมูล <code>T</code> และ <code>Option&lt;T&gt;</code></strong> โดยเป็นการแยก <code>T</code> (ชนิดข้อมูลที่มีค่าเสมอ) และ <code>Option&lt;T&gt;</code> (ชนิดข้อมูลที่อาจเป็นค่าว่าง) ออกจากกันอย่างเด็ดขาด ช่วยตัดปัญหา <strong>Null Pointer Exception</strong> ออกไปได้ตั้งแต่ขั้นตอนคอมไพล์</li>

<li><strong>ตัวแปรจะอยู่ในสถานะใดสถานะหนึ่งได้เพียงอย่างเดียวในขณะนั้น</strong> เช่น <code>Option</code> (<code>Some</code> หรือ <code>None</code>) และ <code>Result</code> (<code>Ok</code> หรือ <code>Err</code>)</li>

<li><strong>มีแอตทริบิวต์ <code>#[must_use]</code></strong> บังคับให้ผู้เขียนโปรแกรมต้องนำผลลัพธ์ไปจัดการต่อเสมอ หากละเลยคอมไพเลอร์จะแจ้งเตือนทันที</li>
</ul>
</td>

<td valign="top">
<ul>
<li><strong>นิยามข้อผิดพลาดผ่าน <code>Enum</code> ที่นำโปรโตคอล <code>Error</code> มาใช้งาน</strong> และรองรับการใส่ <code>Associated Values</code> เพื่อระบุรายละเอียดหรือสาเหตุการเกิดข้อผิดพลาดขณะรันไทม์</li>

<li><strong>มีกลไก Type Transformation</strong> โดยคีย์เวิร์ด <code>try?</code> ทำหน้าที่แปลงประเภทข้อมูลผลลัพธ์จาก <code>T</code> (ชนิดข้อมูลของผลลัพธ์ปกติเมื่อประมวลผลสำเร็จ) ไปเป็น <code>Optional&lt;T&gt;</code> (ชนิดข้อมูลห่อหุ้มที่อาจมีค่าเป็น <code>T</code> หรือเป็นค่าว่าง <code>nil</code>) เพื่อลดรูปสถานะข้อผิดพลาดให้กลายเป็นค่าว่าง (<code>nil</code>) โดยไม่ต้องใช้โครงสร้าง <code>do-catch</code> แบบเต็มรูปแบบ</li>
</ul>
</td>
</tr>

<tr>
<td valign="top"><strong>Memory Management</strong></td>

<td valign="top">
<ul>
<li><strong>คืนทรัพยากรอัตโนมัติด้วยระบบ <code>Ownership</code></strong> ทันทีที่ <code>Option</code> หรือ <code>Result</code> หมดขอบเขตการทำงาน (<code>Scope</code>)</li>

<li><strong>มี <code>Null Pointer Optimization (NPO)</code></strong></li>

<li><strong>ไม่มีการสร้าง <code>Overhead</code> บน <code>Heap</code></strong> เพราะโครงสร้าง <code>Enum</code> มีพฤติกรรมเป็น <code>Value Type</code> ซึ่งข้อมูลจะถูกประมวลผลและส่งผ่านบน <code>Stack</code></li>
</ul>
</td>

<td valign="top">
<ul>
<li><strong>ประมวลผลบน <code>Stack</code> ไม่เสีย Overhead บน <code>Heap</code></strong> โดยใช้กลไกการรับส่งค่าในรูปแบบ <code>Value Type</code> บน <code>Stack</code> เป็นหลัก จึงไม่ต้องเสีย <code>Runtime Overhead</code> ในการสร้าง <code>Exception Object</code> ขนาดใหญ่บน <code>Heap Memory</code></li>

<li><strong>เมื่อคำสั่ง <code>throw</code> ทำงาน</strong> ระบบจะคืนพื้นที่ตัวแปร Local บน <code>Stack</code> และทำลาย <code>Reference Objects</code> ใน <code>Scope</code> นั้นทันที ก่อนกระโดดไปยังบล็อก <code>catch</code></li>
</ul>
</td>
</tr>

<tr>
<td valign="top"><strong>Safety</strong></td>

<td valign="top">
<ul>
<li><strong>การันตีความปลอดภัยตั้งแต่ขั้นตอนคอมไพล์</strong> โดยคอมไพเลอร์จะบังคับให้โปรแกรมเมอร์ต้องเขียนโค้ดรองรับทุกกรณีของข้อผิดพลาดที่อาจเกิดขึ้นขณะรันโปรแกรม (<code>Runtime</code>) ไว้ล่วงหน้าเสมอ ทำให้โปรแกรมไม่ Crash โดยที่ไม่ได้คาดคิด</li>

<li><strong>การันตีการตรวจสอบครบทุกกรณี</strong> โดยคอมไพเลอร์บังคับให้ต้องเขียนโค้ดรองรับทุก <code>Variant</code> ของ <code>Enum</code> (<code>Ok/Err</code> หรือ <code>Some/None</code>) ให้ครบถ้วน เพื่อป้องกันไม่ให้มีเงื่อนไขตกหล่น</li>

<li><strong>การันตีว่า Error จะไม่ถูกละเลย</strong> โดยคอมไพเลอร์บังคับให้ต้องจัดการค่า <code>Result</code> ที่คืนกลับมาเสมอ ไม่สามารถเรียกฟังก์ชันแล้วปล่อยผ่านไปเฉย ๆ ได้</li>

<li><strong>ป้องกันความผิดพลาดของระบบด้วย <code>panic!</code></strong> โดยจะสั่งหยุดโปรแกรมทันทีเมื่อเกิด Bug ร้ายแรง เพื่อการันตีว่าระบบจะไม่ฝืนทำงานต่อด้วยข้อมูลที่ผิดพลาด</li>
</ul>
</td>

<td valign="top">
<ul>
<li><strong>คอมไพเลอร์จะไม่สามารถคอมไพล์โปรแกรมได้</strong> ถ้ามีการเรียกใช้ฟังก์ชันที่มี <code>throws</code> โดยที่ไม่ใส่คำสั่ง <code>try</code> หรือไม่มีบล็อก <code>do-catch</code> ที่คอยจัดการข้อผิดพลาดอย่างถูกต้อง</li>

<li><strong>บังคับให้ดักจับข้อผิดพลาดให้ครบทุกกรณี</strong> โดยคอมไพเลอร์จะบังคับให้ดักจับ Error ในบล็อก <code>catch</code> ให้ครบทุกกรณี ถ้าระบุเคสไม่ครบ จะต้องเพิ่มบล็อก <code>catch</code> ทั่วไปเป็นค่าเริ่มต้น (<code>Fallback</code>) เพื่อรองรับข้อผิดพลาดที่เหลือทั้งหมด</li>

<li><strong>ป้องกันปัญหา Null Pointer Dereference ตั้งแต่ตอนคอมไพล์</strong> โดยบังคับให้ต้องทำการแกะค่าจาก <code>Optional&lt;T&gt;</code> ก่อนนำไปใช้งานเสมอ</li>
</ul>
</td>
</tr>

</tbody>
</table>




<p><strong>หมายเหตุ:</strong> แบ่งการเปรียบเทียบออกเป็น 3 ตาราง เพื่อให้เนื้อหาไม่แน่นเกินไปและอ่านง่ายขึ้น โดยเปรียบเทียบ Rust กับ Java, Python และ Swift ตามลำดับ</p>

---

### ตัวอย่างโค้ดการจัดการข้อผิดพลาดจากการแปลงข้อมูลและการตรวจสอบผลลัพธ์ในแต่ละภาษา

ตัวอย่างนี้เป็นการ**แปลงข้อมูลจากข้อความ (String) ให้เป็นข้อมูลชนิดจำนวนเต็ม (Integer/Int)** ถ้าสามารถแปลงเป็นตัวเลขได้จะนำค่าที่แปลงได้ไปคูณ 2 แล้วแสดงผลลัพธ์ที่คำนวณได้ แต่ถ้าแปลงไม่ได้ เช่น ข้อมูลเป็น `"abc"` จะเกิดข้อผิดพลาดขึ้น ซึ่งแต่ละภาษาจะมีวิธีจัดการข้อผิดพลาดที่แตกต่างกันไปตามรูปแบบของภาษา

### Rust Example

```rust
use std::num::ParseIntError;

fn parse_and_double(text: &str) -> Result<i32, ParseIntError> {
    let number = text.parse::<i32>()?; 
    Ok(number * 2)
}

fn main() {
    let input = "abc"; 

    match parse_and_double(input) {
        Ok(val) => println!("Success: {}", val),
        Err(err) => println!("Parsing Failure: {}", err),
    }
}
```
### Java Example

```java
public class ErrorHandlingJava {
    public static void main(String[] args) {
        String input = "abc";
        try {
            int number = Integer.parseInt(input);
            number = number * 2;
            System.out.println("Success: " + number);
        } catch (NumberFormatException e) {
            System.out.println("Parsing Failure -> " + e);
        }
    }
}

```

### Python Example

```python
def parse_and_double(text: str) -> int:
    number = int(text)
    return number * 2

input = "abc"
try:
    result = parse_and_double(input)
    print(f"Success: {result}")
except ValueError as err:
    print(f"Parsing Failure: {err}")
```

### Swift Example

```swift
import Foundation

enum ParseError: Error {
    case invalidDigit
}

func parseAndDouble(_ text: String) throws -> Int {
    guard let number = Int(text) else {
        throw ParseError.invalidDigit
    }
    return number * 2
}

let input = "abc" 

do {
    let result = try parseAndDouble(input)
    print("Success: \(result)")
} catch ParseError.invalidDigit {
    print("Parsing Failure: Invalid digit found")
} catch {
    print("Parsing Failure: \(error)")
}
```

### Analysis

วิเคราะห์ความแตกต่างที่สำคัญ และเหตุผลด้านการออกแบบภาษาของภาษา Rust, Java, Python และ Swift
- **ด้านไวยากรณ์และรูปแบบโค้ด:** Java และ Python เลือกแยกโค้ดปกติกับโค้ดจัดการข้อผิดพลาดออกจากกันอย่างชัดเจนด้วย `try-catch/except` เพื่อเน้นให้อ่านง่าย ส่วน Rust มองข้อผิดพลาดเป็นค่าข้อมูลธรรมดาที่ส่งกลับมาจากฟังก์ชัน และมี `?` ช่วยย่อโค้ดให้สั้นลง ส่วน Swift เน้นความยืดหยุ่น โดยรวมทั้งบล็อก `do-catch` และใช้ `try?` เพื่อแปลงข้อผิดพลาดเป็นค่าว่าง (`nil`) ได้ทันที

- **ด้านลำดับการทำงาน:** Java และ Python จะใช้วิธีกระโดดย้อน `Stack` เพื่อค้นหาจุดดักจับข้อผิดพลาด ทำให้ทิศทางของโปรแกรมกระโดดข้ามไปมาและคาดเดาได้ยากกว่า ส่วน Rust และ Swift เน้นส่งข้อผิดพลาดกลับมาตามลำดับการทำงานปกติ ทำให้โปรแกรมทำงานอย่างตรงไปตรงมา คาดเดาได้ง่าย

- **ด้านความปลอดภัยและระบบชนิดข้อมูล:** Python จะเน้นเขียนไวและยืดหยุ่น จึงรอตรวจชนิดข้อผิดพลาดตอนโปรแกรมทำงานเท่านั้น ทำให้เสี่ยงโปรแกรมล่มสูง Java จะเพิ่มความปลอดภัยด้วยการบังคับเช็กบางส่วนตอนคอมไพล์ ส่วน Rust และ Swift เน้นความปลอดภัยสูงสุด โดยบังคับให้ผู้เขียนต้องรับมือกับทุกเคสข้อผิดพลาดตั้งแต่ตอนเขียนโค้ด เพื่อป้องกันไม่ให้โปรแกรมล่มหรือเกิดปัญหาเรื่องหน่วยความจำ

- **ด้านประสิทธิภาพและหน่วยความจำ:** Java และ Python เน้นความสะดวก จึงยอมเสียความเร็วบางส่วนจากการสร้าง `Exception Object` เก็บลง `Heap Memory` ส่วน Rust และ Swift เน้นความเร็วสูงสุด โดยส่งข้อผิดพลาดผ่าน `Stack` โดยตรง ทำให้ไม่กินทรัพยากรเพิ่ม
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

- อธิบายแนวคิดพื้นฐานของ Error Handling ใน Rust
  
- อธิบายแนวชนิดข้อมูล Result และ Option

- อธิบายแนวคิดพื้นฐานของ match, error propagation 


**Member 2**

- อธิบายการทำงานของ Code ที่นำมายกตัวอย่าง
- อธิบายผลลัพธ์ที่เกิดขึ้น

**Member 3**

- วิเคราะห์เกี่ยวกับ Error Handling ในภาษา Rust ในมุมมองของ Programming Languages ในแต่ละหัวข้อ ได้แก่ Syntax, Semantics, Type System, Memory / Resource Management, Abstraction / Other PPL Concepts และ Why Rust?

- เปรียบเทียบภาษา Rust กับ Java, Python และ Swift

- ยกตัวอย่างโค้ดของทั้ง 4 ภาษาที่นำมาเปรียบเทียบกัน

- วิเคราะห์ความแตกต่างที่สำคัญ และเหตุผลด้านการออกแบบภาษาของภาษา Rust, Java, Python และ Swift

**Member 4**

- ยกตัวอย่างโค้ดของ Common Mistakes

- ออกแบบโจทย์ที่เกี่ยวกับหัวข้อของกลุ่ม
 
> สมาชิกทุกคนต้องสามารถอธิบาย Code ของกลุ่มได้ ไม่ใช่เฉพาะส่วนที่ตนเองเขียน

---

## 12. References

> แนะนำให้มีอย่างน้อย **4 แหล่งอ้างอิง** และควรใช้เอกสารทางการเป็นหลัก

1. `Rust by Example: Error Handling — https://doc.rust-lang.org/rust-by-example/error.html`
2. `Rust Reference: Type Layout — https://doc.rust-lang.org/reference/type-layout.html`
3. `Rust Standard Library: Option — https://doc.rust-lang.org/std/option/`
4. `Rust Standard Library: Result — https://doc.rust-lang.org/std/result/`
5. `Rust Standard Library: #[must_use] Attribute — https://doc.rust-lang.org/std/attribute.must_use.html`
6. `The Rust Programming Language: Recoverable Errors with Result — https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html`
7. `The Rust Programming Language — https://doc.rust-lang.org/book/title-page.html`
8. `Rust by Example — https://doc.rust-lang.org/rust-by-example/index.html`
9. `Python Documentation: Errors and Exceptions — https://docs.python.org/3/tutorial/errors.html`
10. `Python Documentation: Exceptions — https://docs.python.org/3/reference/executionmodel.html#exceptions`
11. `Python Documentation: Traceback Objects — https://docs.python.org/3/reference/datamodel.html#traceback-objects`
12. `Python Documentation: The try Statement — https://docs.python.org/3/reference/compound_stmts.html#the-try-statement`
13. `The Swift Programming Language: Error Handling — https://docs.swift.org/latest/documentation/the-swift-programming-language/errorhandling/`
14. `Apple Developer Documentation: Error — https://developer.apple.com/documentation/swift/error`
15. `Oracle Java Tutorials: Exceptions — https://docs.oracle.com/javase/tutorial/essential/exceptions/index.html`
16. `Java Language Specification: Chapter 11 — Exceptions — https://docs.oracle.com/javase/specs/jls/se21/html/jls-11.html`
17. `Rust Error Handling in 2026: Result, Option, and the ? Operator — https://rustify.rs/articles/rust-error-handling-result-option`

---

## 13. AI Usage Declaration

สามารถใช้ AI เป็นเครื่องมือช่วยเรียนรู้และพัฒนาได้ แต่สมาชิกทุกคนต้องเข้าใจและสามารถอธิบายผลงานของกลุ่มได้

| AI Tool | Purpose | How the Result Was Verified |
|---|---|---|
| `Chatgpt` | • ใช้ช่วยเขียนโค้ดภาษา Python และ Swift<br>• ใช้ช่วยเขียน Format ในไฟล์ .md <br> •ใช้ในการหาข้อมูล| • ตรวจสอบโค้ดโดยเปรียบเทียบกับเว็บของภาษานั้น ๆ ว่าไวยากรณ์และโครงสร้างเหมือนกันหรือไม่ และตรวจสอบกับโปรแกรม VS Code อีกครั้ง<br>• ตรวจสอบ Format ในไฟล์ .md โดยดูรูปที่ได้ว่าตรงตามที่ต้องการหรือไม่ <br> • ตรวจสอบจากการนำข้อมูลที่ได้ไปเทียบกับข้อมูลจากเว็บที่น่าเชื่อถือว่ามีความถูกต้องตรงกันไหม|
| `Gemini` | • ใช้แปลข้อมูลในเว็บที่เป็นภาษาอังกฤษและใช้ช่วยค้นหาเว็บที่มีเนื้อหาเกี่ยวกับหัวข้อนั้น ๆ ในการวิเคราะห์เชิง PPL<br> |  • ตรวจสอบโดยเปรียบเทียบข้อมูลกับเว็บหลาย ๆ แหล่งและกับที่เป็นภาษาไทยว่าเนื้อหาถูกต้องตรงกันหรือไม่ <br> |
| `claude` | • ใช้ช่วยสร้างโค้ดตัวอย่างเพื่อสาธิต Common mistakes และ ช่วยออกแบบโค้ดสำหรับ Exercises <br> | • ตรวจสอบโดยการนำโค้ดไปเทียบกับ syntax ของ official documents และทดสอบการรันโค้ดใน Online compiler ว่าสามารถรันได้และถูกต้อง<br> |

### Declaration

- [x] Code ทุกส่วนที่นำเสนอได้รับการ Compile และทดสอบแล้ว
- [x] สมาชิกทุกคนสามารถอธิบาย Code ที่นำเสนอได้
- [x] ตรวจสอบข้อมูลจากแหล่งอ้างอิงที่น่าเชื่อถือแล้ว
- [x] ระบุการใช้ AI อย่างโปร่งใส

**รายละเอียดการใช้ AI**

 - ใช้ AI ในขั้นตอนการหาข้อมูลเกี่ยวกับหัวข้อ Error handing โดยตรวจสอบความถูกต้องจากการเปรียบเทียบข้อมูลที่ได้จากหลาย ๆ เว็บที่เกี่ยวข้อง <br>
 - ใช้ Chatgpt ในขั้นตอนสร้างโค้ด Python, Swift และใช้ Claude ในขั้นตอนการสร้าโค้ดตัวอย่าง Common Mistakes และออกแบบ Exercise โดยตรวจสอบความถูกต้องจากการนำโค้ดไปเปรียบเทียกับ Syntax ของภาษานั้น ๆ และนำไป compile ตรวจสอบว่าทำงานได้หรือไม่ <br>
 - ใช้ในขั้นตอนการตรวจสอบความถูกต้องของข้อมูลและcodeที่นำมาใช้ โดยตรวจสอบความถูกต้องผ่านการนำโค้ดไป compile และเช็คข้อมูลจากเว็บที่น่าเชื่ออีกครั้ง <br>
 - ใช้ในขั้นตอนปรับปรุงเนื้อหางานเขียน โดยตรวจสอบจากการอ่านประโยคที่ถูกแก้ไขว่าความหมายโดยรวมเปลี่ยนไหม <br>
 - ใช้ Chatgpt ในขั้นตอนเขียน Format ลงไฟล์ .md โดยตรวจสอบไฟล์หลังเขียนเสร็จว่าไฟล์มี Format ตรงตามที่ต้องการหรือไม่ <br>

---

## 14. GitHub Contribution

| Member | Issues | Commits | Pull Requests | Code Reviews | Contribution |
|---|---:|---:|---:|---:|---|
| Member 1 | `0` | `30` | `5` | `1` | `Introduction, Key Concepts` |
| Member 2 | `0` | `36` | `2` | `1` | `Important Syntax/Rules, Runnable Code Examples` |
| Member 3 | `0` | `32` | `2` | `1` | `PPL Perspective, Rust vs. Other Language` |
| Member 4 | `0` | `14` | `1` | `1` | `Common Mistakes, Exercises` |

### Teamwork Reflection

**How did your team collaborate?**

- มีการพูดคุยร่วมกันเกี่ยวกับขอบเขตของหัวข้อที่ได้รับมา และแบ่งหน้าที่ความรับผิดชอบของสมาชิกแต่ละคน

**Problems encountered**

- ในช่วงเริ่มแรก สมาชิกในกลุ่มยังไม่ค่อยคุ้นชินวิธีการใช้งาน GitHub และการทำงานร่วมกันผ่าน Git

**How did you solve them?**

- ศึกษาวิธีการใช้งาน GitHub และ Git เพิ่มเติมจากแหล่งข้อมูลเช่น เว็บไซต์, YouTube รวมถึงทดลองใช้งานจริงร่วมกันภายในทีม

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

**Repository:** `[https://github.com/670710620/rust-tutorial-2569/tree/main/14-error-handling-option-result]`

**Chapter Path:** `[14-error-handling-option-result/]`

**Final PR:** `#[38]`

**Submitted by:** `[Group 14]`

**Date:** `[2026-10-04]`

*โครงสร้างเอกสารฉบับเต็ม (Key Concepts, Runnable Code Examples, Common Mistakes, Exercises, PPL Perspective, Rust vs Other Language, References, AI Usage Declaration, GitHub Contribution, Final Checklist) ให้ทำต่อจากจุดนี้ตาม Template หลักของวิชา (`rust_tutorial_template.md`) ที่แนบมากับใบมอบหมายงาน*
