# 🦀 Rust Tutorial Project — Principles of Programming Languages

> **Topic No.:** `22`  
> **Topic Name:** `Concurrency & Threads`  
> **Group No.:** `22`

> Tutorial ภาษาไทยสำหรับรายวิชา **517321 Principles of Programming Languages**

---

## 1. Members

| # | Name | Student ID | GitHub Username | Main Responsibility |
|---|---|---|---|---|
| 1 | นายกรวิชญ์ บุญชู | `630710836` | `@[630710836]` | Concept + Short Code |
| 2 | นายผกาย เมืองแมน | `640710542` | `@[640710542]` | Detailed Code + Live Demo |
| 3 | นางสาวศศิธร โตนาม | `640710572` | `@[username]` | Rust vs Other Language + PPL |
| 4 | นายธนภัทร คงหอม | `640710845` | `@[username]` | Exercises + Common Mistakes |

> สมาชิกทุกคนต้องสามารถอธิบายเนื้อหาและ Code ของกลุ่มได้ทั้งหมด ไม่ใช่เฉพาะส่วนที่ตนเองรับผิดชอบ

---

## 2. Learning Objectives

หลังจากศึกษา Topic นี้แล้ว ผู้เรียนสามารถ:

1. อธิบายแนวคิด **Concurrency & Threads** ในภาษา Rust ได้
2. สร้างและจัดการ Thread ด้วย `std::thread::spawn()` และ `.join()` ได้
3. ใช้ **Message Passing** ผ่าน `mpsc::channel()` และ **Shared State** ผ่าน `Arc<Mutex<T>>` ได้
4. วิเคราะห์ Concurrency ของ Rust ในมุมมองของ **Syntax, Semantics, Type System และ Memory Management**
5. เปรียบเทียบแนวทาง Concurrency ของ Rust กับภาษาโปรแกรมอื่นได้
6. อธิบายบทบาทของ `Send` และ `Sync` ต่อ Concurrency Safety ได้

---

## 3. Introduction

**Concurrency** คือแนวทางการออกแบบโปรแกรมที่ทำให้หลายส่วนของโปรแกรมสามารถดำเนินงานไปพร้อมกันหรือสลับการทำงานกันได้

ใน Rust การทำงานแบบ Concurrent มีแนวคิดสำคัญคือ **Fearless Concurrency** ซึ่งใช้ Ownership System, Borrowing, Type System และ Traits เช่น `Send` และ `Sync` เพื่อช่วยตรวจสอบความปลอดภัยของการทำงานข้าม Thread ตั้งแต่ Compile-time

Concurrency ช่วยให้โปรแกรมสามารถใช้ทรัพยากรของระบบได้อย่างมีประสิทธิภาพ แต่ก็มีปัญหาที่ต้องระวัง เช่น:

- **Data Race** — หลาย Thread เข้าถึงและแก้ไขข้อมูลเดียวกันอย่างไม่ปลอดภัย
- **Deadlock** — Thread รอ Resource ซึ่งกันและกันจนไม่สามารถทำงานต่อได้
- **Race Condition** — ผลลัพธ์ขึ้นอยู่กับลำดับหรือจังหวะการทำงานของ Thread

Rust จึงออกแบบกลไกด้าน Ownership และ Type System เพื่อช่วยป้องกันปัญหาเหล่านี้ตั้งแต่ก่อน Runtime

---

## 4. Key Concepts

### 4.1 Thread Creation & Lifecycle

**คำอธิบาย**

Rust ใช้ `std::thread::spawn()` เพื่อสร้าง Thread ใหม่ และใช้ `JoinHandle` สำหรับจัดการ Thread ที่ถูกสร้างขึ้น

`.join()` ใช้สำหรับรอให้ Thread ทำงานเสร็จสิ้นก่อนที่ Main Thread จะดำเนินการต่อ

**ตัวอย่าง**

```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        println!("Hello from spawned thread!");
    });

    handle.join().unwrap();
}
```

**Explanation**

`thread::spawn()` รับ Closure สำหรับให้ Thread ใหม่ทำงาน ส่วน `.join()` ทำให้ Main Thread รอ Thread ที่สร้างขึ้นจนเสร็จ

---

### 4.2 Message Passing with `mpsc`

Rust รองรับการสื่อสารระหว่าง Thread ผ่าน Channel โดยใช้ `std::sync::mpsc`

`mpsc` ย่อมาจาก:

> **Multiple Producer, Single Consumer**

ประกอบด้วย `Sender` สำหรับส่งข้อมูล และ `Receiver` สำหรับรับข้อมูล

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send(String::from("Hello from thread!"))
            .unwrap();
    });

    let received = rx.recv().unwrap();

    println!("Received: {}", received);
}
```

---

### 4.3 Shared State with `Arc<Mutex<T>>`

เมื่อหลาย Thread จำเป็นต้องเข้าถึงข้อมูลเดียวกัน สามารถใช้ `Arc<Mutex<T>>`

- `Arc<T>` — Atomic Reference Counting ใช้แชร์ Ownership ระหว่าง Thread
- `Mutex<T>` — Mutual Exclusion ใช้ควบคุมการเข้าถึงข้อมูล
- `Arc<Mutex<T>>` — ใช้ร่วมกันเพื่อสร้าง Shared State ที่ปลอดภัย

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Counter: {}", *counter.lock().unwrap());
}
```

---

### 4.4 `Send` and `Sync`

`Send` และ `Sync` เป็น Traits ที่มีบทบาทสำคัญต่อ Thread Safety

- **`Send`** — Ownership ของ Type สามารถส่งข้าม Thread ได้อย่างปลอดภัย
- **`Sync`** — Reference ของ Type สามารถแชร์ข้าม Thread ได้อย่างปลอดภัย

Compiler จะใช้ข้อมูลจาก Type System เพื่อช่วยตรวจสอบว่าการใช้งานประเภทข้อมูลข้าม Thread เป็นไปตามข้อกำหนดหรือไม่

---

### 4.5 Fearless Concurrency

แนวคิด **Fearless Concurrency** คือแนวคิดที่ Rust พยายามทำให้การเขียน Concurrent Program มีความปลอดภัยมากขึ้น โดยให้ Compiler ช่วยตรวจสอบกฎด้าน Ownership, Borrowing และ Type System

```text
Ownership
    ↓
Borrowing
    ↓
Type System
    ↓
Send / Sync
    ↓
Concurrency Safety
```

---

## 5. Important Syntax / Rules

| Syntax / Rule | Meaning | Example |
|---|---|---|
| `thread::spawn()` | สร้าง Thread ใหม่ | `thread::spawn(|| {})` |
| `.join()` | รอ Thread ให้ทำงานเสร็จ | `handle.join().unwrap()` |
| `move` | ย้าย Ownership เข้า Closure | `spawn(move || {})` |
| `mpsc::channel()` | สร้าง Channel สำหรับ Message Passing | `let (tx, rx) = mpsc::channel()` |
| `Arc<T>` | แชร์ Ownership แบบ Atomic | `Arc::clone(&data)` |
| `Mutex<T>` | ควบคุมการเข้าถึง Shared State | `data.lock().unwrap()` |
| `Send` | ส่ง Ownership ข้าม Thread | `Send` |
| `Sync` | แชร์ Reference ข้าม Thread | `Sync` |

### Important Rules

1. Thread ที่ต้องใช้ข้อมูลจาก Scope ภายนอกอาจต้องใช้ `move` เพื่อย้าย Ownership เข้า Closure
2. Shared State ที่ถูกแก้ไขจากหลาย Thread ต้องมีการ Synchronization ที่เหมาะสม เช่น `Mutex`
3. `MutexGuard` จะคืน Lock เมื่อหลุดออกจาก Scope
4. Type ที่ถูกส่งหรือแชร์ข้าม Thread ต้องเป็นไปตามข้อกำหนดของ `Send` / `Sync`
5. `Rc<T>` ไม่เหมาะสำหรับการแชร์ Ownership ข้าม Thread ควรใช้ `Arc<T>` ในกรณีดังกล่าว

---

## 6. Runnable Code Examples

> **ข้อกำหนด:** Code ทุกตัวต้อง Compile และ Run ได้จริงก่อนนำมาใส่ในเอกสาร

### Example 1 — Thread Creation & Join

**Purpose:** สาธิตการสร้าง Thread และรอ Thread ด้วย `.join()`

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..=3 {
            println!("[Spawned Thread] {}", i);
            thread::sleep(Duration::from_millis(10));
        }
    });

    for i in 1..=2 {
        println!("[Main Thread] {}", i);
        thread::sleep(Duration::from_millis(10));
    }

    handle.join().unwrap();

    println!("Spawned Thread finished.");
}
```

**Expected Output**

```text
[Main Thread] 1
[Spawned Thread] 1
[Main Thread] 2
[Spawned Thread] 2
[Spawned Thread] 3
Spawned Thread finished.
```

> ลำดับของ Output อาจแตกต่างกันได้ตาม Thread Scheduling

**Explanation**

Main Thread และ Spawned Thread สามารถทำงานสลับกันได้ และ `.join()` ทำให้ Main Thread รอ Spawned Thread ให้เสร็จ

---

### Example 2 — Shared State with `Arc<Mutex<T>>`

**Purpose:** สาธิตการแชร์ข้อมูลและแก้ไขข้อมูลร่วมกันระหว่างหลาย Thread

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..10 {
        let counter_clone = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            let mut num = counter_clone.lock().unwrap();

            *num += 1;

            println!(
                "Thread {} -> Counter = {}",
                i, *num
            );
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!(
        "Final Counter = {}",
        *counter.lock().unwrap()
    );
}
```

**Expected Output**

```text
Thread 0 -> Counter = 1
Thread 1 -> Counter = 2
Thread 2 -> Counter = 3
...
Thread 9 -> Counter = 10
Final Counter = 10
```

**Explanation**

`Arc` ทำให้หลาย Thread สามารถถือ Reference ไปยัง Counter เดียวกัน ส่วน `Mutex` ทำหน้าที่ป้องกันไม่ให้หลาย Thread แก้ไข Counter พร้อมกันในช่วงเวลาเดียวกัน

---

## 7. Common Mistakes

### Mistake 1 — Forget `move`

**Problem**

```rust
let v = vec![1, 2, 3];

thread::spawn(|| {
    println!("{:?}", v);
});
```

**Correct Code**

```rust
let v = vec![1, 2, 3];

thread::spawn(move || {
    println!("{:?}", v);
});
```

**Why?**

Thread อาจมี Lifetime ยาวกว่า Scope ที่สร้างมันขึ้นมา Compiler จึงไม่สามารถรับประกันว่า `v` จะยังมีชีวิตอยู่ การใช้ `move` ทำให้ Ownership ของ `v` ถูกย้ายเข้า Closure

---

### Mistake 2 — Using `Rc<T>` Across Threads

**Problem**

```rust
use std::rc::Rc;

let data = Rc::new(5);

thread::spawn(move || {
    println!("{}", data);
});
```

**Correct Code**

```rust
use std::sync::Arc;

let data = Arc::new(5);

thread::spawn(move || {
    println!("{}", data);
});
```

**Why?**

`Rc<T>` ใช้ Reference Counting สำหรับ Single-threaded Context ส่วน `Arc<T>` ใช้ Atomic Reference Counting ซึ่งออกแบบมาเพื่อรองรับการแชร์ Ownership ระหว่าง Thread

---

## 8. Exercises

### Exercise 1 — Parallel Sum

**Problem**

แบ่งตัวเลขออกเป็น 2 ส่วน แล้วให้ Thread สองตัวคำนวณผลรวมแต่ละส่วน จากนั้นนำผลลัพธ์กลับมารวมกันบน Main Thread

**Hint**

ใช้:

- `.to_vec()`
- `thread::spawn()`
- `move`
- `.join()`

**Solution**

```rust
use std::thread;

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let mid = numbers.len() / 2;

    let left = numbers[..mid].to_vec();
    let right = numbers[mid..].to_vec();

    let h1 = thread::spawn(move || {
        left.iter().sum::<i32>()
    });

    let h2 = thread::spawn(move || {
        right.iter().sum::<i32>()
    });

    let sum = h1.join().unwrap() + h2.join().unwrap();

    println!("Total Sum: {}", sum);
}
```

**Expected Output**

```text
Total Sum: 55
```

**Explanation**

Array ถูกแบ่งเป็นสองส่วนและส่ง Ownership ของแต่ละส่วนไปยัง Thread คนละตัว จากนั้น Main Thread ใช้ `.join()` เพื่อรับผลลัพธ์กลับมา

---

### Exercise 2 — Shared Counter

**Problem**

สร้างโปรแกรมที่มี Thread จำนวน 5 ตัว โดยแต่ละ Thread เพิ่มค่า Counter ขึ้น 10 ครั้ง และใช้ `Arc<Mutex<i32>>` เพื่อป้องกัน Race Condition

**Hint**

ใช้:

```rust
Arc::new(Mutex::new(0))
```

จากนั้นใช้:

```rust
Arc::clone()
```

เพื่อให้แต่ละ Thread เข้าถึง Counter เดียวกัน

**Solution**

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..5 {
        let counter_clone = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            for _ in 0..10 {
                let mut value = counter_clone.lock().unwrap();
                *value += 1;
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final Counter: {}", *counter.lock().unwrap());
}
```

**Expected Output**

```text
Final Counter: 50
```

**Explanation**

แต่ละ Thread เพิ่ม Counter จำนวน 10 ครั้ง และมีทั้งหมด 5 Threads ดังนั้นผลลัพธ์สุดท้ายคือ `50`

`Mutex` ทำให้การเพิ่มค่า Counter ถูกควบคุมไม่ให้หลาย Thread แก้ไขข้อมูลพร้อมกันในช่วงเดียวกัน

---

## 9. PPL Perspective

> **ส่วนนี้เป็นหัวใจของรายวิชา Principles of Programming Languages**

### 9.1 Syntax

Concurrency ใน Rust ใช้ Syntax หลายส่วน เช่น:

```rust
thread::spawn(move || {
    // concurrent code
});
```

Construct ที่เกี่ยวข้องได้แก่:

- Function Call
- Closure
- `move`
- Method Call
- Generic Type เช่น `Mutex<T>` และ `Arc<T>`

---

### 9.2 Semantics

`thread::spawn()` มีพฤติกรรมในการสร้าง Thread ใหม่เพื่อให้ Closure ทำงานแยกจาก Thread ที่เรียกใช้งาน

ส่วน `.join()` ทำให้ Thread ที่เรียกสามารถรอให้ Spawned Thread ทำงานเสร็จ

สำหรับ `Mutex<T>` การเรียก `.lock()` จะพยายามขอ Lock ก่อนเข้าถึงข้อมูลภายใน

---

### 9.3 Type System

Rust ใช้ Type System เป็นส่วนหนึ่งของกลไก Concurrency Safety

Traits สำคัญคือ:

- `Send`
- `Sync`

Compiler สามารถปฏิเสธการใช้งาน Type ที่ไม่เป็นไปตามข้อกำหนดของ Thread Safety

ตัวอย่างเช่น `Rc<T>` ไม่สามารถใช้แทน `Arc<T>` ในกรณีที่ต้องแชร์ Ownership ระหว่าง Thread

---

### 9.4 Memory / Resource Management

Concurrency เชื่อมโยงกับ Memory Management ผ่าน:

- Ownership
- Borrowing
- Scope
- Smart Pointers
- RAII

ตัวอย่างเช่น `MutexGuard` จะถูกคืนเมื่อหลุดออกจาก Scope ซึ่งช่วยลดความจำเป็นในการจัดการ Lock ด้วยตนเอง

---

### 9.5 Abstraction / Other PPL Concepts

Rust สร้าง Abstraction สำหรับ Concurrency ผ่าน:

```text
Thread
├── Message Passing
│   └── mpsc
│
└── Shared State
    ├── Arc
    └── Mutex
```

แนวคิดเหล่านี้ทำให้ Programmer สามารถจัดการ Concurrent Program ผ่าน Abstraction ระดับสูง โดยยังคงใช้ Type System ของภาษาในการตรวจสอบความถูกต้อง

---

### 9.6 Why Rust?

Rust ให้ความสำคัญกับ:

- Memory Safety
- Type Safety
- Concurrency Safety
- Reliability
- Performance

โดยใช้ Compiler เป็นส่วนสำคัญในการตรวจสอบกฎของ Ownership และ Type System ก่อน Runtime

---

## 10. Rust vs. Other Language

**Comparison Language:** C++

| Aspect | Rust | C++ |
|---|---|---|
| Syntax | `thread::spawn`, `Arc`, `Mutex`, `mpsc` | `std::thread`, `std::mutex`, C++ synchronization tools |
| Semantics / Behavior | Concurrency ผูกกับ Ownership และ Type System | Programmer ต้องจัดการ Lifetime และ Synchronization อย่างระมัดระวัง |
| Type System | มี `Send` / `Sync` และ Ownership | มี Static Type System แต่ไม่มีกลไก Ownership แบบ Rust |
| Memory Management | Ownership + Borrowing + RAII | Manual / RAII |
| Safety | Compiler ช่วยตรวจสอบ Memory และ Concurrency Safety หลายกรณี | ความปลอดภัยหลายส่วนขึ้นอยู่กับการเขียนโปรแกรมและการจัดการ Resource |

### Rust Example

```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        println!("Hello from Rust thread!");
    });

    handle.join().unwrap();
}
```

### C++ Example

```cpp
#include <iostream>
#include <thread>

void task() {
    std::cout << "Hello from C++ thread!";
}

int main() {
    std::thread t(task);
    t.join();

    return 0;
}
```

### Analysis

Rust และ C++ ต่างสามารถสร้าง Thread ได้ แต่ Rust ผูกแนวคิดด้าน Memory Safety และ Concurrency Safety เข้ากับ Ownership และ Type System ของภาษา

ในขณะที่ C++ ให้ Programmer มีอิสระในการจัดการ Memory และ Thread มากกว่า ดังนั้น Programmer ต้องรับผิดชอบการจัดการ Resource และ Synchronization มากขึ้น

---

## 11. Teach Your Topic

การนำเสนอมีสมาชิก **4 คน คนละประมาณ 5 นาที**

| Member | Responsibility | Time |
|---|---|---:|
| Member 1 — นายกรวิชญ์ บุญชู | Concept + Short Code Illustration | 5 min |
| Member 2 — นายผกาย เมืองแมน | Detailed Code + Live Demo | 5 min |
| Member 3 — นางสาวศศิธร โตนาม | Rust vs Other Language + PPL Analysis | 5 min |
| Member 4 — นายธนภัทร คงหอม | Exercises + Common Mistakes + Challenge | 5 min |

### Individual Contribution

**Member 1**

Concept, Introduction และ Short Code Illustration

**Member 2**

Detailed Code และ Live Demo ของ `mpsc` / `Arc<Mutex<T>>`

**Member 3**

Rust vs Other Language และ PPL Analysis

**Member 4**

Exercises, Common Mistakes และ Challenge

> สมาชิกทุกคนต้องสามารถอธิบาย Code ของกลุ่มได้ ไม่ใช่เฉพาะส่วนที่ตนเองเขียน

---

## 12. References

1. [The Rust Programming Language — Chapter 16: Fearless Concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html)
2. [Rust Documentation — `std::thread`](https://doc.rust-lang.org/std/thread/)
3. [Rust Documentation — `std::sync::mpsc`](https://doc.rust-lang.org/std/sync/mpsc/)
4. [Rust Documentation — `std::sync::Mutex`](https://doc.rust-lang.org/std/sync/struct.Mutex.html)
5. [Rust by Example](https://doc.rust-lang.org/rust-by-example/)

---

## 13. AI Usage Declaration

สามารถใช้ AI เป็นเครื่องมือช่วยเรียนรู้และพัฒนาได้ แต่สมาชิกทุกคนต้องเข้าใจและสามารถอธิบายผลงานของกลุ่มได้

| AI Tool | Purpose | How the Result Was Verified |
|---|---|---|
| `ChatGPT` | ช่วยจัดโครงสร้างเอกสารและอธิบายแนวคิด | ตรวจสอบกับ Rust Documentation และทดลอง Compile / Run |
| `Claude` | ช่วยเรียบเรียงและตรวจสอบเนื้อหา | สมาชิกตรวจสอบเนื้อหาและ Code ด้วยตนเอง |

### Declaration

- [x] Code ทุกส่วนที่นำเสนอได้รับการ Compile และทดสอบแล้ว
- [x] สมาชิกทุกคนสามารถอธิบาย Code ที่นำเสนอได้
- [x] ตรวจสอบข้อมูลจากแหล่งอ้างอิงที่น่าเชื่อถือแล้ว
- [x] ระบุการใช้ AI อย่างโปร่งใส

### รายละเอียดการใช้ AI

AI ถูกใช้เพื่อช่วยจัดระเบียบโครงสร้างเอกสารตาม Template ช่วยเรียบเรียงคำอธิบาย และช่วยตรวจสอบรูปแบบของ Code และ Markdown

สมาชิกกลุ่มเป็นผู้ตรวจสอบผลลัพธ์ด้วยตนเอง และทดสอบ Code ด้วย Cargo ก่อนนำมาใช้ใน Tutorial และการนำเสนอ

---

## 14. GitHub Contribution

| Member | Issues | Commits | Pull Requests | Code Reviews | Contribution |
|---|---:|---:|---:|---:|---|
| นายกรวิชญ์ บุญชู | 2 | 3 | 1 | 2 | Concept + Short Code |
| นายผกาย เมืองแมน | 2 | 4 | 1 | 2 | Detailed Code + Live Demo |
| นางสาวศศิธร โตนาม | 2 | 3 | 1 | 2 | Rust vs Other Language + PPL |
| นายธนภัทร คงหอม | 2 | 3 | 1 | 2 | Exercises + Common Mistakes |

### Teamwork Reflection

**How did your team collaborate?**

สมาชิกแบ่งงานผ่าน GitHub Issues และพัฒนาใน Branch ของตนเอง จากนั้นรวมผลงานผ่าน Pull Request

**Problems encountered**

ปัญหาหลักคือการจัดโครงสร้าง Tutorial ให้ครอบคลุมทั้งเนื้อหา Rust และการวิเคราะห์ในมุมมอง PPL รวมถึงการตรวจสอบ Code ตัวอย่างให้สามารถ Compile และ Run ได้จริง

**How did you solve them?**

สมาชิกช่วยกันตรวจสอบ Code, ทบทวนเนื้อหา และทำ Code Review ผ่าน Pull Request ก่อน Merge เข้า Branch หลัก

---

## 15. Final Checklist

### Tutorial

- [x] Learning Objectives
- [x] Introduction
- [x] Key Concepts
- [x] Important Syntax / Rules
- [x] Runnable Code Examples
- [x] Common Mistakes
- [x] Exercises **2 ข้อ**
- [x] PPL Perspective
- [x] Rust vs Other Language
- [x] Teach Your Topic
- [x] References
- [x] AI Usage Declaration
- [x] GitHub Contribution

### Code

- [x] Code อยู่ใน Cargo Project
- [x] Code สามารถ Compile ได้
- [x] Code สามารถ Run ได้
- [x] ทดสอบ Runnable Examples แล้ว

### GitHub

- [x] Issues
- [x] Branches
- [x] Commits
- [x] Pull Requests
- [x] Code Reviews
- [x] Merge เข้า Branch หลัก

### Presentation

- [x] Member 1 — Concept + Short Code
- [x] Member 2 — Detailed Code + Live Demo
- [x] Member 3 — Rust vs Other Language + PPL Analysis
- [x] Member 4 — Exercises + Common Mistakes + Challenge
- [ ] เตรียม Live Demo
- [ ] เตรียม Q&A

---

## 📌 Submission Information

| Item | Information |
|---|---|
| **Topic** | 22 — Concurrency & Threads |
| **Course** | 517321 Principles of Programming Languages |
| **Group** | 22 |
| **Submission** | 4 October 2026, 23:59 |
| **Repository** | `[https://github.com/soonklang/rust-tutorial-2569/tree/main/22-concurrency-threads]` |
| **Pull Request** | `#[26]` |

---

<div align="center">

### 🦀 Concurrency & Threads in Rust

**517321 Principles of Programming Languages**

*Fearless Concurrency through Ownership, Types and Abstraction.*

</div>