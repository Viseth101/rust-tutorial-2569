fn main() {
    let medals = vec![
        ("Sepak Takraw", 4), 
        ("Esports (RoV)", 1), 
        ("Tamiya", 5), //กีฬา Tamiya เพิ่งถูกบรรจุใหม่ที่สนาม Nagoya-grand prix 
        ("Muay Tha", 3)
    ];
    
    let total_gold = medals.iter().fold(2, |acc, &(_, count)| acc + count);
    
    println!("Thailand total gold medals won: {} medals", total_gold);
}

