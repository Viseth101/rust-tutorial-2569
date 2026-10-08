fn main() {
    let areas = vec![
        ("Chiang Rai", 150), 
        ("Phayao", 80), 
        ("Chiang Mai", 120), 
        ("Bangkok", 15)
    ];
    
    let rescue_missions: Vec<String> = areas
        .iter()
        .filter(|&&(_, water_level)| water_level >= 100) 
        .map(|&(province, _)| format!("{} prepare boats and mama rn", province)) 
        .collect(); 
        
    println!("{:?}", rescue_missions);
}
