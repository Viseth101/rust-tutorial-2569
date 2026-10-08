fn main() {
    let scores = vec![35, 50, 68, 72, 90, 45, 80];

    println!("Original Scores: {:?}", scores);

    let scholarship_students = scores // ยังไม่คำนวณ สร้าง pipeline ไว้ก่อน
        .iter()
        .filter(|score| **score >= 60)
        .map(|score| {
            println!("Adding bonus to: {}", score);
            score + 5
        });

    //จุดที่เริ่มประมวลผลจริง
    let final_scores: Vec<i32> = scholarship_students.collect();

    println!("\n========= Scholarship Award Results =========");

    for score in &final_scores {
        println!("Student recieved scholarship | Final score: {}", score);
    }

    println!("\nTotal Scholarship Students: {}", final_scores.len());

    println!("========= Congratuations ! =========");
}
