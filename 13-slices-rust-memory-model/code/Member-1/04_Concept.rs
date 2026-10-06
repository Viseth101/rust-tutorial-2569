fn main(){
    // Memory model of Stack

    let x = 60;
    let y = 7;
    let z = x + y;

    // Memory model of Heap

    let s = String::from("Hello");
    let myVec = vec![1, 2, 3, 5];

    // Slicing

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

    // Mutable Slices

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

    // &str vs .clone()
    
    let s = String::from("Hello");
    let borrowed: &str = &s;
    let copied = s.clone();

    println!("s         = {s}");        // จะได้ Hello โดยที่อยู่ Heap จะเป็นของ s 
    println!("borrowed  = {borrowed}"); // จะได้ Hello โดยที่อยู่ Heap จะเป็นของ s
    println!("copied    = {copied}");   // จะได้ Hello โดยที่อยู่ Heap จะเป็นของ Copied เอง
}
