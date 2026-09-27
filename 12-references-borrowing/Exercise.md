## 8. Exercises

### Exercise 1 — `To go or not to go, that is the question`

**Problem**

`กลุ่มเพื่อนกลุ่มหนึ่งกำลังจะไปเที่ยวด้วยกัน โดย Noah ชวนเพื่อนมาได้อีก 2 คนคือ Oliver กับ James ในภายหลัง Quinn ตัดสินใจที่จะไปด้วย
เหลือเพื่อนอีกคนชื่อ Charlotte ที่กำลังตัดสินใจว่าจะไปมั้ย โดยที่ตนคิดไว้ว่าจะไปถ้า Oliver ไปด้วย James จึงอยากเขียนโค้ดออกมาสั้นๆให้เห็นว่าใครเป็นคนชวน
ไปเที่ยว รวมถึงคนที่จะไปด้วยตามลำดับเหตุการณ์ แต่โค้ดของเขากลับติด error`

```rust
// James' code
fn main() {
    let mut names = vec![String::from("Noah"), String::from("Oliver"), String::from("James")];

    let first = &names[0];
    names.push(String::from("Quinn"));
    println!("Hosted by: {}", first);

    for name in &names {
        if name == "Oliver" {
            names.push(String::from("Charlotte"));
        }
    }
    println!("{:?}", names);
}
```
`แต่โค้ดของเขากลับ compile ไม่ผ่าน จงหาว่า ทำไม compile ไม่ผ่านเพราะส่วนไหนบ้าง แล้วแก้ไขอย่างไรโดยการ restructure โค้ดดังกล่าว`


**Hint**

`ลองสังเกตการ borrow แต่ละอันว่าเริ่ม/จบตรงไหน`

**Solution**

```rust
// Solution code
fn main() {
    let mut names = vec![String::from("Noah"), String::from("Oliver"), String::from("James")];

    let first = &names[0];
    println!("Hosted by: {}", first);
    names.push(String::from("Quinn"));
    
    let mut add = false;
    for name in &names {
        if name == "Oliver" {
            add = true;
        }
    }
    if add {
        names.push(String::from("Charlotte"));
    }
    println!("{:?}", names);
}
```

**Explanation**

`มีส่วนผิดอยู่ 2 จุด
จุดที่ 1: first ถูกยืมใช้งานข้าม push()
first เป็น immutable reference ที่ยังถูกใช้งานอยู่ ในขณะที่ names.push(...) เรียกใช้งาน mutable borrow ของ names ซึ่ง Rust ไม่อนุญาต
เนื่องจาก push อาจทำให้เกิด vector reallocate memory ทำให้ first กลายเป็น dangling reference ได้
วิธีแก้: ใช้ first ก่อน แล้วนำไปใช้งานเพื่อให้ borrow จบลงก่อนที่จะใช้ push
`
```rust
println!("Hosted by: {}", &names[0]); // ยืม ใช้ แล้วจบเลยในบรรทัดเดียว
names.push(String::from("Quinn"));
```
`
จุดที่ 2: push ข้างในลูปที่กำลัง borrow names แบบ immutable
for name in &names ทำให้ names ถูกยืมแบบ immutable ไปตลอดการวนลูป การจะ push เข้าไปใน names ระหว่างนั้นต้องใช้ mutable borrow ซึ่งขัดกัน
วิธีแก้: แบ่งการทำงานเดิมออกเป็น 2 ส่วนคือ ส่วนการตัดสินใจ และ ส่วนการเพิ่มข้อมูลเข้า
`
```rust
let mut should_add = false;
for name in &names {
    if name == "Oliver" {
        should_add = true;
    }
}
if should_add {
    names.push(String::from("Charlotte"));
}
```

---

### Exercise 2 — `Longest Entry in Report`

**Problem**

```rust
fn longest_line(text: &String) -> &str {
    let mut best = "";
    for line in text.lines() {
        if line.len() > best.len() {
            best = line;
        }
    }
    best
}

fn get_report() -> &'static str {
    let text = String::from("short\na much longer line here\nmid");
    longest_line(&text)
}

fn main() {
    let report = get_report();
    println!("{}", report);
}
```

`โค้ดนี้สามารถ compile ได้ไหม ถ้าไม่ได้ เป็นเพราะอะไร แก้ไขอย่างไร`

**Hint**

`ลองพิจารณาว่าการ return ค่าแบบใดที่จะทำให้ข้อมูลยังสามารถถูก reference ได้`

**Solution**

```rust
// Solution code
fn longest_line(text: &String) -> String {
    let mut best = "";
    for line in text.lines() {
        if line.len() > best.len() {
            best = line;
        }
    }
    best.to_string()
}

fn get_report() -> String {
    let text = String::from("short\na much longer line here\nmid");
    longest_line(&text)
}

fn main() {
    let report = get_report();
    println!("{}", report);
}
```

**Explanation**

`โค้ดนี้ไม่สามารถ compile ได้เพราะ get_report() ประกาศว่าจะคืนค่า &'static str แต่ longest_line() คืน reference ที่ยืมมาจาก text ซึ่งเป็น local variable และมีอายุไม่ถึง 'static จึงไม่สามารถคืน reference นี้ออกจากฟังก์ชันได้ วิธีแก้หนึ่งคือเปลี่ยน return type ของทั้งสองฟังก์ชันเป็น String เพื่อคืน ownership ของข้อความ`

---
