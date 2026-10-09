// ========================================
//            key 1: iter()
// ========================================
fn key_concept_1() {
    let numbers = vec![1, 2, 3];

    for number in numbers.iter() {
        println!("{}", number);
    }
}

// ========================================
//             key 2: map()
// ========================================
fn key_concept_2() {
    let numbers = vec![1, 2, 3];

    let doubled = numbers.iter().map(|number| number * 2);

    for number in doubled {
        println!("{}", number);
    }
}

// ========================================
//            key 3: filter()
// ========================================
fn key_concept_3() {
    let numbers = vec![1, 2, 3, 4, 5];

    let even_numbers = numbers
        .iter()
        .filter(|number| **number % 2 == 0);

    for number in even_numbers {
        println!("{}", number);
    }
}

// ========================================
//              key 4: fold()
// ========================================
fn key_concept_4() {
    let numbers = vec![1, 2, 3, 4];

    let sum = numbers
        .iter()
        .fold(0, |total, number| total + number);

    println!("{}", sum);
}

// ========================================
//            key 5: collect()
// ========================================
fn key_concept_5() {
    let numbers = vec![1, 2, 3];

    let doubled: Vec<i32> = numbers
        .iter()
        .map(|number| number * 2)
        .collect();

    println!("{:?}", doubled);
}
// ========================================
// iterator and High order programming
// ========================================


fn main() {
    println!("========================================");
    println!("key concept 1: iter()");
    key_concept_1();
    println!("========================================");
    println!("key concept 2: map()");
    key_concept_2();
    println!("========================================");
    println!("key concept 3: filter()");
    key_concept_3();
    println!("========================================");
    println!("key concept 4: fold()");
    key_concept_4();
    println!("========================================");
    println!("key concept 5: collect()");
    key_concept_5();
    println!("========================================");
}
