fn main(){
    let row = 3;
    let column = 4;

    for i in 1..=row{
        for j in 1..=column{
            print!("{} ", i * j);
        }
        println!();
    }
}