# Rust Tutorial Project — Principles of Programming Languages

> **กลุ่มที่:** 16
> **Topic No.:** 16
> **Topic Name:** Traits & Polymorphism
> **ประเด็นหลักที่ควรครอบคลุม:** trait, trait implementation, trait bounds, polymorphism

---

## 1. Members

| # | Name | Student ID | GitHub Username | Main Responsibility |
|---|---|---|---|---|
| 1 | นายนพรัตน์ นรนิล | 670710627 | `@[670710627]` | Concept + Short Code Illustration (สรุปแนวคิดหลัก + โค้ดตัวอย่างสั้น) |
| 2 | นายปธานิน พรรณหาญ | 670710628 | `@[กรอก GitHub username]` | Detailed Code + Live Demo (โค้ดเชิงลึก + สาธิตสด) |
| 3 | นางสาวปริยากร คาวิน | 670710629 | `@670710629` | Rust vs Other Language + PPL Analysis (เปรียบเทียบภาษา + วิเคราะห์เชิง PPL) |
| 4 | นางสาวพิชญธิดา รักดี | 670710630 | `@670710630` | Exercises + Common Mistakes + Challenge (แบบฝึกหัด + ข้อผิดพลาดที่พบบ่อย + คำถามท้าทาย) |

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

`Polymorphism เป็นแนวคิดที่ช่วยให้เราสามารถเขียนโค้ดชุดเดียวให้ทำงานกับหลาย Type ได้ โดยไม่ต้องเขียนโค้ดซ้ำสำหรับแต่ละ Type ทำให้โปรแกรมสามารถรองรับความหลากหลายและขยายระบบได้ง่ายขึ้น ในภาษา Rust ซึ่งไม่มี Class และ Inheritance แบบภาษาเชิงวัตถุทั่วไป Rust ใช้ Trait ในการสร้าง Polymorphism โดย Trait ช่วยกำหนดพฤติกรรมที่ Type ต่าง ๆ สามารถนำไปใช้ร่วมกันได้ ทำให้เราสามารถออกแบบโค้ดที่ยืดหยุ่นและนำกลับมาใช้ซ้ำได้ โดยไม่จำเป็นต้องพึ่งพา Inheritance`

---

## 4. Key Concepts

### 4.1 `[Polymorphism]`

**คำอธิบาย**

`การใช้ interface เดียวกันกับหลาย Type โดยแต่ละ Type สามารถมีพฤติกรรมแตกต่างกันได้ ทำให้ ผู้เรียกใช้(caller) ไม่ต้องรู้ว่าเป็น Type ไหน และ เพิ่ม Type ใหม่ได้ง่าย โดย Rust ใช้ Trait สำหรับทำ Polymorphism`

**ตัวอย่าง**

```rust
trait Animal {
    fn speak(&self);
}
struct Dog;
struct Cat;
impl Animal for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}
impl Animal for Cat {
    fn speak(&self) {
        println!("Meow!");
    }
}
fn make_sound<T: Animal>(animal: T) {
    animal.speak();
}
fn main() {
    let dog = Dog;
    let cat = Cat;

    make_sound(dog);
    make_sound(cat);
}
```

**Explanation**

`ใช้ Trait Animal เป็น interface ร่วม  แต่ละ Type implement พฤติกรรมของตัวเอง  T: Animal รับ Type ที่ implement Animal  Compiler เลือก implementation ตอน Compile time`

---

### 4.2 `[Traits]`

`Trait คือ ข้อกำหนดของพฤติกรรม โดยรวมกลุ่มของ method signature ที่เกี่ยวข้องกันไว้ด้วยกัน type ใดที่ implement trait นั้น ถือว่าได้ รับรอง ว่าจะมี method ตามที่ traitกำหนดไว้ครบถ้วน Method ใน trait แบ่งเป็น 2 แบบ: `<br>`1 required method (มีแค่ signature ไม่มี body บังคับให้ type ที่ implement ต้องเขียนเอง)`<br>`2 default method (มี implementation สำเร็จรูปให้แล้วจะใช้ตามเดิมหรือ override ก็ได้)`

**ตัวอย่าง**
```rust
trait Animal {
    fn name(&self) -> String;              

    fn greet(&self) -> String {             
       format!("Hi, I'm {}", self.name())
    }
}

struct Dog { name: String }

impl Animal for Dog {
    fn name(&self) -> String { self.name.clone() }
}

fn main() {
    let d = Dog { name: String::from("Rex") };
    println!("{}", d.greet());
}
```
**Explanation**

`Trait Animal มี 2 method`<br>
`name()  ไม่มี body ต้องให้ type ที่ implement เขียนเอง (required)`<br>
`greet()  มี body สำเร็จรูปให้แล้ว ใช้ได้เลยไม่ต้องเขียนใหม่ (default) และข้างในมันเรียก self.name() ได้เลย ทั้งที่ name() ยังไม่รู้ตอนนิยาม trait ว่าใครจะ implement ยังไง  เพราะมั่นใจได้ว่า type ไหนก็ตามที่implement มาต้องมี name() แน่นอน`
`Dog implement Animal โดยเขียนแค่ name() (ตัวเดียวที่บังคับ) ส่วน greet() ไม่เขียนเลย ใช้ default จาก trait`
`พอเรียก d.greet() → มันไปรัน default method → ข้างในเรียก self.name() → ได้ค่าจาก Dog ที่ implement ไว้ ("Rex") → ผลลัพธ์คือ "Hi, I'm Rex"`

---

### 4.3 `[Trait Implementation]`

`[คือขั้นตอนที่ type หนึ่ง เขียนโค้ดให้ตรงกับ contract ที่ trait กำหนดไว้ `<br>
`ใช้ syntax impl TraitName for TypeName { ... }]`


**ตัวอย่าง**
```rust
trait Speak {         
    fn speak(&self) -> String;
}

struct Dog;             

impl Speak for Dog {    
    fn speak(&self) -> String {
        String::from("Woof!")
    }
}
fn main() {
    let d = Dog ;
    println!("{}", d.speak());
}
```
**Explanation**

`impl Speak for Dog คือการบอก compiler ว่า "ตอนนี้ Dog เป็น type ที่ implement Speak แล้ว" ข้างในต้องเขียน method ที่เป็น required ให้ครบ (ในที่นี้คือ speak()) ถ้าเขียนไม่ครบ compiler จะ error ทันที เพราะถือว่า contract ยังไม่สมบูรณ์`

---

### 4.4 `[Trait Bounds]`

`การที่เรายังไม่กำหนดว่า parameterนั้น เป็น Type อะไร แต่เรากำหนดว่า Typeนัั้น ต้อง implement traitนี้`


**ตัวอย่าง**
```rust
trait Speak {
    fn speak(&self) -> String;
}

struct Dog;
impl Speak for Dog {
    fn speak(&self) -> String { String::from("Woof!") }
}

// T: Speak คือ trait bound
fn make_it_speak<T: Speak>(item: T) {
    println!("{}", item.speak());
}

fn main() {
    make_it_speak(Dog);
}
```
**Explanation**

`<T: Speak> บอกว่า T จะเป็น type อะไรก็ได้ แต่ต้อง implement Speak เท่านั้น เพราะข้างในฟังก์ชันมีการเรียก item.speak() — ถ้าไม่ใส่ bound ไว้ compiler จะไม่รู้ว่า T มี method speak() ไหม และจะ error ทันที`

---


### 4.5 `[Supertraits]`
`[Rust ไม่มี "inheritance" แต่ สามารถสร้าง trait ใหม่ ขึ้นมา แล้วกำหนดว่า Trait นี้ ต้องอาศัยความสามารถจากอีก Trait หนึ่ง]`

**ตัวอย่าง**
```rust
trait Named {
    fn name(&self) -> String;
}

trait Greet: Named {
    fn greet(&self) -> String {
        format!("Hi, {}", self.name())
    }
}

struct Person { name: String }

impl Named for Person {
    fn name(&self) -> String { self.name.clone() }
}

impl Greet for Person {}

fn main() {
    let p = Person { name: String::from("Alice") };
    println!("{}", p.greet());
}
```
**Explanation**

`trait Greet สามารถ "อ้างอิง" method จาก trait Named ได้ โดยการประกาศความสัมพันธ์ไว้ล่วงหน้าด้วย Greet: Named หมายความว่า Type ที่ implement Greet ต้อง implement Named ด้วย ทำให้ Greet สามารถเรียกใช้ method ที่กำหนดไว้ใน Named ได้ ในตัวอย่าง Person implement ทั้ง Named และ Greet จึงสามารถเรียก greet() ได้`

---

### 4.6 `[Static Dispatch]`
`[compiler รู้ concrete type ตั้งแต่ compile time จึงเลือก method ที่ต้องเรียกได้ล่วงหน้า โดยมักใช้ Generics หรือ impl Trait]`

**ตัวอย่าง**
```rust
trait Speak {
    fn speak(&self) -> String;
}

struct Dog;
impl Speak for Dog {
    fn speak(&self) -> String { String::from("Woof!") }
}

// T: Speak คือ trait bound
fn make_it_speak<T: Speak>(item: T) {
    println!("{}", item.speak());
}

fn main() {
    make_it_speak(Dog);
}
```
**Explanation**

`T: Speak ทำให้ compiler รู้ว่า T คือ Dog ตั้งแต่ compile time จึงสามารถเลือก method speak() ได้โดยตรง`

---

### 4.7 `[Dynamic Dispatch]`
`compiler ไม่ต้องรู้ concrete type ตอน compile time และเลือก implementation ที่เหมาะสมตอน runtime ผ่าน dyn Trait`

**ตัวอย่าง**
```rust
trait Speak {
    fn speak(&self);
}

struct Dog;

impl Speak for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}

fn make_sound(animal: &dyn Speak) {
    animal.speak();
}

fn main() {
    let dog = Dog;
    make_sound(&dog);
}
```
**Explanation**

`&dyn Speak สามารถรับ type ที่ implement Speak ได้ และการเรียก method จะเลือก implementation ผ่าน vtable ตอน runtime`

---

## 5. Important Syntax / Rules
|Syntax / Rule| Meaning | Example |
|---|---|---|
| `trait TraitName { ... }` | `ประกาศ trait` | `trait Speak { fn speak(&self) -> String;}` |
| `impl TraitName for TypeName { ... }` | `implement trait ให้กับ type ที่ระบุ` | `impl Speak for Dog { fn speak(&self) -> String { ... } }` |
| `fn f<T: Trait>(x: T)` | `trait bound บังคับว่า T ต้อง implement Trait นั้น` | `fn make_it_speak<T: Speak>(item: T) { ... }` |
| `trait A: B { ... }` | `supertrait - type ที่ implement A ต้อง implement B ` | `trait Greet: Named { ... }` |

### Important Rules

1. `Type ที่ implement Trait ต้อง implement required methods ให้ครบ`
2. `Trait bound ช่วยให้ compiler ตรวจสอบความสามารถของ Type ตอน compile time`
3. `Supertrait (A: B) กำหนดว่า Type ที่ implement A ต้อง implement B ด้วย`
4. `Static Dispatch รู้ concrete type ตอน Compile Time และ compiler เลือก implementation ล่วงหน้า`
5. `Dynamic Dispatch ใช้ dyn Trait และเลือก implementation ตอน Runtime`
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

### Mistake 1 — `ลืม Trait Bound ใน Generic Function `

**Problem**

`เมื่อใช้ Generic Function แล้วเรียก method ที่มาจาก Trait จะต้อง implement Trait นั้นทำให้ Rust ไม่สามารถรู้ได้ว่า method นั้นมีอยู่จริง`

**Incorrect Code**

```rust
trait Animal {
    fn make_sound(&self);
}

struct Dog;

impl Animal for Dog {
    fn make_sound(&self) {
        println!("Woof!");
    }
}

fn make_sound<T>(animal: T) {
    animal.make_sound(); // Error
}

fn main() {
    let dog = Dog;
    dog.make_sound();
}
```

**Correct Code**

```rust
trait Animal {
    fn make_sound(&self);
}

struct Dog;

impl Animal for Dog {
    fn make_sound(&self) {
        println!("Woof!");
    }
}

fn make_sound<T: Animal>(animal: T) {
    animal.make_sound();
}

fn main() {
    let dog = Dog;
    make_sound(dog);
}

```

**Why?**

`T สามารถเป็น Type อะไรก็ได้ ดังนั้น Rust ไม่สามารถรับประกันได้ว่า T จะมี make_sound() เลยจะต้องเพิ่ม T: Animal เพื่อให้ Rust รู้ว่า T ต้องเป็น Type ที่ implement Animal เมื่อมี Trait Bound แล้ว Rust จึงมั่นใจว่า animal สามารถเรียก make_sound() ได้ `

---

### Mistake 2 — `ลืม Implement Trait ให้กับ Type`

**Problem**

`ประกาศ Trait และสร้าง Type แล้ว แต่ไม่ได้ใช้ impl เพื่อบอกว่า Type นั้น implement Trait ทำให้ไม่สามารถเรียก method ของ Trait ผ่าน Type นั้นได้`

**Incorrect Code**

```rust
trait Animal {
    fn make_sound(&self);
}

struct Dog;

fn main() {
    let dog = Dog;
    dog.make_sound();//error
}
```

**Correct Code**

```rust
trait Animal {
    fn make_sound(&self);
}

struct Dog;

impl Animal for Dog {
    fn make_sound(&self) {
        println!("Woof!");
    }
}

fn main() {
    let dog = Dog;
    dog.make_sound();
}
```

**Why?**

`การประกาศ trait Animal เป็นการกำหนดว่า Animal นั้นต้องมี make_sound() แต่ไม่ได้หมายความว่า Dog จะมีความสามารถของ Animal ซึ่ง Dog ยังไม่ได้เป็น Animal จนกว่าเราจะเขียน Trait Implementation ที่กำหนดให้ Dog มีความสามารถตาม Animal และบอกวิธีทำงานของความสามารถนั้น`

---

## 8. Exercises

> จัดทำแบบฝึกหัด **2 ข้อ** ที่สอดคล้องกับ Topic และมีระดับความยากเหมาะสม

### Exercise 1 — `ระบบการชำระเงิน`

**Problem**

`ให้สร้าง trait Payment มี Method pay()สำหรับกำหนดพฤติกรรมการจ่ายเงิน โดยมี Struct 3 ประเภท คือ 

1.Cash โดยให้แสดงข้อความ "Pay with Cash"

2.QRCode แสดงข้อความ "Pay with QR Code" 

3.CreditCard แสดงข้อความ "Pay with Credit Card"" `

**Hint**

`ใช้ trait Payment เพื่อกำหนด Method pay() และใช้ impl กำหนด Payment สำหรับแต่ละประเภท`

**Solution**

```rust
trait Payment {
 fn pay(&self);
}
struct CreditCard;
struct Cash;
struct QRCode;

impl Payment for CreditCard {
 fn pay(&self) {
     println!("Pay with Credit Card");
    }
}

impl Payment for Cash {
    fn pay(&self) {
        println!("Pay with Cash");
    }
}

impl Payment for QRCode {
    fn pay(&self) {
        println!("Pay with QR Code");
    }
}


fn main() {
    let credit_card = CreditCard;
    let cash = Cash;
    let qr_code = QRCode;
    credit_card.pay();
    cash.pay();
    qr_code.pay();
}
```

**Explanation**

`Payment เป็น Trait ที่กำหนดว่า Type ที่ Implement Trait นี้จะต้องมี Method pay() แล้วก็กำหนด Struct 3 แบบ คือ CreditCard, Cash และ QRCode ทั้ง 3 Struct Implement Payment เหมือนกัน แต่กำหนดการทำงานของ pay() แตกต่างกันคือส่วนที่พิมพ์บอกด้านในว่าจ่ายกับอะไรดังนั้นเมื่อเรียกแต่ละตัวก็จะจะทำงานตาม Implementation ของตัวเอง`

---

### Exercise 2 — `ระบบจองคอร์ทแบดมินตัน`

**Problem**

`ให้สร้าง trait Booking ที่มี Method book() สำหรับกำหนดพฤติกรรมการจองสนาม จากนั้นสร้าง Struct 4 ประเภท ได้แก่

1.Student สำหรับนักศึกษาจองคอร์ท แสดงข้อความ "Student booked a badminton court"

2.Academic_staffสำหรับบุคลากรจองคอร์ท แสดงข้อความ "Academic staff booked a badminton court"

3.Guest สำหรับบุคคลภายนอกจองคอร์ท แสดงข้อความ "Guest booked a badminton court"

4.Athlete สำหรับนักกีฬาจองคอร์ท แสดงข้อความ "Athlete booked a badminton court"
`

**Hint**

`ใช้ trait Booking เพื่อกำหนด Method book() และใช้ impl กำหนด Booking สำหรับแต่ละประเภท`

**Solution**

```rust
trait Booking {
    fn book(&self);
}

struct Student;
struct Staff;
struct Guest;
struct Athlete;

impl Booking for Student {
    fn book(&self) {
        println!("Student booked a badminton court");
    }
}

impl Booking for Staff {
    fn book(&self) {
        println!("Staff booked a badminton court");
    }
}

impl Booking for Guest {
    fn book(&self) {
        println!("Guest booked a badminton court");
    }
}

impl Booking for Athlete {
    fn book(&self) {
        println!("Athlete booked a badminton court");
    }
}

fn main() {
    let student = Student;
    let staff = Staff;
    let guest = Guest;
    let athlete = Athlete;

    student.book();
    staff.book();
    guest.book();
    athlete.book();
}
```


**Explanation**

`Booking เป็น Trait ที่กำหนด Method book() สำหรับการจองคอร์ทจากนั้นสร้าง Struct 4 ประเภท ได้แก่ Student, Staff, Guest และ Athlete โดยแต่ละประเภท Implement Trait Booking และกำหนดการทำงานของ book() แตกต่างกัน`
### Challenge — `คำถามท้าทายผู้ฟัง`


`โค้ดด้านล่างนี้ compile ผ่านหรือไม่? ถ้าไม่ผ่าน ติดตรงไหน? `

```rust
trait Order {
    fn order(&self);
}

struct Student;
struct Teacher;
struct Staff;
struct Guest;

impl Order for Student {
    fn order(&self) {
        println!("Student ordered food");
    }
}

impl Order for Teacher {
    fn order(&self) {
        println!("Teacher ordered food");
    }
}

impl Order for Staff {
    fn order(&self) {
        println!("Staff ordered food");
    }
}

fn make_order<T: Order>(customer: T) {
    customer.order();
}

fn main() {
    let student = Student;
    let teacher = Teacher;
    let staff = Staff;
    let guest = Guest;

    make_order(student);
    make_order(teacher);
    make_order(staff);
    make_order(guest);
}
```

**Solution**

`compile ไม่ผ่าน เพราะ error[E0277]: the trait bound `Guest: Order` is not satisfied`

**Explanation**

`จาก Code จะเห็นว่า Student, Teacher และ Staff มีการ Implement Order แล้วแต่ Guest ยังไม่ได้ Implement Trait Order ขณะที่ Function กำหนด Trait Bound ว่า T ต้องเป็น Type ที่ Implement Order ดังนั้นเมื่อเรียก make_order(guest)  Rust จะตรวจสอบว่า Guest มี Order หรือไม่ แต่ไม่พบ จึงเกิด Compile Error`

**Correct Code**
```rust
trait Order {
    fn order(&self);
}

struct Student;
struct Teacher;
struct Staff;
struct Guest;

impl Order for Student {
    fn order(&self) {
        println!("Student ordered food");
    }
}

impl Order for Teacher {
    fn order(&self) {
        println!("Teacher ordered food");
    }
}

impl Order for Staff {
    fn order(&self) {
        println!("Staff ordered food");
    }
}

impl Order for Guest {
    fn order(&self) {
        println!("Guest ordered food");
    }
}

fn make_order<T: Order>(customer: T) {
    customer.order();
}

fn main() {
    let student = Student;
    let teacher = Teacher;
    let staff = Staff;
    let guest = Guest;

    make_order(student);
    make_order(teacher);
    make_order(staff);
    make_order(guest);
}
```

---

## 9. PPL Perspective

> **ส่วนนี้เป็นหัวใจของรายวิชา Principles of Programming Languages**

วิเคราะห์ Topic นี้ในมุมมองของ Programming Languages

### 9.1 Syntax

`ใช้ trait และ struct ,impl`

### 9.2 Semantics

`โดย trait จะกำหนดพฤติกรรมที่มีร่วมกันให้แต่ละ stuct ซึ่งแต่ละ struct สามารถกำหนดการทำงานของพฤติกรรมให้แตกต่างได้ และใช้ impl ในการ implement trait ออกมา เช่น impl Eat for Person โดย Eat คือ trait `

### 9.3 Type System

`static type system จะตวรจสอบชนิดของตัวแปรตั้งแต่ compile time และใช้ generic bound เพื่อบังคับว่าประเภทที่ใช้ต้องพฤติกรรมตามที่ trait กำหนดไว้จึงจะสามารถใช้งานได้ , Trait Bound ใช้กำหนดว่า Generic Type ต้อง implement Trait ใด Trait หนึ่ง เพื่อให้ function หรือ struct สามารถเรียกใช้งาน method ที่กำหนดใน Trait ได้ `

### 9.4 Memory / Resource Management

`ใช้ ownership และ borrowing ในการจัดการ Memory โดยตรวจสอบในระหว่าง complile time  โดยไม่จำเป็นต้องใช้ garbage collector`

### 9.5 Abstraction / Other PPL Concepts

`zero-cost abstraction สามารถใช้ abstraction ระดับสูงโดยไม่เสียประสิทธิภาพการทำงานลงไป และมี spoce บอก compiler ว่า borrow จะใช้งานได้เมื่อใด ทรัพยากรสามารถคืนได้เมือใด ตัวแปรที่สร้างจะถูกทำลายเมื่อใด`

### 9.6 Why Rust?

`มี ownership และ borrowing ช่วยจัดการ memory และ safety ลดปัญหา memory leaks และ crashes ได้มากขึ้น , Zero-cost Abstraction `

---

## 10. Rust vs. Other Language

**Comparison Language:** `[Python / C / C++ / Java ]`

| Aspect | Rust | Other Language |
|---|---|---|
| Syntax | `Rust : ใช้ Traits และ struct , impl ` | `Java : ใช้ interface , extends ,implements และ class `<br>` Python : ใช้ class โดยการ overriding method หรือ duck Typing  `<br>` C++ : ใช้ class Function Overloading , Function Overriding , Virtual Functions ` |
| Semantics / Behavior | ` Rust : โดย trait จะกำหนดพฤติกรรมที่มีร่วมกันให้แต่ละ struct ซึ่งแต่ละ struct สามารถกำหนดการทำงานของพฤติกรรมให้แตกต่างได้ ` | ` java : โดย interface จะกำหนดพฤติกรรมที่มีร่วมกันให้แต่ละ class ซึ่งแต่ละ class สามารถ implements หรือ overriding การทำงานให้แตกต่างได้ `<br> `Python : แต่ละ class สามารถมี method ชื่อเดียวกันได้แต่กำหนดการทำงานแตกต่างกันได้  โดยใช้  duck Typing หรือ overriding `<br>` C++ : โดยคลาสหลักสามารถกำหนด method ให้เป็น virtual method และคลาสลูกสามารถ override methods นั้นได้ ` |
| Type System | ` Rust : Static Typing ` | ` java : Static Typing `<br>` Python : Dynamic Typing `<br>` C++ : Static Typing  `|
| Memory Management | `Rust : ใช้ Ownership + Borrowing  ` | `java : ใช้ Garbage Collector `<br>` Python : ใช้ Reference Counting  และ Garbage Collection โปรแกรมเมอร์ไม่จำเป็นต้องจัดการเอง  `<br>` C++ : ใช้แบบ Dynamic Memory Allocation ในขณะ runtime โดยใช้ new และ deleteเพื่อคืนหน่วยความจำ(deallocate) ` |
| Safety | ` Rust : ปลอดภัยสูง เพราะ ใช้ ownership + borrowing  ในการตรวจสอบ `| ` java : มีsafety สูงเพราะมีการตรวจตอบโดย JVM หรือ Garbage Collector `<br>` Python : มีความยืดหยุ่นและใช้งานง่ายจาก dynamic typing  `<br>` C++ : โปรแกรมเมอร์สามารถจัดการ Memory โดยตรงได้ ซึ่งอาจทำให้เกิดปัญหาเกี่ยวกับ memory ได้ ` |

### Rust Example

```rust 
struct Person {
    name : String
}
struct Cat {
    name : String
}

trait Eat{
    fn eat_dinner(&self);
}

impl Eat for Person{
    fn eat_dinner(&self){
        println!("{} Yummy",self.name);
    }
}

impl Eat for Cat{
    fn eat_dinner(&self){
        println!("{} Num Num NUm",self.name);
    }
}

fn main() {
   let perr = Person{
    name : String::from("Coco")
   };
   perr.eat_dinner();
   let catt = Cat{
    name : String::from("Mr.Joe")
   };
    catt.eat_dinner();
}

```


### `[Other Language]` Example

python
``` python

class Person:
    def __init__(self, name):
        self.name = name

    def eat_dinner(self):
        print(self.name +" Yummy")


class Cat:
    def __init__(self, name):
        self.name = name

    def eat_dinner(self):
        print(self.name +" Num Num Num")


perr = Person("Coco")
perr.eat_dinner()

catt = Cat("Mr.Joe")
catt.eat_dinner()
```
 java
 ``` java 
interface Eat {
    void eat_dinner();
}

class Person implements Eat{
    String name ;

    Person(String name ){
        this.name = name ; 
    }

    public  void eat_dinner(){
        System.out.println(name +"  Yummy");
    }
}

class Cat implements Eat{
    String name ;

    Cat(String name ){
        this.name = name ; 
    }

    public  void eat_dinner(){
        System.out.println(name + " Num Num NUm");
    }
}

public class javacode {
    public static void main(String[] args) {
        Person perr = new Person("Coco");
        perr.eat_dinner();
        Cat catt = new Cat("Mr.Joe");
        catt.eat_dinner();
    }
    
}
```
c++
 ``` c++
#include <iostream>
using namespace std;

class Eat {
public:
    void eat_dinner() {
    }
};

class Person : public Eat {
public:
    string name;

    Person(string name) {
        this->name = name;
    }

    void eat_dinner() {
        cout << name << " Yummy\n";
    }
};

class Cat : public Eat {
public:
    string name;

    Cat(string name) {
        this->name = name;
    }

    void eat_dinner() {
        cout << name << " Num Num Num\n";
    }
};

int main() {
    Person perr("Coco");
    perr.eat_dinner();

    Cat catt("Mr.Joe");
    catt.eat_dinner();

    return 0;
}
```
---
### Analysis

`Rust จะไม่มี class และ Inheritance แต่จะใช้ trait เป็นตัวกำหนดพฤติกรรมที่สามารถนำไปใช้ได้แต่สามารถกำหนดการทำงานของพฤติกรรมให้แตกต่างกันได้ โดย trait สามารถมีหลาย method ได้และใช้ impl ในการ implement funtiocn ใน trait`

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

`ถอนไปแล้ว`

**Member 3**

`Rust VS OtherLanguage และ PPL Analysis`

**Member 4**

`Exercises + Common Mistakes + Challenge`

> สมาชิกทุกคนต้องสามารถอธิบาย Code ของกลุ่มได้ ไม่ใช่เฉพาะส่วนที่ตนเองเขียน

---

## 12. References

> แนะนำให้มีอย่างน้อย **4 แหล่งอ้างอิง** และควรใช้เอกสารทางการเป็นหลัก

1. `The Rust Programming Language - https://doc.rust-lang.org/book/title-page.html`
2. `Java Polymorphism - https://www.w3schools.com/java/java_polymorphism.asp`
3. `Polymorphism in C++ - https://www.udacity.com/blog/understanding-polymorphism-in-cpp/`
4. `Polymorphism in Python - https://www.codecademy.com/article/understanding-polymorphism-in-python`

---

## 13. AI Usage Declaration

สามารถใช้ AI เป็นเครื่องมือช่วยเรียนรู้และพัฒนาได้ แต่สมาชิกทุกคนต้องเข้าใจและสามารถอธิบายผลงานของกลุ่มได้

| AI Tool | Purpose | How the Result Was Verified |
|---|---|---|
| `ChatGPT` | `แปลบทความ rust และเอกสารต่างๆ` | `ทำการตรวจทานเพิ่มเติม` |
| `ChatGPT` | `ช่วยตรวจสอบ syntax ของ code` | `ทำการตรวจทาน และทำความเข้าใจใน code เพิ่มเติม` |


### Declaration

- [✓] Code ทุกส่วนที่นำเสนอได้รับการ Compile และทดสอบแล้ว
- [✓] สมาชิกทุกคนสามารถอธิบาย Code ที่นำเสนอได้
- [✓] ตรวจสอบข้อมูลจากแหล่งอ้างอิงที่น่าเชื่อถือแล้ว
- [✓] ระบุการใช้ AI อย่างโปร่งใส

**รายละเอียดการใช้ AI**

`ใช้ AI ในการแปลเอกสารต่างๆ และการตรวจสอบจากหลายๆแหล่งรวมกันเพื่อป้องกันความผิดพลาด`

---

## 14. GitHub Contribution

| Member | Issues | Commits | Pull Requests | Code Reviews | Contribution |
|---|---:|---:|---:|---:|---|
| Member 1 | `0` | `4` | `0` | `0` | `จัดทำหัวข้อ Concept + Short Code Illustration` |
| Member 2 | `0` | `0` | `0` | `0` | `ถอนไปแล้ว` |
| Member 3 | `0` | `24` | `0` | `0` | `จัดทำหัวข้อ Rust vs Other Language + PPL Analysis` |
| Member 4 | `0` | `21` | `0` | `0` | `จัดทำหัวข้อ Common Mistake , Exercise ,Challenge และทดสอบโค้ด` |

### Teamwork Reflection

**How did your team collaborate?**

`เริ่มจับกลุ่มคุยกันและแบ่งงานในส่วนที่แต่ละสมาชิกรับผิดชอบและนำขึ้น github และนำเนื้อกาที่รับผิดชอบใส่ในสไลด์โดยมีคนใดคนหนึ่งสร้าง branch และ link canva เพื่อทำงานรว่มกันและช่วยกันตรวจสอบความถูกต้อง`

**Problems encountered**

`ยังไม่คุ้นชินการใช้ github จึงต้องใช้เวลาศึกษาเพิ่มเติ่ม`

**How did you solve them?**

`สอบถามสมาชิกในกลุ่มที่มีประสบการณ์ใช้ github มาก่อน`

---

## 15. Final Checklist

- [✓] Learning Objectives ครบ 3–4 ข้อ
- [✓] Key Concepts ครบถ้วน
- [✓] Syntax / Rules
- [ ] Runnable Code Examples
- [ ] Code Compile และ Run ได้จริง
- [✓] Common Mistakes
- [✓] Exercises 2 ข้อ พร้อม Solutions
- [✓] PPL Perspective
- [✓] Rust vs Other Language
- [✓] References อย่างน้อย 4 แหล่ง
- [✓] AI Usage Declaration
- [✓] GitHub Contribution
- [ ] สมาชิกทั้ง 4 คนมีส่วนร่วม
- [ ] สมาชิกทั้ง 4 คนพร้อมนำเสนอคนละ 5 นาที
- [✓] สมาชิกทุกคนสามารถอธิบาย Code ของกลุ่มได้

---

## Submission Information

**Repository:** `https://github.com/670710629/rust-tutorial-2569`

**Chapter Path:** `[16-traits-polymorphism]`

**Final PR:** `#40`

**Submitted by:** `[Group 16]`

**Date:** `[2569-10-04]`

---

*โครงสร้างเอกสารฉบับเต็ม (Key Concepts, Runnable Code Examples, Common Mistakes, Exercises, PPL Perspective, Rust vs Other Language, References, AI Usage Declaration, GitHub Contribution, Final Checklist) ให้ทำต่อจากจุดนี้ตาม Template หลักของวิชา (`rust_tutorial_template.md`) ที่แนบมากับใบมอบหมายงาน*
