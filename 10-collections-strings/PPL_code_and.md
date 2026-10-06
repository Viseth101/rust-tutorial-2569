## 9. PPL Perspective

> **ส่วนนี้เป็นหัวใจของรายวิชา Principles of Programming Languages**

วิเคราะห์ Topic นี้ในมุมมองของ Programming Languages

### 9.1 Syntax
```rust
//let mut name = String::from("Rust");
//name.push_str(" Language");
```

---

#### `Vec<T>`

```rust
//let numbers: Vec<i32> = vec![10, 20, 30];
```

ตัวอย่างอื่น

```rust
//Vec<String>
//Vec<char>
//Vec<u8>
```

---

#### `String`

```rust
//let mut s = String::from("Hello");
//s.push('!');
```

---

#### `&str`


```rust
//let s = String::from("Hello");
//let part: &str = &s[0..5];
```


---


### การประกาศตัวแปรและ Mutability


```rust
//let name = String::from("Rust");
```


```rust
//let mut name = String::from("Rust");
//name.push_str(" Language");
```

---

### การสร้าง Collection

Collection แต่ละประเภทมี Syntax สำหรับสร้างแตกต่างกัน

```rust
// Vec
//let a: Vec<i32> = Vec::new();
//let b = vec![1, 2, 3];

// String
//let s1 = String::new();
//let s2 = String::from("Rust");

// &str
//let text: &str = "Rust";
```
---

### Syntax ของ String: `String` กับ `&str`


```rust
//let a = "Hello";
//let b = String::from("Hello");
```
---

### Syntax กับ UTF-8

ตัวอย่าง

```rust
//let s = String::from("สวัสดี");
```

ดังนั้น

```rust
//s.len()
```


---

### ทำไม Rust จึงไม่มี `String[0]` แบบ Character Index?

ถ้าเขียน

```rust
//s[0]
```

```rust
//s.bytes()
//s.chars()
//s.char_indices()
```

หรือ String Slice

```rust
//&s[start..end]
```

---

### 9.2 Semantics

ตัวอย่าง

```rust
//let mut s = String::from("Hello");
//s.push('!');
```
---

### Ownership, Move และ Borrowing ในมุมของ Semantics

```rust
//let s1 = String::from("Hello");
//let s2 = s1;
```
ในทางกลับกัน การ Borrow

```rust
//let s = String::from("Hello");
//let r = &s;
```
---

### Semantics ของ `Vec`

`Vec<T>` มีพฤติกรรมเป็น Collection แบบลำดับ

```rust
//let mut v = vec![10, 20, 30];
//v.push(40);
```
---

### Index กับ `get`

```rust
//let v = vec![10, 20, 30];

//let a = v[0];
//let b = v.get(0);
```

---

### 1. Byte
แต่ตัวอักษรที่อยู่นอก ASCII เช่นภาษาไทย ใช้หลาย Byte

ดังนั้น

```rust
//let s = "ก";
//println!("{}", s.len());
```

จะได้จำนวน Byte ไม่ใช่ “จำนวนตัวอักษร”

---

### 2. Unicode Scalar Value

ใน Rust `char` แทน **Unicode Scalar Value**

```rust
//for c in s.chars() {
    //println!("{}", c);
//}
```

ดังนั้น `chars()` ไม่ได้วนทีละ Byte แต่แปลง UTF-8 เป็น Unicode Scalar Values ตามลำดับ
---

### 4. String Slice

```rust
//let s = String::from("Hello");
//let part: &str = &s[0..3];
```

`part` เป็น View ไปยังข้อมูลเดิม ไม่ใช่ String ชุดใหม่ในความหมายของ Ownership

---

### `len()` กับความหมายของ Length

```rust
//let s = String::from("สวัสดี");
//println!("{}", s.len());
```

`String::len()` วัดเป็น **Byte**
---

### ทำไม Rust ไม่ให้ `String[0]`?

```rust
//s[0]
```
ผู้เขียนต้องเลือกวิธีที่สื่อความหมาย เช่น

```rust
//s.bytes()
//s.chars()
//s.char_indices()
```
---

### Semantics ของ `Option`

Collection บาง API คืน `Option`

```rust
//let value = v.get(10);
```
---

### 9.3 Type System
---

### Static Typing

ตัวอย่าง

```rust
//let x: i32 = 10;
//let s: String = String::from("Rust");
```
---

### Type Inference

```rust
//let x = 10;
//let s = String::from("Rust");
```
---

### Monomorphization

เช่น

```rust
//fn print_value<T>(x: T) {
    // ...
//}
```
---


### `String`

```rust
//let s = String::from("Rust");
```

### `&str`

```rust
//let s = String::from("Rust");
//let view: &str = &s;
```

---

### `String`, `&String` และ `&str`

ตัวอย่าง

```rust
//fn show(text: &str) {
    //println!("{}", text);
//}

//let s = String::from("Rust");
//show(&s);
```
---

### UTF-8 และ Type Safety

```rust
//let bytes: Vec<u8> = vec![...];
```


```rust
//let result = String::from_utf8(bytes);
```

---

### `Option<T>` ใน Type System



```rust
//Some(value)
//None
```

ตัวอย่าง

```rust
//let value = numbers.get(0);
```

---

### 9.4 Memory / Resource Management
ตัวอย่างเช่น

```rust
//let s = "ก";
//println!("{}", s.len());
```
---

### Allocation
ตัวอย่าง

```rust
//let mut v = Vec::new();
//v.push(1);
//v.push(2);
```

หรือ

```rust
//let mut s = String::new();
//s.push_str("Rust");
```
---

### ความสัมพันธ์ระหว่าง Reallocation กับ Borrowing

สมมติ

```rust
//let mut v = vec![1, 2, 3];

//let first = &v[0];

// ถ้า operation นี้ต้อง Reallocate
//v.push(4);
```
---

### Deallocation

เมื่อ Owner ของ Resource หมด Scope Rust สามารถจัดการ Resource ที่เกี่ยวข้องโดยอัตโนมัติ

```rust
//{
    //let s = String::from("Rust");
//} // s ถูก Drop
```
---

### Ownership กับ Memory

ตัวอย่าง

```rust
//let s1 = String::from("Rust");
//let s2 = s1;
```
---

### Borrowing กับ Memory

Reference เป็นการยืมข้อมูล

```rust
//let s = String::from("Rust");
//let r = &s;
```
---

---

### 9.5 Abstraction / Other PPL Concepts

ตัวอย่าง

```rust
//let mut s = String::from("Hello");
//s.push_str(" Rust");
```
---

## Abstraction ไม่ได้แปลว่า “ซ่อนทุกอย่าง”

Rust เปิดให้ผู้เขียนเลือกว่าจะทำงานในระดับใด

```rust
//s.len()
```

→ มอง Length ในระดับ Byte

```rust
//s.as_bytes()
```

→ มอง Representation เป็น Bytes

```rust
//s.chars()
```

→ มอง Unicode Scalar Values

```rust
//s.char_indices()
```
---

## String เป็น Abstraction ของ UTF-8 Text

```rust
//String::from_utf8(bytes)
```

ถ้า Byte Sequence ไม่ถูกต้อง ก็ไม่ควรสร้าง `String` ปกติขึ้นมา

---

## ทำไม String ไม่ใช่ `Vec<char>`?

ถ้ามองจากชื่อ “String = ตัวอักษรหลายตัว” อาจเกิดความคิดว่า

```text
//String = Vec<char>
```

---

## Trait

Trait ใช้กำหนดพฤติกรรมหรือความสามารถร่วมกันของ Type

เช่น

```rust
//trait Printable {
    //fn print(&self);
//}
```

แนวคิดนี้ช่วยให้สามารถสร้าง Abstraction ที่ไม่ได้ผูกกับ Type ใด Type หนึ่งโดยตรง

---
## Iterator

Iterator เป็น Abstraction สำหรับการประมวลผลข้อมูลทีละ Element

ตัวอย่าง

```rust
//let numbers = vec![1, 2, 3, 4, 5];

//let result: Vec<i32> = numbers
    //.iter()
    //.map(|x| x * 2)
    //.filter(|x| x > &5)
    //.collect();
```

---

## Scope และ Binding

**Scope** คือขอบเขตที่ Binding สามารถใช้งานได้

```rust
//{
    //let s = String::from("Rust");
//}
```
---

## Programming Paradigm

Rust รองรับหลาย Programming Paradigm

### Imperative Programming

เน้นการสั่งให้โปรแกรมทำงานเป็นขั้นตอน

```rust
//let mut total = 0;

//for n in numbers {
    //total += n;
//}
```

### Functional Programming

สามารถใช้ Iterator และ Function มาประกอบกัน

```rust
//let total: i32 = numbers
    //.iter()
    //.map(|x| x * 2)
    //.sum();
```

ดังนั้น Rust สามารถผสมแนวคิดหลาย Paradigm ได้

---

### 9.6 Why Rust?

## Rust ถูกออกแบบมาเพื่ออะไร?
---

## 5. ทำไม Rust ไม่ให้ `String[index]`?

เพราะ Rust ไม่ต้องการสร้าง Abstraction ที่อาจทำให้เกิดความเข้าใจผิด

ถ้าเขียน

```rust
//s[0]
```
---

## 10. Rust vs. Other Language

**Comparison Language:** `[Python / C / C++ / Java /]`

### Rust Example

```rust

fn main() {
    // Vec<T>: เหมือน list ที่ขยายขนาดได้เรื่อย ๆ
    let mut numbers: Vec<i32> = Vec::new();
    numbers.push(1);
    numbers.push(2);
    numbers.push(3);

    // ยืมดู (borrow) เพื่ออ่านค่า ไม่ได้เอาไปเป็นของตัวเอง
    let total: i32 = numbers.iter().sum();
    println!("Sum = {}", total);

    // String: ข้อความที่เราเป็นเจ้าของเต็มตัว แก้ไข/ต่อได้
    let mut name = String::from("Rust");
    name.push_str(" Language");

    // &str: แค่ "มองไปที่" ส่วนหนึ่งของ String เท่านั้น (view)
    let slice: &str = &name[0..4];
    println!("{} | {}", name, slice);

    // Move semantics: ยกความเป็นเจ้าของจาก name ไปให้ name2
    let name2 = name; // ตั้งแต่นี้ name ใช้งานต่อไม่ได้แล้ว (compile error ถ้าเรียก name อีก)
    println!("{}", name2);
}

```

### `[Other Language]` Example

> python
``` 
def main():
    # list: เหมือน Vec ในภาษา Rust ใส่ของเพิ่มได้เรื่อย ๆ
    numbers = []
    numbers.append(1)
    numbers.append(2)
    numbers.append(3)

    total = sum(numbers)
    print(f"Sum = {total}")

    # str: ข้อความที่แก้ไขไม่ได้เลย (immutable)
    name = "Python"
    name = name + " Language"  # จริง ๆ คือสร้าง object ใหม่ทั้งชิ้น เพราะ str แก้ไขตรง ๆ ไม่ได้

    slice_ = name[0:6]
    print(f"{name} | {slice_}")

    # Reference semantics: name2 แค่ "ชี้" ไปที่ object เดียวกับ name
    name2 = name  # name ยังใช้งานได้ตามปกติ ไม่มีการยึดไปเป็นของ name2 ฝ่ายเดียว
    print(name2)


main()
```

> C
``` 
#include <stdio.h>
#include <string.h>
#include <stdlib.h>

int main(void) {
    // C ไม่มี dynamic array ให้ในตัว ต้องขอความจำเองด้วย malloc
    int *numbers = malloc(3 * sizeof(int));
    numbers[0] = 1;
    numbers[1] = 2;
    numbers[2] = 3;

    int total = 0;
    for (int i = 0; i < 3; i++) {
        total += numbers[i];
    }
    printf("Sum = %d\n", total);

    // string ใน C คือ char array ที่จบด้วย \0 ต้องจองที่ให้พอกับข้อความสุดท้าย
    char name[20];
    strcpy(name, "C");
    strcat(name, " Language");

    char slice[2];
    strncpy(slice, name, 1);
    slice[1] = '\0';
    printf("%s | %s\n", name, slice);

    // ค่าตัวแปรถูก copy ตรง ๆ ไม่มีแนวคิดเจ้าของแบบ Rust
    char name2[20];
    strcpy(name2, name); // name ยังใช้งานต่อได้ตามปกติ เพราะเป็นการ copy ข้อมูลจริง
    printf("%s\n", name2);

    free(numbers); // ต้องคืนความจำเองด้วยมือ ลืมคืนคือ memory leak
    return 0;
}
```

> C++
``` 
#include <iostream>
#include <vector>
#include <string>
#include <numeric>

int main() {
    // std::vector: dynamic array ที่จัดการความจำให้อัตโนมัติ (คล้าย Vec ของ Rust)
    std::vector<int> numbers;
    numbers.push_back(1);
    numbers.push_back(2);
    numbers.push_back(3);

    int total = std::accumulate(numbers.begin(), numbers.end(), 0);
    std::cout << "Sum = " << total << std::endl;

    // std::string: จัดการความจำให้อัตโนมัติเหมือน String ของ Rust
    std::string name = "C++";
    name += " Language";

    std::string slice = name.substr(0, 3); // ดึงข้อความส่วนหนึ่งออกมา (เป็นการ copy)
    std::cout << name << " | " << slice << std::endl;

    // ค่าเริ่มต้นคือ copy semantics: name2 ได้ข้อมูลชุดใหม่ที่แยกจาก name
    std::string name2 = name; // name ยังใช้งานต่อได้ตามปกติ เพราะถูก copy ไม่ใช่ move
    std::cout << name2 << std::endl;

    return 0;
}
```

> Java
```
import java.util.ArrayList;

public class Main {
    public static void main(String[] args) {
        // ArrayList: dynamic array ที่ Garbage Collector จัดการความจำให้
        ArrayList<Integer> numbers = new ArrayList<>();
        numbers.add(1);
        numbers.add(2);
        numbers.add(3);

        int total = 0;
        for (int n : numbers) {
            total += n;
        }
        System.out.println("Sum = " + total);

        // String ใน Java เป็น immutable object เหมือนใน Python
        String name = "Java";
        name = name + " Language"; // สร้าง object ใหม่ทั้งชิ้น เพราะ String แก้ไขตรง ๆ ไม่ได้

        String slice = name.substring(0, 4);
        System.out.println(name + " | " + slice);

        // Reference semantics: name2 ชี้ไปที่ object เดียวกับ name
        String name2 = name; // name ยังใช้งานได้ตามปกติ ไม่มีการยึดไปฝ่ายเดียว
        System.out.println(name2);
    }
}
```
