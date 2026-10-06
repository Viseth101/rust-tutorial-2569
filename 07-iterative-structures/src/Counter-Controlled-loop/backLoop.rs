
fn main(){
    for i in (0..=10).rev(){
        println!("i = {}", i);
    }

    println!("===========================");

    for i in (2..=20).step_by(2){
        println!("i = {}", i);
    }
}

