use std::io::stdin;
use rand::Rng;
use std::ops::RangeInclusive;


fn basit_rassal_ornekleme(mini: i32, maxi: i32, adet: i32) {
    let mut i: i32 = 0;

    let mut r_numbers: Vec<i32> = Vec::new();

    let range = RangeInclusive::new(mini, maxi);

    if (maxi - mini +1) > adet{
        while i < adet {
            let random_num = rand::thread_rng().gen_range(range.clone());

            if !r_numbers.contains(&random_num) {
                r_numbers.push(random_num);
            }
            i += 1;
        }

        for random_num in r_numbers {
            println!("{}", random_num);
        }
        
    } else {
        while i < adet {
            let random_num = rand::thread_rng().gen_range(range.clone());
            
            println!("{}", random_num);
                
            i += 1;
        }
    }

    
    

}


fn main() {
    
    // Maximum number
    let mut minimum = String::new();
    println!("Minimum number: ");
    stdin().read_line(&mut minimum).expect("Failed");
    let minimum: i32 = minimum.trim().parse().expect("İnvalid");

    //minimum number
    let mut maximum = String::new();
    println!("Maximum number: ");
    stdin().read_line(&mut maximum).expect("Failed");
    let maximum: i32 = maximum.trim().parse().expect("İnvalid"); 
   

    // rastegele sayı adedi
    let mut adet = String::new();
    println!("İstenen rastgele sayi adedi: ");
    stdin().read_line(&mut adet).expect("Failed");
    let adet:i32 = adet.trim().parse().expect("İnvalid"); 
    

    basit_rassal_ornekleme(minimum, maximum, adet);


}
