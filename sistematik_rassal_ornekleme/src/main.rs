use std::io::stdin;
use rand::Rng;

fn sistematik_rassal_ornekleme(evren: i32, n: i32 ,k: i32) {
    let mut ran_num = rand::thread_rng().gen_range(1..=k);

    while ran_num < evren {
        println!("{ran_num}");
        ran_num += n;
    }
}

fn main() {
    
    //Evren hacmini al
    println!("Evren Hacmi: ");
    let mut evren = String::new();   
    stdin().read_line(&mut evren).expect("Failed");
    let evren: i32 = evren.trim().parse().expect("Invalid"); 

    //Örneklem hacmini al
    println!("Örneklem hacmi: ");
    let mut n = String::new();
    stdin().read_line(&mut n).expect("Failed");
    let n: i32 = n.trim().parse().expect("Invalid");

    //k değerini hesapla
    let k = evren / n;

    sistematik_rassal_ornekleme(evren , n , k);

}
