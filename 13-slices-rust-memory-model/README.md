# Rust Tutorial Project — Principles of Programming Languages

> **กลุ่มที่:** 13
> **Topic No.:** 13
> **Topic Name:** Slices & Rust Memory Model
> **ประเด็นหลักที่ควรครอบคลุม:** slices, &str, array/vector slices, stack vs heap และความสัมพันธ์กับ ownership

---

## 1. Members

| # | Name | Student ID | GitHub Username | Main Responsibility |
|---|---|---|---|---|
| 1 | นายปภังกร มงคลนรกิจ | 670710293 | `@670710293` | Concept + Short Code Illustration (สรุปแนวคิดหลัก + โค้ดตัวอย่างสั้น) |
| 2 | นางสาวศิริกานต์ หรุ่นมาบแค | 670710294 | `@670710294` | Detailed Code + Live Demo (โค้ดเชิงลึก + สาธิตสด) |
| 3 | นายสิรวิชญ์ เชี่ยวชาญ | 670710296 | `@670710296` | Rust vs Other Language + PPL Analysis (เปรียบเทียบภาษา + วิเคราะห์เชิง PPL) |
| 4 | นายวีรภัทร พุฒหอม | 670710336 | `@670710336` | Exercises + Common Mistakes + Challenge (แบบฝึกหัด + ข้อผิดพลาดที่พบบ่อย + คำถามท้าทาย) |

---

## 2. Learning Objectives

หลังจากศึกษา Topic นี้แล้ว ผู้เรียนสามารถ:

1. `[อธิบายแนวคิดสำคัญได้]`
2. `[เขียนโปรแกรม Rust ที่เกี่ยวข้องได้]`
3. `[วิเคราะห์พฤติกรรม/กฎของภาษาได้]`
4. `[เปรียบเทียบ Rust กับภาษาอื่นได้]`

---

## 3. Introduction

`Topic นี้จะพูดถึงการแบ่งย่อยในตัวแปรต่างๆ ไม่ว่าจะเป้น List , Array , String หรืออื่นๆ และการจัดการ Memory ของภาษา Rust`

`เริ่มที่การจัดการ Memory หากไม่มีการจัดการ Memory แบบ Rust`
1) [ตัวแปรข้อมูลจะไม่ปลอดภัย เนื่องจากใครๆก็มาหยิบไปใช้ได้ แก้ข้อมูลได้เสมอ]
2) [หากไม่มี memory Model ตัวข้อมูลจะมั่วซั่วไปหมด เก็บที่ไหนไปเรื่อย และจะทิ้งเป็น Garbage ไว้ใน Ram ซึ่งจะทำให้เปลืองทรัพยากรมากๆ]
3) [Slicing ช่วยให้ลดการจองพื้นที่ Ram แบบไม่จำเป็นทิ้งไป]  

`ส่วนในพาร์ทของ Memory Model ใน Rust จะมี Stack + Heap`  
`โดย Memory Model : ชุดการจัดเก็บข้อมูล/ตัวแปร การใช้งาน การคืนความจำ โดยใช้หลัก Ownership , Borrowing  และ Lifetime`
1) `Stack ใช้ Concept แบบ LIFO`  
`จะเก็บค่าตัวแปรที่รู้ขนาดแน่นอนเท่านั้น เช่น integer,อาเรย์ที่กำหนดขนาดแล้ว`  
2) `Heap ใช้เก็บค่าตัวแปรที่ไม่ทราบขนาดแน่นอน ( เพิ่มขึ้นหรือลดลงได้ตอน Compile )`  
` โดยหลักการมันจะเก็บตัว pointer กับชื่อของตัวแปรไว้ที่ Stack แล้วให้ชี้มาที่ Heap ของค่านั้นๆ `  
` จะเก็บตัวแปรประเภท String , Dynamic Collection เป็นต้น `  


`ส่วนตัวของ Slices คือการแบ่งย่อยจากตัวเซตข้อมูลหลัก ไม่ว่าจะเป็น List , Array หรือ String เป็นต้น`  
`ที่ช่วยทำให้เกิด Zero - Heap Allocation หรือก็คือ ไม่เกิดการจองข้อมูลใน Heap เพิ่มเติม`  
`(แต่ยังคงเก็บใน stack นิดนึงนะ คือชื่อกับ Pointer เพื่อบ่งบอกว่าเราชี้ไปที่ Heap จุดไหน)`  

`เช่น เรามี List หรือ Array ที่มีข้อมูลเป็น [2,3,5,6,7] แต่เราต้องการตั้งแต่ตัวที่ 3 ( คือเลข 5 ) เป็นต้นไป`  
`เพื่อนำไปคำนวณต่อ ก็สามารถใช้การ slices เพื่อตัดแค่ข้อมูลที่ต้องการ แล้วนำมาใช้ต่อได้เลย`  


---

## 4. Key Concepts

### 4.1 `[Memory Model ของ Stack]`

**คำอธิบาย**

`[Stack สำหรับเก็บค่าที่เป็น Imutable หรือค่าที่ถูกฟิคขนาดไว้แล้ว ในช่วงเวลานั้นๆ]`

**ตัวอย่าง**

```rust
fn main(){
    let x = 60;
    let y = 7;
    let z = x + y;
}
```

**Explanation**

`[ตัว Memory Model ของภาษา rust จะเก็บค่า x และ y เข้าไปใน stack ก่อน และให้มันมีค่าเป็น 60 และ 7 ตามลำดับ]`  
`[จากนั้น Memory ของภาษา rust จะ อ่านค่า x และ y นำมาบวกกันแล้วเก็บเข้าไปในค่า z ของ stack]`  


---

### 4.2 `[Memory Model ของ Heap]`

`[Heap จะเป็นก้อนเก็บข้อมูลก้อนนึง สำหรับตัวแปรที่ยืดหยุ่นเรื่องขนาดระหว่างการคอมไพล์ โดยตัวแปรเหล่านั้นจะมีทั้งเก็บค่าไว้ที่ stack และ heap]`
`[โดยหลักๆจะแบ่งเป็น 2 ประเภท 1.เก็บค่าใน stack เป็น thin pointer , 2.เก็บค่าใน stack เป็น fat pointer เพื่อชี้ข้อมูลไปที่ ก้อนใน heap]`

```rust
fn main(){
    let s = String::from("Hello");
    let myVec = vec![1, 2, 3, 5];
}
```
**Explanation**

`เริ่มที่ตัว s จะสร้างก้อนheap ที่เก็บคำว่า ['H' , 'e' , 'l' , 'l' , 'o']ไว้ แล้วจะเก็บค่าใน stack เป็น pointer + len + capacity `
`โดย pointer จะชี้ไปที่ก้อน heap`  
`ส่วน myVec ก็จะทำงานในทำนองเดียวกัน`  

---

### 4.3 `Slicing`

`[การ Slice จะไม่ใช่การแย่ง owner หรือการแก้ค่าแต่อย่างใด แต่มันแค่การขออ่านค่าตรงตำแหน่งที่ต้องการพอดี]`  
`[และอ่านไปเรื่อยๆตามจำนวนที่ขอ เช่น &s[1..3] แปลว่าจะขออ่านค่า ตั้งแต่ตัวที่ 1 จนถึงตัวที่ 2 ซึ่งมีความยาว = 2]`

```rust
fn main() {
    let s = String::from(“Silpakorn”);
    
    // แบบที่ 1 จะได้ตัวเอง
    let same = &s[..];
    println!("Slices ได้ตัวเอง จะได้ {same}");

    // ควรระวัง หากกำหนดเอง เนื่องจากการ Slice จะ Slice ถึงแค่ n-1
    // แบบที่ 2 ตั้งแต่ตัวแรกถึงตัวที่เรากำหนด
    let silp = &s[..3];
    println!("Slices ถึงตัวที่ 3 จะได้ {silp}");

    // 3 ตั้งแต่ตัวที่ i ถึงตัวที่ n ซึ่ง i กับ n เราสามารถกำหนดเองได้
    let pako = &s[3..7];
    println!("Slices ตั้งแต่ตัวที่ 3 - 7 จะได้ {pako}");

    // 4 ตั้งแต่ตัวที่เรากำหนดเป็นต้นไป
    let rn = &s[7..];
    println!("Slices ตั้งแต่ตัวที่ 7 เป็นต้นไป จะได้ {rn}");

}
```
### 4.4 `Mutable Slicing`

`[การ Mutable Slice จะเป็นการอนุญาตให้ตัวที่ยืมไปแก้ไขค่าได้]`  
`[เมื่อแก้ไขค่าเสร็จ ตัวค่าที่แก้ไขจะถูกเปลี่ยนตามใน index ที่ slice ไปเท่านั้น]`

```rust
fn main() {
    // Vec: ยืมเฉพาะช่วง index 1..3 แล้วแก้ค่าใน slice
    let mut numbers = vec![10, 20, 30, 40];
    let part: &mut [i32] = &mut numbers[1..3];
    part[0] = 1;
    part[1] = -20;
    println!("Vec: {numbers:?}"); // [10, 1, -20, 40]

    // mutable str เปลี่ยนจำนวนไบต์หรือเพิ่ม/ลบตัวอักษรไม่ได้
    let mut text = String::from("hello");
    let part: &mut str = &mut text[1..4];
    part.make_ascii_uppercase();
    println!("String: {text}"); // hELLo
}
```

### 4.5 `&str vs .clone()`

`[&str : การยืมข้อความจากตัวแปรหลัก ( s )  โดยการใช้ ptr ชี้ไปยังตัวแปรนั้นๆ ( ใช้กับจำพวก slice ) ] `  
`[.clone() : สร้างก้อน String บน heap ขึ้นมาใหม่ พร้อมคัดลอกข้อความจากตัวแปรที่ต้องการไปยังก้อน heap ใหม่ ] `  

```rust
fn main() {
    let s = String::from("Hello");
    let borrowed: &str = &s;
    let copied = s.clone();

    println!("s         = {s}");        // จะได้ Hello โดยที่อยู่ Heap จะเป็นของ s 
    println!("borrowed  = {borrowed}"); // จะได้ Hello โดยที่อยู่ Heap จะเป็นของ s
    println!("copied    = {copied}");   // จะได้ Hello โดยที่อยู่ Heap จะเป็นของ Copied เอง
}
```

## 5. Important Syntax / Rules

| Syntax / Rule | Meaning | Example |
|---|---|---|
| `String::from(...)` | `การเรียก associated function from ของชนิด String เพื่อ สร้าง String (owned, อยู่บน heap) จากข้อมูลอื่น` | `let s = String::from("Hello");` |
| `&data[a..b]/[..n]/[n..]` | `ตัดช่วงโดยไม่ copy` | `&arr[1..4]` |
| `clone()` | `copy ข้อมูลบน heap จริงๆ ได้เจ้าของใหม่อีกก้อน` | `let b = s.clone();` |
| `as_ptr()` | `ดึงที่อยู่ (Pointer) ในหน่วยความจำ` | `data.as_ptr()` |
| `len()` | `หาขนาดหรือความยาวของข้อมูล` | `data.len()` ||
| `as_bytes()` | `เมธอดของ str ที่ มองสตริงเป็น byte slice (&[u8]) โดยไม่ copy ข้อมูล` | `let bytes = s.as_bytes();` |
| `chars().count()` | `ใช้นับจำนวนอักขระ` | `th.chars().count()` |
| `is_char_boundary(i)` | `เช็คว่าตำแหน่ง byte นี้เป็นจุดเริ่มของตัวอักษรไหม` | `th.is_char_boundary(3)` |
| `split_at_mut(mid)` | `แบ่งเป็นสองส่วนที่ไม่ทับกันให้ขอ &mut พร้อมกันได้` | `let (l, r) = a.split_at_mut(3);` |
| `iter_mut()` | `แก้ค่าทีละตัว` | `for x in s.iter_mut() { *x += 1 }` |

### Important Rules

1. `Slice ต้องชี้ไปที่ข้อมูลที่ใช้งานได้จริงเสมอ`
2. `Slice แค่ borrow ไม่ได้เป็นเจ้าของ`
3. `กับข้อมูลชิ้นหนึ่ง จะมี mutable slice (&mut) ได้ 1 ตัว หรือ immutable slice (&) กี่ตัวก็ได้ แต่ ห้ามมีทั้งสองแบบพร้อมกัน`
4. `Slice ต้อง อยู่ไม่นานกว่า ข้อมูลที่มันอ้างอิง`
5. `ทุกค่าใน Rust มี เจ้าของ (owner) เสมอ`
6. `ณ เวลาหนึ่งมี เจ้าของได้แค่ 1 ตัวเท่านั้น`
7. `เมื่อเจ้าของ จบ scope (ออกจาก { }) ค่านั้นจะถูก drop คือคืนหน่วยความจำให้อัตโนมัติ`
8. `Array ขนาดคงที่อยู่ stack, Vec เก็บข้อมูลบน heap`
9. `String แก้ไข/ขยายได้, &str ยืมดูอย่างเดียว`
10. `String slice ใช้ตำแหน่ง byte ไม่ใช่ char`

---

## 6. Runnable Code Examples

> **ข้อกำหนด:** Code ทุกตัวต้อง Compile และ Run ได้จริงก่อนนำมาใส่ในเอกสาร

### Example 1 — `Slice is "Fat Pointer"`

**Purpose:** `สาธิตว่า slice (&[T]) คือ fat pointer ที่เก็บค่า 2 อย่างคือ address ของข้อมูล (ptr) และ จำนวนสมาชิก (len) ไม่ใช่ pointer ธรรมดา`

```rust
pub fn show_fat_pointer<T: std::fmt::Debug>(data: &[T]) {
    let ptr = data.as_ptr();
    let len = data.len();

    println!("ptr : {:p} | len : {}", ptr, len);
    println!("size of &[T] : {} bytes", std::mem::size_of::<&[T]>());
    println!("size of T : {} bytes", std::mem::size_of::<T>());

    let s1 = &data[..data.len() - 2];
    println!("slice ptr : {:p} | len : {}", s1.as_ptr(), s1.len());
    println!("{:?}", s1);
}

fn main() {
    let arr = [1, 2, 3, 4, 5, 6];
    let vec = vec![1, 2, 3, 4, 5, 6];

    println!("--- array ---");
    println!("{:?}", arr);
    show_fat_pointer(&arr);

    println!("--- vector ---");
    println!("{:?}", vec);
    show_fat_pointer(&vec);
}
```

**Expected Output**

```text
--- array ---
[1, 2, 3, 4, 5, 6]
ptr : 0x78442ff7c0 | len : 6
size of &[T] : 16 bytes
size of T : 4 bytes
slice ptr : 0x78442ff7c0 | len : 4
[1, 2, 3, 4]
--- vector ---
[1, 2, 3, 4, 5, 6]
ptr : 0x274cf800830 | len : 6
size of &[T] : 16 bytes
size of T : 4 bytes
slice ptr : 0x274cf800830 | len : 4
[1, 2, 3, 4]
```

**Explanation**

`&[T] (slice) ไม่ได้เก็บแค่ที่อยู่ในหน่วยความจำ แต่เก็บ 2 อย่างคู่กัน:
ptr คือที่อยู่ของสมาชิกตัวแรก
len คือจำนวนสมาชิก
เรียกว่า fat pointer เพราะใหญ่กว่า pointer ธรรมดาที่เก็บแค่ที่อยู่อย่างเดียว (thin pointer)`

`data.as_ptr() และ data.len() ดึง ptr กับ len ออกมาจาก slice`

`size_of::<&[T]>() จะได้ 16 bytes บนเครื่อง 64-bit (pointer 8 + len 8) ซึ่งเป็น fat pointer`

`size_of::<T>() คือขนาดสมาชิก 1 ตัว (ในที่นี้ i32 = 4 bytes)`

`ข้อควรระวัง
fatptr.len - 2 ถ้า slice ยาวน้อยกว่า 2 จะเกิด underflow (panic ใน debug mode) ถ้าจะใช้จริงควรเช็กความยาวก่อน
from_raw_parts เป็น unsafe เพราะ Rust ตรวจให้ไม่ได้ว่า ptr/len ถูกต้อง ในโค้ดนี้ปลอดภัยเพราะเราตัดให้สั้นลงเท่านั้น`

---

### Example 2 — `String and &str`

**Purpose:** `String/&str ใน Rust เป็น UTF-8 และ slice ด้วย byte index ไม่ใช่ตัวอักษร ส่วนท้ายเทียบ ownership ระหว่าง String กับ &str`

```rust
// pub fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;}}
//     s.len()}

pub fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];}}
    &s[..]}

fn main() {
    let mut string = String::from("Silpakorn University");
    let literal: &'static str = "Silpakorn";

    let word = first_word(&string);
    println!("word from String : {word}");
    let word = first_word(literal);
    println!("word from literal : {word}");
    string.clear();
    println!("string after clear : {:?}", string);

    let th = String::from("รองเท้าเนรคุณ");
        println!("---------- {} ----------", th);
    println!("len : {} bytes", th.len());
    println!("char count : {} chars", th.chars().count());
    //println!("first letter : {}", &th[0..1]); // error

    println!("---------- is char boundary ----------");
    println!("1 byte : {}", th.is_char_boundary(1));
    println!("3 bytes : {}", th.is_char_boundary(3));
    println!("first letter : {}", &th[0..3]);

    println!("---------- fat pointer ownership ----------");
    let mut s1 = String::from("ญี่ปุ่น");
    let s2 = "มาแล้ว";
    s1.push_str(s2);
    println!("{}", s1);
    println!("s2 is {s2}");
}
```

**Expected Output**

```text
word from String : Silpakorn
word from literal : Silpakorn
string after clear : ""
---------- รองเท้าเนรคุณ ----------
len : 39 bytes
char count : 13 chars
---------- is char boundary ----------
1 byte : false
3 bytes : true
first letter : ร
---------- fat pointer ownership ----------
ญี่ปุ่นมาแล้ว
s2 is มาแล้ว
```

**Explanation**

`(&String -> usize) คืนแค่ตำแหน่ง index ของช่องว่างแรก ปัญหาคือ index นี้ไม่ผูกกับสตริงเลย ถ้าเรียก string.clear() ทีหลัง index ก็ยังอยู่แต่ไม่มีความหมายแล้ว`

`(&str -> &str) คืน slice ที่ชี้เข้าไปในสตริงเดิม ทำให้ borrow checker รู้ว่าผลลัพธ์ยังยืมสตริงอยู่ ถ้าสตริงต้นทางถูกแก้ไขหรือล้างขณะที่ slice ยังถูกใช้ จะ compile ไม่ผ่าน`

`&str ทุกตัวการันตีว่าเป็น UTF-8 ซึ่งภาษาอังกฤษ 1 ตัว = 1 byte จึงเท่ากันหมด และตัด [0..1] ได้ตัวแรกพอดี`

`ภาษาไทยตัวแรกกินที่ 3 bytes การใช้ &th[0..1] จะ error เพราะตัดกลางตัว byte ที่ไม่สมบูรณ์`

`is_char_boundary ใช้ถามว่า "ตำแหน่ง byte นี้เป็นจุดเริ่มของตัวอักษรไหม"`

`s1 เป็น String คือ เป็นเจ้าของ ข้อมูล แก้ไข/ขยายได้`

`s2 เป็น &str คือ fat pointer (ptr + len) แค่ "ยืมดู" ข้อมูล เป็นเจ้าของไม่ได้`

`push_str แค่ยืม s2 มา copy ข้อความเข้า heap ของ s1 จึงไม่ได้เอา s2 ไป และยังใช้ s2 ต่อได้`

---

### Example 3 — `Mutable Slices`

**Purpose:** `mutable slice (&mut [T]) แก้ข้อมูลต้นทางได้โดยตรง และ กฎ borrow ของ Rust ห้ามมี &mut ซ้อนกันบนข้อมูลก้อนเดียว พร้อมวิธีแก้ด้วย split_at_mut`

```rust
pub fn transform_even_odd(slice: &mut [i32]) {
    for x in slice.iter_mut() {
        if *x % 2 == 0 {
            *x *= 2;
        } else {
            *x -= 1;
}}}

fn main() {
    let mut num = [1, 2, 3, 4, 5, 6];
    println!("before:  {:?}", num);
    transform_even_odd(&mut num);
    println!("after: {:?}", num);

    // let s1 = &mut num[..2];
    // let s2 = &mut num[1..];   // error

    // transform_even_odd(s1);
    // transform_even_odd(s2);

    let (left, right) = num.split_at_mut(3);
    // println!("numbers : {:?}", num); // error
    left[0] = 111;
    right[0] = 999;
    println!("left : {:?}", left);
    println!("right : {:?}", right);
    // println!("numbers : {:?}", num);
}
```

**Expected Output**

```text
before:  [1, 2, 3, 4, 5, 6]
after: [0, 4, 2, 8, 4, 12]
left : [111, 4, 2]
right : [999, 4, 12]
```

**Explanation**

`iter_mut() ให้ &mut i32 ของแต่ละตัว`

`*x คือการ "ตามไปที่ค่าจริง" เพื่ออ่าน/แก้`

`ฟังก์ชันรับ &mut [i32] จึงแก้ข้อมูลของคนเรียกได้โดยตรง ไม่ต้อง return`

`ต้องประกาศ let mut ไม่งั้นแก้ไม่ได้`

`&mut numbers (ชนิด &mut [i32; 6]) ถูกแปลงเป็น &mut [i32] ให้อัตโนมัติ`

`s1 กับ s2 ถูกใช้ต่อหลังจากนั้นทั้งคู่ Rust จึงไม่ยอมให้คอมไพล์`

`หมายเหตุ: จริงๆ Rust ไม่ได้ดูว่า range ทับกันจริงหรือเปล่า แค่เห็นว่า numbers[..] ถูก &mut ยืมซ้ำก็ error แล้ว แม้ range จะไม่ทับกัน (เช่น [..2] กับ [3..]) ก็ยัง error เพราะมันตรวจ range ตอนคอมไพล์ไม่ได้`

`ใช้ split_at_mut แบ่ง slice เป็น 2 ส่วนที่ ไม่ทับกันแน่นอน`

`ตราบใดที่ left/right ยังถูกใช้อยู่ข้างล่าง numbers ถือว่า ถูกยืมแบบ mutable อยู่ จะอ่าน numbers ตรงๆ ไม่ได้`

---

### Example 4 — `Slice methods`

**Purpose:** `เมธอดใช้งานกับ slice ได้ มีทั้งการดู/แบ่งข้อมูลแบบไม่ copy (zero-copy view) และการแปลงเป็นข้อมูลที่เป็นเจ้าของ (to_vec)`

```rust
fn main() {
    let nums = [1, 2, 3, 4, 5, 6, 7];
    
    match &nums[..] {
        [first, .., last] => println!("first / last : {first} / {last}"),
        [only] => println!("only one: {only}"),
        [] => println!("empty"),
    }
    // println!("first / last : {:?} / {:?}", nums.first(), nums.last());
    match &nums[..] {
        [first, rest @ ..] => println!("first / rest : {first} / {rest:?}"),
        [] => {}
    }
    println!(".get(10) : {:?}", nums.get(10));
    println!(".split_at(3) : {:?}", nums.split_at(3));
    println!(".chunks(3) : {:?}", nums.chunks(3).collect::<Vec<_>>());
    println!(".windows(3) : {:?}", nums.windows(3).collect::<Vec<_>>());
    println!(".contains(&4) : {}", nums.contains(&4));
 
    let owned: Vec<i32> = nums[..3].to_vec();
    println!("to_vec : {:?}", owned);
}
```

**Expected Output**

```text
first / last : 1 / 7
first / rest : 1 / [2, 3, 4, 5, 6, 7]
.get(10) : None
.split_at(3) : ([1, 2, 3], [4, 5, 6, 7])
.chunks(3) : [[1, 2, 3], [4, 5, 6], [7]]
.windows(3) : [[1, 2, 3], [2, 3, 4], [3, 4, 5], [4, 5, 6], [5, 6, 7]]
.contains(&4) : true
to_vec : [1, 2, 3]
```

**Explanation**

`first / last/ rest หาข้อมูลตัวแรก/สุดท้ายในอาเรย์/ตัวที่เหลือ (ถ้า slice ว่างจะได้ None)`

`get(x) index เกินขอบเขตได้ None ไม่ panic (ต่างจาก nums[10] ที่ panic)`

`split_at(x) แบ่งเป็น 2 slice ที่ index x คืนเป็น tuple (ไม่ copy)`

`chunks(x) ตัดเป็นท่อนละ x ไม่ซ้อนกัน ท่อนสุดท้ายเหลือเท่าไรก็เท่านั้น`

`windows(x) "หน้าต่าง" ขนาด x เลื่อนทีละ 1 ซ้อนกัน`

`contains(&x) เช็กว่ามีค่านี้อยู่ไหม รับ reference และวนหาแบบเรียงทีละตัว`

`to_vec() คัดลอก ไปสร้าง Vec ใหม่บน heap ที่เป็นเจ้าของข้อมูลเอง`

`เมธอดส่วนใหญ่ (first, get, split_at, chunks, windows) คืน slice/reference = ยืมข้อมูลเดิม ไม่ copy`

`มีแค่ to_vec() ที่ copy จริง`

---

## 7. Common Mistakes

### Mistake 1 — `[borrow of moved value: `s1`]`

**Problem**

`ผิดเพราะ let s2 = s1 เป็นการ ย้าย ownership (Move) จาก s1 ไปให้ s2 ทำให้ s1 หมดสิทธิ์ใช้งาน จึงไม่สามารถ println!("{}", s1) ได้อีก`

**Incorrect Code**

```rust
fn main() {
    let s1 = String::from("hello"); // heap-allocated
    let s2 = s1; // move, s1 ใช้ไม่ได้อีก
    println!("{}", s1); 
}
```

**Correct Code**

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    let s2 = &s1;
```

**Why?**

`เลือกใช้ .clone() หรือยืมค่าด้วย &s1 แทนการเขียนวิธีแก้รวมกันหมดเพื่อไม่ให้เกิดการประกาศตัวแปร s2 ซ้ำซ้อน`

---

### Mistake 2 — `[Range Out of Bounds]`

**Problem**

`ผิดเพราะ let s2 = s1 เป็นการ ย้าย ownership (Move) จาก s1 ไปให้ s2 ทำให้ s1 หมดสิทธิ์ใช้งาน จึงไม่สามารถ println!("{}", s1) ได้อีก`

**Incorrect Code**

```rust
fn main() {
    let numbers = [10, 20, 30];

    let x = &numbers[1..4]; 
}
```

**Correct Code**

```rust
fn main() {
    let numbers = [10, 20, 30];

    let x = &numbers[0..3]; 
}
```

**Why?**

`กำหนดข้อมูล หรือเลือกใช้ข้อมูล ให้ไม่เกินขอบเขต`

---

### Mistake 3 — `[Cannot assign to data in an immutable reference]`

**Problem**

`ใน Rust ถ้าเราใช้ & เพื่อยืมข้อมูล จะเป็นการ ยืมแบบอ่านอย่างเดียว
ดังนั้นเราจะไม่สามารถเปลี่ยนค่าข้างใน Slice ได้`

**Incorrect Code**

```rust
fn main() {
    let numbers = [10, 20, 30];

    let x = &numbers[0..2];

    x[0] = 100;
}
```

**Correct Code**

```rust
fn main() {
    let numbers = [10, 20, 30];

    let x = &mut numbers[0..2];

    x[0] = 100;
}
```

**Why?**

`ใน Rust ถ้าเราใช้ & เพื่อยืมข้อมูล จะเป็นการ ยืมแบบอ่านอย่างเดียว ดังนั้นเราจะไม่สามารถเปลี่ยนค่าข้างใน Slice ได้
ถ้าต้องการแก้ไขข้อมูล เราต้องใช้ &mut และประกาศข้อมูลด้วย mut เพื่อให้สามารถแก้ไขค่าได้`

---

### Mistake 4 — `[cannot borrow `numbers` as mutable because it is also borrowed as immutable]`

**Problem**

`พยายามแก้ไขหรือเพิ่มข้อมูลลงใน Vector  ในขณะที่ยังมีตัวแปรยืมอ่านข้อมูลใน Vector นั้นอยู่`

**Incorrect Code**

```rust
fn main() {
    let mut numbers = vec![1, 2, 3];

    for num in &numbers { // ยืมอ่านแบบ Immutable Borrow
        if  num == 2 {
            numbers.push(4); //Error: cannot borrow `numbers` as mutable
        }
    }
}

```

**Correct Code**

```rust
fn main() {
    let mut numbers = vec![1, 2, 3];
    let len = numbers.len(); ก่อน

    for i in 0..len {
        if numbers[i] == 2 {
            numbers.push(4);
        }
    }

    println!("{:?}", numbers); 
}

```

**Why?**

`โค้ดแบบแรกพังเพราะ for num in &numbers เป็นการ ยืมอ่านค้างไว้ตลอดการวน Loop ทำให้ Vector ถูกล็อคไม่ให้แก้ไข ถ้าสั่ง .push() แล้ว Vector ต้องขยายพื้นที่บน Heap ตัวแปร num ที่ชี้อ่านอยู่จะกลายเป็น Pointer ชี้ไปที่ขยะ  ทันที
เปลี่ยนมาใช้ for i in 0..len ซึ่งเป็นการ วน Loop ตามลำดับตัวเลข (0, 1, 2) แทน ไ	ม่ได้ยืมอ่าน Vector ค้างไว้ การอ่าน numbers[i] เกิดขึ้นและจบลงทันทีในบรรทัดนั้น บรรทัด numbers.push(4) จึงขอสิทธิ์แก้ไขได้อย่างปลอดภัย
`

---

## 8. Exercises

> จัดทำแบบฝึกหัด **2 ข้อ** ที่สอดคล้องกับ Topic และมีระดับความยากเหมาะสม

### Exercise 1 — `[Rust_Compro_01]`

**Problem**

`[กำหนดให้ numbers มีค่า [10, 20, 30, 40, 50] จงใช้ Slice เพื่อเลือกข้อมูล 20, 30, 40 และเปลี่ยนค่าเป็น 200, 300, 400 จากนั้นแสดงผลข้อมูลที่ถูกเปลี่ยนแปลงแล้ว โดยห้ามเปลี่ยนค่าใน numbers โดยตรง ให้แก้ไขข้อมูลผ่าน Slice เท่านั้น]`

**Hint**

`[ให้ใช้ Slice เพื่อเลือกเฉพาะข้อมูล 20, 30, 40 จาก numbers โดยกำหนดช่วงให้ถูกต้อง จากนั้นใช้การยืมแบบ Mutable เพื่อให้สามารถแก้ไขค่าผ่าน Slice ได้ และแสดงผล]`

**Solution**

```rust
fn main() {
    let mut numbers = [10, 20, 30, 40, 50];

    let x = &mut numbers[1..4];

    x[0] = 200;
    x[1] = 300;
    x[2] = 400;

    println!("{:?}", x);
}
```

**Explanation**

`1.สร้าง Mutable Slice อ้างอิง index ที่ 1 ถึง 3 (ได้แก่ [20, 30, 40])// สร้าง Mutable Slice อ้างอิง index ที่ 1 ถึง 3 (ได้แก่ [20, 30, 40])
 2.แก้ไขค่าตำแหน่งแรกของ Slice (คือ 20 ใน numbers)
 3.แก้ไขค่าตำแหน่งที่สองของ Slice (คือ 30 ใน numbers)
 4.แก้ไขค่าตำแหน่งที่สามของ Slice (คือ 40 ใน numbers)`

---

### Exercise 2 — `[Rust_Compro_02]`

**Problem**

[จงสร้าง Vector เก็บชุดตัวเลขจำนวนเต็ม (i32) ขนาด N ตัว (เช่น [1, 2, 3, 4, 5]) จากนั้นให้ทำการตรวจเช็คข้อมูลตัวเลขเดิมใน Vector:
ถ้าเจอ เลขคู่ ให้ทำการเพิ่ม (Push) ค่า x *2 ต่อท้ายเข้าไปใน Vector
ถ้าเจอ เลขคี่ ให้ทำการเพิ่ม (Push) ค่า x / 2 ต่อท้ายเข้าไปใน Vector
เงื่อนไข:ต้องตรวจเช็คครบเฉพาะ N ตัวแรกเท่านั้น

จากโจทย์ถ้าต้องการให้ Compile ผ่าน ตัวเลือกข้อไหนถูกต้อง

A. for val in &numbers[..len] { if val % 2 == 0 { numbers.push(val * 2); } else { numbers.push(val / 2); } }

B. let slice_vals = numbers[..len].to_vec(); for val in slice_vals { if val % 2 == 0 { numbers.push(val * 2); } else { numbers.push(val / 2); } }

C. for i in 0..numbers.as_slice().len() { let val = &mut numbers[i]; if *val % 2 == 0 { numbers.push(*val * 2); } else { numbers.push(*val / 2); } }

D. for i in 0..numbers.len() { let val = numbers[i]; if val % 2 == 0 { numbers.push(val * 2); } else { numbers.push(val / 2); } }]

**Hint**

`[ห้ามยืมอ่านข้อมูลค้างไว้ทั้ง Loop แล้วสั่งเพิ่มขนาด Vector พร้อมกัน]`

**Solution**

```rust
fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let len = numbers.len();

    let slice_vals = numbers[..len].to_vec();
    for val in slice_vals {
        if val % 2 == 0 {
            numbers.push(val * 2);
        } else {
            numbers.push(val / 2);
        }
    }

    println!("{:?}", numbers);
}
```

**Explanation**

`[คัดลอกข้อมูลช่วง Slice ออกมาเป็น Vector ใหม่ด้วย .to_vec() เพื่อแยกหน่วยความจำออกจากกัน ทำให้สามารถอ่านค่าไปพร้อมกับแก้ไข numbers ได้โดยไม่ขัดต่อกฎ Borrow Checker ของ Rust ครับ]`

---

## 9. PPL Perspective

> **ส่วนนี้เป็นหัวใจของรายวิชา Principles of Programming Languages**

วิเคราะห์ Topic นี้ในมุมมองของ Programming Languages

### 9.1 Syntax

Rust ใช้ slice เพื่ออ้างอิงข้อมูลบางส่วนของ collection โดยไม่ต้องสร้าง collection ใหม่ขึ้นมา

รูปแบบที่พบบ่อยคือ

```rust
&collection[start..end]
```
หรือถ้าเริ่มจากตำแหน่งแรก
```rust
&collection[..end]
```

ตัวอย่าง
```rust
let numbers = [10, 20, 30, 40, 50];

let part = &numbers[1..4];

println!("{:?}", part);
```
ผลที่ได้คือ
```rust
[20, 30, 40]
```
สำหรับ String สามารถใช้
```rust
let text = String::from("Hello Rust");

let word = &text[0..5];
```
โดย `word` จะมีชนิดเป็น `&str` ซึ่งเป็น string slice

สำหรับ `Vec`
```rust
let numbers = vec![10, 20, 30, 40, 50];

let part = &numbers[1..4];
```
`part` จะเป็น slice ที่อ้างอิงข้อมูลบางส่วนของ `Vec` โดยไม่ได้สร้าง `Vec` ใหม่

ดังนั้น Syntax ของ Slice จะเน้นการใช้ `&` ร่วมกับช่วง `[start..end]` เพื่อสร้าง reference ไปยังข้อมูลเดิม

### 9.2 Semantics

Slice มีความหมายว่าเป็น ส่วนหนึ่งของข้อมูลเดิม ไม่ใช่ข้อมูลชุดใหม่

ตัวอย่าง

```rust
let data = [10, 20, 30, 40, 50];
let part = &data[1..4];
```
`part` หมายถึงข้อมูล `20, 30, 40` แต่ข้อมูลยังอยู่ใน `data` เหมือนเดิม

แนวคิดสำคัญคือ Slice เป็น borrowed reference จึงไม่ได้เป็นเจ้าของข้อมูล
```rust
let numbers = vec![10, 20, 30, 40];

let part = &numbers[1..3];
```
ในกรณีนี้ `part` ยืมข้อมูลจาก `numbers` มาใช้ชั่วคราว ดังนั้นไม่สามารถใช้ `part` หลังจากเจ้าของข้อมูลหมดอายุได้

อีกประเด็นหนึ่งคือการใช้ String slice ต้องระวังเรื่อง UTF-8 เพราะ `&str` ไม่ได้แบ่งข้อมูลตามตัวอักษรโดยตรง แต่ใช้ byte range
```rust
let text = String::from("Hello");
let part = &text[0..2];
```
กรณีนี้จะได้ `"He"` เพราะตัวอักษร ASCII ใช้ 1 byte ต่อตัว

แต่ String ที่มีภาษาไทยหรือตัวอักษร Unicode การตัด byte ผิดตำแหน่งอาจทำให้เกิด panic ได้ เพราะไม่ใช่ทุก byte position ที่เป็นขอบเขตของตัวอักษร

ดังนั้น Semantics ของ Slice คือ การอ้างอิงข้อมูลเดิมบางช่วงผ่าน borrowing โดยไม่โอน ownership

### 9.3 Type System

Rust มี Type System ที่ทำงานร่วมกับ Slice และ Ownership อย่างชัดเจน

ตัวอย่างชนิดที่เกี่ยวข้อง

```rust
let arr: [i32; 5] = [1, 2, 3, 4, 5];

let slice: &[i32] = &arr[1..4];

let text: &str = "Hello Rust";

let vec: Vec<i32> = vec![1, 2, 3, 4, 5];

let vec_slice: &[i32] = &vec[1..4];
```
ความแตกต่างที่สำคัญคือ `[i32; 5]` เป็น Array ที่มีขนาดแน่นอน `&[i32]` เป็น Slice Reference ซึ่งขนาดของข้อมูลที่อ้างอิงสามารถเปลี่ยนได้ตามช่วงที่เลือก `&str` เป็น String Slice ใช้สำหรับอ้างอิงข้อมูล String ที่เป็น UTF-8 และ `Vec<i32>` เป็น Collection ที่สามารถเพิ่มหรือลดจำนวนสมาชิกได้ และเป็นเจ้าของข้อมูลของตัวเอง

ดังนั้น Rust จึงใช้ Type System เพื่อแยกให้ชัดว่าอะไรเป็นเจ้าของข้อมูล และอะไรเป็นเพียง reference ที่ยืมข้อมูลมาใช้

### 9.4 Memory / Resource Management

Rust มีการจัดการ Memory โดยใช้แนวคิด Ownership, Borrowing และ Lifetime แทนการใช้ Garbage Collector แบบภาษาอย่าง Java

ตัวอย่าง

```rust
let numbers = vec![10, 20, 30, 40, 50];
let part = &numbers[1..4];
```
ในตัวอย่างนี้ `numbers` เป็นเจ้าของ `Vec` และข้อมูลของ `Vec` โดยทั่วไปจะเก็บอยู่บน Heap

ส่วนตัวแปร `part` เป็น slice reference ที่ใช้ชี้ไปยังข้อมูลบางส่วนของ `numbers`

สำหรับ Array ที่มีขนาดคงที่ เช่น 
```rust
let numbers = [1, 2, 3, 4, 5];
```
ข้อมูลมักจะถูกเก็บบน Stack เมื่อ local variable ถูกสร้างขึ้นแบบปกติ

แต่ `Vec` จะเก็บตัวข้อมูลไว้บน Heap และตัวแปร `Vec` บน Stack จะเก็บข้อมูลสำหรับจัดการ buffer เช่น pointer, length และ capacity

เมื่อออกจาก Scope Rust จะเรียก `drop` และคืนทรัพยากรที่เจ้าของข้อมูลรับผิดชอบโดยอัตโนมัติ

จุดสำคัญคือ Slice ไม่ได้เป็นเจ้าของข้อมูล
```rust
let data = vec![1, 2, 3, 4];
let part = &data[1..3];
```
เมื่อ `data` หมด Scope ข้อมูลที่ `part` อ้างอิงอยู่ก็ไม่สามารถถูกใช้งานต่อได้ เพราะ Rust ใช้ Borrow Checker ป้องกันไม่ให้เกิดการอ้างอิงข้อมูลที่หมดอายุแล้ว

### 9.5 Abstraction / Other PPL Concepts

แนวคิด Slice ของ Rust เชื่อมโยงกับ PPL หลายเรื่อง ได้แก่ Abstraction, Scope, Binding และ Lifetime

**Abstraction**

Slice ช่วยให้โปรแกรมเมอร์สนใจเฉพาะ “ข้อมูลช่วงที่ต้องการ” โดยไม่ต้องจัดการรายละเอียดการสร้าง collection ใหม่

```rust
fn print_data(data: &[i32]) {

    println!("{:?}", data);

}
```
ฟังก์ชันนี้สามารถรับได้ทั้ง
```rust
let arr = [1, 2, 3, 4];
let vec = vec![5, 6, 7, 8];
```
เพราะทั้ง Array และ Vector สามารถถูกยืมมาเป็น `&[i32]` ได้

จึงเป็นการสร้าง Abstraction ให้ฟังก์ชันทำงานกับ “ลำดับของข้อมูล” โดยไม่ต้องสนใจว่าเจ้าของข้อมูลจริง ๆ เป็น Array หรือ Vector

**Scope**

ตัวแปรและ reference จะใช้งานได้ภายใน Scope ที่กำหนด

```rust
let data = vec![1, 2, 3];

{
    let part = &data[0..2];
}
```
`part` จะมี Scope อยู่ภายใน block เท่านั้น

**Binding**

Rust ใช้ `let` สำหรับ Binding ตัวแปรกับค่า

```rust
let data = vec![1, 2, 3, 4];
let part = &data[1..3];
```
ในที่นี้ `part` ถูก Binding ให้เป็น reference ไปยังส่วนหนึ่งของ `data`

**Lifetime**

Slice ต้องไม่สามารถมีอายุยาวกว่า Owner ของข้อมูล

```rust
let data = vec![1, 2, 3];

let part = &data[0..2];
```
Lifetime ของ `part` จึงขึ้นอยู่กับ `data`

แนวคิดนี้ช่วยให้ Rust ตรวจสอบปัญหาเกี่ยวกับ Memory ตั้งแต่ Compile Time

### 9.6 Why Rust?

Rust ออกแบบมาให้สามารถควบคุม Memory ได้ดี แต่ในขณะเดียวกันก็พยายามป้องกันปัญหาที่เกิดจากการจัดการ Memory

สำหรับ Slice จุดเด่นคือสามารถ เข้าถึงข้อมูลบางส่วนโดยไม่ต้อง Copy ข้อมูลใหม่

ตัวอย่าง
```rust
let data = vec![10, 20, 30, 40, 50];

let part = &data[1..4];
```
`part` เพียงแค่อ้างอิงข้อมูลเดิม ทำให้ลดการสร้างข้อมูลซ้ำและช่วยเรื่อง Performance

ดังนั้น Rust ให้ความสำคัญกับ Memory Safety โดยใช้ Ownership, Borrowing และ Lifetime ในการจัดการหน่วยความจำ แนวคิดเหล่านี้ช่วยป้องกันปัญหาที่พบบ่อยในภาษาแบบ Manual Memory Management เช่น dangling reference คือ reference ที่ชี้ไปยังข้อมูลที่หมดอายุแล้ว, use-after-free คือการใช้ข้อมูลหลังจาก Memory ถูกคืนไปแล้ว, double free คือการคืน Memory เดิมมากกว่าหนึ่งครั้ง และ invalid memory access คือการเข้าถึง Memory ในตำแหน่งที่ไม่ถูกต้อง เช่น การเข้าถึง Array เกินขอบเขต

ในกรณีของ Slice จะเห็นว่า Slice ไม่ได้เป็นเจ้าของข้อมูล แต่เป็นการ Borrow ข้อมูลจาก Owner ดังนั้น Rust จึงตรวจสอบความสัมพันธ์ระหว่าง Slice กับข้อมูลต้นทางก่อน Compile เพื่อป้องกันการอ้างอิงข้อมูลที่ไม่มีอยู่แล้ว และช่วยให้โปรแกรมมีทั้ง Memory Safety และ Performance

---

## 10. Rust vs. Other Language

**Comparison Language:** Python / C / C++ / Java

| Aspect | Rust | Python | C | C++ | Java |
|---|---|---|---|---|---|
| Syntax | ใช้ `&[T]` สำหรับ slice และ `&str` สำหรับ string slice เช่น `&data[1..4]` | ใช้ slicing เช่น `data[1:4]` และ `text[1:4]` | ไม่มี slice type โดยตรง มักใช้ pointer ร่วมกับ length เช่น `int *p = &data[1]` | มี `std::span` และ `std::string_view` สำหรับมองข้อมูลบางช่วงโดยไม่เป็นเจ้าของ | Array ไม่มี slice โดยตรง มักใช้ `Arrays.copyOfRange()` ซึ่งสร้าง array ใหม่ หรือ `List.subList()` สำหรับ view |
| Semantics / Behavior | Slice เป็น borrowed reference ไปยังข้อมูลเดิม ไม่ได้สร้างข้อมูลใหม่ และไม่เป็นเจ้าของข้อมูล | Slice ของ `list`, `str` โดยทั่วไปได้ข้อมูลใหม่ ส่วนตัวแปรเดิมยังเป็นเจ้าของข้อมูลของตัวเอง | Pointer เพียงชี้ไปยังตำแหน่ง Memory โปรแกรมเมอร์ต้องจัดการว่า pointer และ length ถูกต้อง | `span` / `string_view` เป็น non-owning view จึงไม่เป็นเจ้าของข้อมูล ต้องระวัง lifetime ของข้อมูลต้นทาง | การ slice array แบบ `copyOfRange()` เป็นการ Copy ข้อมูล ส่วน `subList()` เป็น view ที่อ้างอิง List เดิม |
| Type System | Static + Strong typing มีชนิดชัดเจน เช่น `&[i32]`, `&str`, `Vec<i32>` | Dynamic typing ชนิดของตัวแปรถูกตรวจสอบตอน Runtime | Static typing แต่มี pointer และ implicit conversion หลายกรณี | Static + Strong typing มี pointer, reference, `span`, `string_view` | Static + Strong typing ไม่มี pointer arithmetic แบบ C/C++ |
| Memory Management | ใช้ Ownership, Borrowing และ Lifetime โดยไม่มี Garbage Collector | จัดการ Memory อัตโนมัติตาม implementation เช่น reference counting/garbage collection | Manual memory management เช่น `malloc()` / `free()` | ใช้ RAII, destructor และ smart pointers ช่วยจัดการ Memory แต่ยังสามารถใช้ raw pointer ได้ | ใช้ Garbage Collector เป็นหลัก Object และ Array อยู่บน Heap ส่วน local references/stack frames อยู่ใน Stack |
| Safety | เน้น Memory Safety ตั้งแต่ Compile Time เช่น ป้องกัน dangling reference และ use-after-free | มีการจัดการ Memory อัตโนมัติและตรวจสอบหลายอย่างตอน Runtime | Safety ต่ำกว่า เพราะสามารถเกิด dangling pointer, buffer overflow, use-after-free ได้ | ปลอดภัยกว่า C ในหลายด้านเมื่อใช้ RAII/modern C++ แต่ raw pointer และ lifetime ยังทำให้เกิดปัญหาได้ | มี bounds checking, ไม่มี pointer arithmetic และใช้ GC จึงลดปัญหา Memory บางประเภท |

### Rust Example

Slice ของ Vector
```rust
fn main() {
    let numbers = vec![10, 20, 30, 40, 50];

    let part = &numbers[1..4];

    println!("{:?}", part);
}
```
ผลลัพธ์
```rust
[20, 30, 40]
```
ตรงนี้ `part` ไม่ได้สร้าง `Vec` ใหม่ แต่เป็น Slice ที่ ยืมข้อมูลจาก `numbers`

ดังนั้น `numbers` เป็น Owner ส่วน `part` เป็น Borrower

### Rust `&str` Example
```rust
fn main() {
    let text = "Hello Rust";

    let part: &str = &text[0..5];

    println!("{}", part);
}
```
ผลลัพธ์
```rust
Hello
```
`&str` เป็น String Slice ที่ ไม่ได้เป็นเจ้าของ String แต่เป็น reference ไปยังข้อมูล String ที่มีอยู่แล้ว

จุดที่ต้องระวังคือ Rust String ใช้ UTF-8 ดังนั้น `&str` ใช้ byte range ไม่ใช่ตำแหน่งตัวอักษรแบบที่มองเห็น

### Python Example

Python มี slicing โดยตรงและเขียนง่ายมาก
```python
numbers = [10, 20, 30, 40, 50]

part = numbers[1:4]

print(part)
```
ผลลัพธ์
```python
[20, 30, 40]
```
แต่พฤติกรรมต่างจาก Rust เพราะการ slice list แบบนี้โดยทั่วไปจะสร้าง list ใหม่

จึงสามารถมองได้ประมาณว่า
```
numbers → [10,20,30,40,50]

part    → [20,30,40] (object ใหม่)
```
Python จึงใช้งานง่ายกว่า แต่ไม่มีแนวคิด Ownership/Borrowing แบบ Rust ที่ Compiler ตรวจสอบความสัมพันธ์เหล่านี้

### C Example

C ไม่มี Slice เป็น Type โดยตรง ดังนั้นมักใช้ Pointer + Length
```c
#include <stdio.h>

int main() {
    int numbers[] = {10, 20, 30, 40, 50};

    int *part = &numbers[1];
    int length = 3;

    for (int i = 0; i < length; i++) {
        printf("%d ", part[i]);
    }

    return 0;
}
```
ผลลัพธ์
```c
20 30 40
```
ตรงนี้
```c
int *part = &numbers[1];
int length = 3;
```
ต้องใช้ Pointer บอกว่าเริ่มตรงไหน และต้องใช้ length บอกว่ามีข้อมูลกี่ตัว

C ไม่ได้ตรวจสอบให้ว่า Pointer ยังถูกต้องหรือข้อมูลยังมีชีวิตอยู่

จึงมีโอกาสเกิดปัญหา เช่น 

- Dangling Pointer คือ Reference ที่ยังชี้ไปยังข้อมูลเดิม แต่ข้อมูลนั้นถูกทำลายหรือหมดอายุไปแล้ว
- Use-After-Free คือการ นำ Memory ที่ถูกคืนหรือถูกปล่อยไปแล้วกลับมาใช้งานอีก
- Invalid Memory Access คือการ เข้าถึง Memory ในตำแหน่งที่ไม่ควรเข้าถึง


### C++ Example

C++ มี std::span ซึ่งมีแนวคิดใกล้กับ Rust Slice มาก
```cpp
#include <iostream>
#include <vector>
#include <span>

int main() {
    std::vector<int> numbers = {10, 20, 30, 40, 50};

    std::span<int> part(numbers.data() + 1, 3);

    for (int x : part) {
        std::cout << x << " ";
    }

    return 0;
}
```
ผลลัพธ์
```cpp
20 30 40
```
std::span เป็น non-owning view หมายความว่าไม่ได้เป็นเจ้าของข้อมูล เช่นเดียวกับ Slice ของ Rust ในแง่แนวคิด

แต่ความแตกต่างสำคัญคือ C++ ไม่ได้มี Ownership/Borrow Checker แบบ Rust ดังนั้นโปรแกรมเมอร์ยังต้องระวังว่า numbers ต้องมีอายุยาวพอที่จะให้ part ใช้งาน

C++ ยังมี std::string_view สำหรับมองบางส่วนของ String โดยไม่ Copy ข้อมูล

### Java Example

Java ไม่มี Slice Type สำหรับ Array โดยตรง

ตัวอย่าง
```java
import java.util.Arrays;

public class Main {
    public static void main(String[] args) {
        int[] numbers = {10, 20, 30, 40, 50};

        int[] part = Arrays.copyOfRange(numbers, 1, 4);

        System.out.println(Arrays.toString(part));
    }
}
```
ผลลัพธ์
```java
[20, 30, 40]
```
แต่ `Arrays.copyOfRange()` สร้าง Array ใหม่
```
numbers → [10,20,30,40,50]

part    → [20,30,40] (Array ใหม่)
```
จึงต่างจาก Rust
```rust
let part = &numbers[1..4];
```
ที่ part เป็น reference ไปยังข้อมูลเดิม

สำหรับ Collection อย่าง List Java สามารถใช้
```java
List<Integer> part = numbers.subList(1, 4);
```
ซึ่งมีลักษณะเป็น view ของ List เดิม มากกว่าการ Copy ทั้งชุด

### Analysis

ความแตกต่างที่สำคัญระหว่าง Rust กับภาษาอื่นอยู่ที่ แนวคิดในการจัดการ Memory และการออกแบบ Slice โดย Rust พยายามให้โปรแกรมสามารถเข้าถึงข้อมูลเดิมได้โดยไม่ต้อง Copy ข้อมูล แต่ยังคงตรวจสอบความปลอดภัยของการอ้างอิงตั้งแต่ Compile Time

Rust vs. Python

Python ใช้ Syntax ของ Slice ที่ง่าย เช่น `data[1:4]` และจัดการ Memory ให้อัตโนมัติ ทำให้เขียนโปรแกรมได้สะดวก แต่ไม่ได้บังคับให้โปรแกรมเมอร์ระบุ Ownership หรือความสัมพันธ์ของ Reference แบบ Rust การออกแบบของ Rust จึงเน้นให้ผู้เขียนโปรแกรมควบคุมการยืมข้อมูลได้ชัดเจนขึ้น เพื่อให้เกิด Memory Safety โดยไม่ต้องพึ่ง Garbage Collector

Rust vs. C

C ไม่มี Slice type โดยตรง และมักใช้ Pointer กับ Length แทน ทำให้ควบคุม Memory ได้ละเอียดและมี Overhead ต่ำ แต่โปรแกรมเมอร์ต้องรับผิดชอบเรื่อง Pointer, Lifetime และขอบเขตของข้อมูลเอง Rust ออกแบบมาเพื่อลดปัญหาเหล่านี้ โดยให้ Slice เช่น `&[T]` เป็น Borrowed Reference และใช้ Ownership กับ Borrow Checker ตรวจสอบว่าการอ้างอิงยังถูกต้องอยู่

Rust vs. C++

C++ มีแนวคิดที่ใกล้ Rust มากขึ้น เช่น `std::span` และ `std::string_view` ซึ่งใช้เป็นมุมมองไปยังข้อมูลเดิมโดยไม่ต้อง Copy แต่ C++ ยังเปิดให้ใช้ Pointer และจัดการ Lifetime ได้หลายรูปแบบ ขณะที่ Rust ออกแบบกฎ Ownership และ Lifetime ให้เป็นส่วนหนึ่งของ Type System และให้ Compiler ตรวจสอบความสัมพันธ์ของ Reference อย่างเข้มงวดกว่า

Rust vs. Java

Java เน้นการจัดการ Memory อัตโนมัติด้วย Garbage Collector และไม่มี Pointer Arithmetic แบบ C/C++ ทำให้การใช้งาน Memory ค่อนข้างง่าย แต่การออกแบบของ Rust เลือกใช้ Ownership และ Lifetime แทน Garbage Collector เพื่อให้สามารถควบคุมทรัพยากรได้ละเอียดและคาดเดาได้มากขึ้น โดยยังรักษา Memory Safety ไว้

**เหตุผลด้านการออกแบบภาษา**

แนวคิดของ Rust ในเรื่อง Slice ถูกออกแบบให้ตอบโจทย์ 2 อย่างพร้อมกัน คือ Performance และ Safety

ใน Rust Slice คือการอ้างอิงไปยังข้อมูลบางส่วนของข้อมูลเดิม โดยไม่จำเป็นต้องสร้างหรือ Copy ข้อมูลชุดใหม่ขึ้นมา ทำให้ประหยัด Memory และช่วยให้การทำงานมีประสิทธิภาพมากขึ้น อย่างไรก็ตาม Slice ไม่ได้เป็นเจ้าของข้อมูลนั้นเอง แต่เป็นเพียงการ Borrow ข้อมูลจากตัวแปรที่เป็น Owner ดังนั้น Rust จึงใช้แนวคิด Ownership, Borrowing และ Lifetime เพื่อควบคุมว่าข้อมูลสามารถถูกอ้างอิงและใช้งานได้นานแค่ไหน โดย Compiler จะตรวจสอบกฎเหล่านี้ตั้งแต่ Compile Time เพื่อป้องกันปัญหาที่เกี่ยวกับ Memory เช่น การอ้างอิงข้อมูลที่หมดอายุหรือการใช้ข้อมูลผิดช่วงเวลา

ดังนั้น จุดเด่นของ Rust ไม่ได้อยู่ที่การมี Slice เพียงอย่างเดียว แต่คือการออกแบบให้ การอ้างอิงข้อมูลที่มีประสิทธิภาพสามารถใช้งานร่วมกับระบบ Ownership ได้อย่างปลอดภัย ซึ่งเป็นแนวคิดที่แตกต่างจาก Python และ Java ที่เน้นการจัดการ Memory อัตโนมัติ และแตกต่างจาก C/C++ ที่ให้อิสระในการจัดการ Memory มากกว่า

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

`รับผิดชอบส่วน Concept + Short Code (3 - 4.5) , ทำสไลด์ในหน้า concept หลักๆ + มี short code เพื่อให้คนที่ยังไม่รุ้จักเข้าใจง่ายขึ้น, ใส่สิ่งที่ทุกคนจำเป็นต้องรู้จากเนื้อหาส่วนนี้หลักๆ`

**Member 2**

`รับผิดชอบการเขียนและอธิบายโค้ด Rust ในหัวข้อเพิ่มเติมเกี่ยวกับ fat pointer ใน slice, String และ &str slicing, mutable slice, และเมธอดอื่นๆที่เกี่ยวข้อง พร้อมสาธิตสด ทำไสลด์นำเสนอ และเขียน tutorial.md (5 - 6)`

**Member 3**

`รับผิดชอบหัวข้อ PPL Perspective (9.1–9.6), วิเคราะห์ Rust ในด้าน Syntax, Semantics, Type System, Memory/Resource Management, Abstraction และ Why Rust รวมถึงเปรียบเทียบ Rust กับ Python, C, C++ และ Java`

**Member 4**

`ที่รับผิดชอบส่วน Mistake + Exercise (7 - 8), ข้อผิดพลาดที่พบบ่อย และ วิธีการแก้ไขข้อผิดพลาด (4 หัวข้อย่อย), คำถามท้าทายผู้ฟัง และ เฉลย (2 ข้อ)`

---

## 12. References

1. The Rust Programming Language — Understanding Ownership, References and Borrowing, Slices
`https://doc.rust-lang.org/book/`
2. The Rust Reference — Slice Types & Reference Types
`https://doc.rust-lang.org/reference/types/slice.html`
3. Crate std - Primitive Type slice
`https://doc.rust-lang.org/std/primitive.slice.html`
4. Python Documentation — Common Sequence Operations / Slicing
`https://docs.python.org/3/library/stdtypes.html`
5. cppreference — C / C++ Arrays, Pointers and std::span
<br>`https://en.cppreference.com/w/c/language/array`<br>`https://en.cppreference.com/w/cpp/container/span`
6. Oracle Java Documentation — Arrays
`https://docs.oracle.com/javase/tutorial/java/nutsandbolts/arrays.html`
7. Kodekloud - The Slice Type
`https://notes.kodekloud.com/docs/Rust-Programming/Ownership/The-Slice-Type/page`
8. Kodekloud - Rules for Slices
`https://notes.kodekloud.com/docs/Rust-Programming/Ownership/Rules-for-Slices/page`
9. The Rust Programming Language - Storing UTF-8 Encoded Text with Strings
`https://doc.rust-lang.org/book/ch08-02-strings.html`

---

## 13. AI Usage Declaration

สามารถใช้ AI เป็นเครื่องมือช่วยเรียนรู้และพัฒนาได้ แต่สมาชิกทุกคนต้องเข้าใจและสามารถอธิบายผลงานของกลุ่มได้

| AI Tool | Purpose | How the Result Was Verified |
|---|---|---|
| `ChatGPT` | `ช่วยอธิบายและเรียบเรียงหัวข้อ Slices & Rust Memory Model, PPL Perspective 9.1–9.6 และช่วยจัดโครงสร้าง Presentation` | `ตรวจสอบกับ Rust Book, Rust Reference และทดลอง/ตรวจสอบตัวอย่าง Code ด้วยตนเอง` |
| `ChatGPT` | `ช่วยเปรียบเทียบ Rust กับ Python, C, C++ และ Java ในด้าน Syntax, Type System และ Memory Management` | `[ตรวจสอบแนวคิดและ Syntax กับเอกสาร Official ของแต่ละภาษา และตรวจสอบความถูกต้องของ Code` |
| `ChatGPT` | `ช่วยเรียบเรียงและอธิบายหัวข้อ Mistake และ Exercise ให้เข้าใจง่าย เพื่อให้ผู้ฟังเข้าใจเนื้อหาและตัวอย่าง Code ได้ง่ายขึ้น` | `นำคำอธิบายมาปรับให้เหมาะกับเนื้อหาที่นำเสนอ และทดลองรัน Code ด้วยตนเองเพื่อตรวจสอบความถูกต้อง` |
| `Claude AI` | `ช่วยในด้านการอธิบายส่วน code และลองให้ Preview ทั้งสไลด์ และส่วนโค้ดว่าตรงตาม Concept มั้ย, ` | `ตรวจสอบกับเนื้อหาในเอกสารทางการอย่าง Rust Book ส่วนโค้ดลองไปรันใน Rust online complier เพื่อที่จะตรวจสอบว่า เกิด error จริงมั้ย ได้ผลลัพธ์ตามที่ต้องการถูกมั้ย` |
| `Claude AI` | `ทำความเข้าใจโค้ด, หาความหมายของแต่ละ syntax, เรียบเรียงการใช้ภาษาเพื่ออธิบายการทำงาน, หาแนวทางการแก้ไขเมื่อเกิด error` | `ตรวจสอบกับแฟล่งอ้างอิง official, ทดสอบโปรแกรมว่ารันได้ถูกต้องหรือไม่` |
| `Gemini` | `ทำความเข้าใจโค้ด, แปลภาษา, หาความหมายของแต่ละ syntax, สรุปเนื้อหาที่ต้องศึกษา` | `ตรวจสอบกับแหล่งอ้างอิง official, ทดสอบโปรแกรมว่ารันได้ถูกต้องหรือไม่` |

### Declaration

- [x] Code ทุกส่วนที่นำเสนอได้รับการ Compile และทดสอบแล้ว
- [x] สมาชิกทุกคนสามารถอธิบาย Code ที่นำเสนอได้
- [x] ตรวจสอบข้อมูลจากแหล่งอ้างอิงที่น่าเชื่อถือแล้ว
- [x] ระบุการใช้ AI อย่างโปร่งใส

**รายละเอียดการใช้ AI**

สมาชิกใช้ AI ในหลายขั้นตอนของการทำงาน ได้แก่ ใช้ ChatGPT ช่วยอธิบายและเรียบเรียงหัวข้อ Slices & Rust Memory Model และ PPL Perspective 9.1–9.6 ช่วยจัดโครงสร้าง Presentation รวมถึงช่วยเปรียบเทียบ Rust กับ Python, C, C++ และ Java ในด้าน Syntax, Type System และ Memory Management นอกจากนี้ใช้ Claude AI ช่วยอธิบาย Code, ทดลองแนวทางแก้ไขข้อผิดพลาด และช่วยทำความเข้าใจ Syntax และแนวคิดต่าง ๆ ส่วน Gemini ใช้ช่วยแปลภาษา อธิบายความหมายของ Syntax และสรุปเนื้อหาที่ต้องศึกษา

หลังจากได้ผลลัพธ์จาก AI สมาชิกจะ ตรวจสอบข้อมูลกับเอกสารอ้างอิงที่เป็น Official เช่น Rust Book และ Rust Reference รวมถึงตรวจสอบ Syntax และแนวคิดกับเอกสารของแต่ละภาษา และ ทดลอง Compile / รัน Code ด้วยตนเอง เพื่อดูว่าผลลัพธ์ตรงกับคำอธิบายหรือไม่ จากนั้นจึงนำข้อมูลที่ตรวจสอบแล้วมาเรียบเรียงและปรับใช้ใน Presentation อีกครั้ง

---

## 14. GitHub Contribution

| Member | Issues | Commits | Pull Requests | Code Reviews | Contribution |
|---|---:|---:|---:|---:|---|
| Member 1 | `0` | `7` | `4` | `0` | `11` |
| Member 2 | `0` | `51` | `9` | `0` | `60` |
| Member 3 | `0` | `29` | `8` | `0` | `37` |
| Member 4 | `0` | `20` | `3` | `0` | `23` |

### Teamwork Reflection

**How did your team collaborate?**

`[เราเริ่มจากการแบ่งงาน และตรวจสอบหัวข้อที่ต้องรับผิดชอบ (Rust_Tutorial.md, Rust_Tutorial_Presentation)]`

`[จากนั้นก็ศึกษาข้อมูลและทำความเข้าใจเนื้อหาในแต่ละหัวข้อที่ได้รับมอบหมาย โดยหาข้อมูลจากเอกสารและแหล่งข้อมูลต่าง ๆ แล้วลองเขียนโค้ด Rust ด้วยตัวเอง เพื่อให้เข้าใจการทำงานและสามารถอธิบายเนื้อหาได้]`

`[หลังจากนั้นก็ตรวจสอบและทดลองรันโค้ดทั้งหมด เพื่อดูว่าโค้ดสามารถทำงานได้จริงหรือไม่ และตรวจสอบเนื้อหาว่าถูกต้องและตรงกับหัวข้อที่ได้รับหรือไม่]`

**Problems encountered**

`[ทำการ Pull Request แล้วขึ้น Conflict ทำให้ Merge ไม่ได้]`

**How did you solve them?**

`[เข้าไปแก้ไขตรงส่วนที่ทับซ้อน/ขัดแย้งกัน ตรง Conflict]`

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

**Repository:** `**github.com/670710296/rust-tutorial-2569/**`

**Chapter Path:** `13-slices-rust-memory-model/`

**Final PR:** `#39`

**Submitted by:** `[Group 13]`

**Date:** `[2569-10-04]`
