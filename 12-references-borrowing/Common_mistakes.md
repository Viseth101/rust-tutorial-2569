## 7. Common Mistakes

### Mistake 1 — `[Mutable and Immutable Reference Mixup]`

**Problem**

`การใช้งาน Mutable reference และ Immutable reference กับตัวแปรเดียวกันขณะที่ reference ก่อนหน้ายังถูกใช้งานอยู่ ซึ่ง Rust ไม่อนุญาตให้เกิดการ borrow ที่ขัดแย้งกัน`

**Incorrect Code**

```rust
// Incorrect example
fn main() {
    let mut num = vec![1, 2, 3];
    let first_number = &num[0]; // immutable borrow occurs here
    num.push(4); // mutable borrow occurs here
    println!("{}", first_number); // immutable borrow later used here
    //error[E0502]: cannot borrow `num` as mutable because it is also borrowed as immutable  
}
```

**Correct Code**

```rust
// Correct example
fn main() {
    let mut num = vec![1, 2, 3];
    num.push(4); 
    let first_number = &num[0]; // make an immutable reference after the mutable borrow has ended
    println!("{}", first_number); 
}

```

**Why?**

`Rust ไม่อนุญาตให้ immutable borrow และ mutable borrow ของข้อมูลเดียวกันเกิดขึ้นพร้อมกัน เพราะ mutable borrow สามารถแก้ไขข้อมูลได้ ในขณะที่ immutable borrow ต้องสามารถอ่านข้อมูลได้โดยไม่ต้องกังวลว่าข้อมูลจะถูกแก้ไขระหว่างที่ borrow ยังใช้งานอยู่

ในตัวอย่างแรก first_number เป็น immutable reference ที่ยังถูกใช้งานที่ println! หลังจากเรียก num.push(4) (mutable borrow) ดังนั้น borrow นี้ยังไม่สิ้นสุด จึงเกิด conflicting borrow 
เพราะ push อาจทำให้ Vec ทำการ reallocate หน่วยความจำ ซึ่งอาจทำให้ตำแหน่งของข้อมูลที่ first_number อ้างถึงเปลี่ยนไป และทำให้ reference นี้ไม่สามารถใช้งานได้อย่างปลอดภัย

ในตัวอย่างที่ถูกต้องได้ทำการเพิ่มค่าเข้า num ให้เสร็จก่อน หลังจาก mutable borrow สิ้นสุดลงจึงสามารถสร้าง immutable borrow first_number เพื่ออ่านข้อมูล `

---

### Mistake 2 — `[Multiple Mutable References]`

**Problem**

`การสร้าง Mutable reference มากกว่าหนึ่งตัวไปยังตัวแปรเดียวกันขณะที่ mutable reference ก่อนหน้ายังถูกใช้งานอยู่ ซึ่ง Rust ไม่อนุญาตให้เกิดการ borrow ที่ขัดแย้งกัน`

**Incorrect Code**

```rust
// Incorrect example
fn main() {
    let mut num = 10;

    let ref1 = &mut num; // first mutable borrow occurs here
    let ref2 = &mut num; // second mutable borrow occurs here
    // error[E0499]: cannot borrow `num` as mutable more than once at a time
    *ref1 += 5; // first mutable borrow used here
    *ref2 += 10;

    println!("{}", num);
}

```

**Correct Code**

```rust
// Correct example
fn main() {
    let mut num = 10;

    let ref1 = &mut num; 
    *ref1 += 5;

    let ref2 = &mut num; 
    *ref2 += 10;

    println!("{}", num);
}
```

**Why?**

`Rust มีกฎเกี่ยวกับการ borrow ว่า ในช่วงเวลาเดียวกัน ตราบที่ borrow ยังถูกใช้งานอยู่ เราสามารถมี mutable borrow ได้เพียงหนึ่งตัวเท่านั้น
เหตุผลที่ Rust ห้ามมี Mutable reference หลายตัวที่อ้างอิงข้อมูลเดียวกันพร้อมกัน เพราะ Mutable reference สามารถแก้ไขข้อมูลที่อ้างถึงได้ Rust จึงต้องให้ Mutable borrow สามารถเข้าถึงข้อมูลแบบ exclusive
เพื่อป้องกันการเข้าถึงหรือแก้ไขข้อมูลเดียวกันจากหลายจุดในเวลาเดียวกัน

ในตัวอย่างแรก ref1 เป็น mutable borrow ที่ยังถูกใช้งานหลังจากมีการสร้าง ref2 (ที่ *ref1 += 5) ดังนั้น borrow นี้ยังไม่สิ้นสุด แต่มีการสร้าง ref2 ซึ่งเป็น mutable borrow อีกตัวไปยัง num เดียวกัน จึงเกิด conflicting borrow (E0499)

ในตัวอย่างที่ถูกต้อง เราทำการใช้ ref1 ให้เสร็จก่อน เมื่อ Mutable borrow ของ ref1 สิ้นสุดลง จึงสามารถสร้าง ref2 เพื่อแก้ไข num ต่อได้`

---
