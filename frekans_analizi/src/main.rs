use std::{collections::HashMap, io::stdin};
use std::hash::Hash;


fn frekans_hesapla<T: Eq + Hash + Copy + std::fmt::Debug>(arr: Vec<T>) -> HashMap<T,i32>{
    let mut frekans: HashMap<T,i32> = HashMap::new();

    for val in arr {
        let counter = frekans.entry(val).or_insert(0);
        *counter += 1;
    }
    
    //println!("{:?}", frekans);

    frekans
}

fn  frekans_seri<T: Eq + Hash + Copy + Ord + std::fmt::Debug + std::fmt::Display>(arr: Vec<T>) {
    let frekanslar = frekans_hesapla(arr.clone());
    let mut frekans_vec: Vec<(&T, &i32)> = frekanslar.iter().collect();
    frekans_vec.sort_by(|a, b| a.0.cmp(b.0)); // Anahtara göre sıralama

    println!("\nFrekans Serisi:");
    for (val,count) in frekans_vec {
        println!("{}: {}", val, count);
    }
}

fn frekans_tablosu<T: Ord + Copy + std::fmt::Display + std::ops::Sub>(arr: Vec<T>) {
    let n:i32 = arr.capacity().try_into().unwrap();
    println!("\n\nn: {}", n);
    let min = *arr.iter().min().unwrap();
    let max = *arr.iter().max().unwrap();
    //println!("min: {} \t max: {}", min, max);
    let r = max - min;
    
}

fn main() {
    
    let mut input = String::new();

    println!("Verileri Giriniz(Verileri Boşluk ile ayiriniz): ");
    stdin().read_line(&mut input).unwrap();

    //let mut data: Vec<&str> = input.trim().split(" ").map(|x| x.parse().unwrap()).collect();
    let data: Vec<&str> = input.trim().split_whitespace().collect();
    let data: Vec<i32> = data.iter().map(|&x| x.parse().unwrap()).collect();

    // Vectördeki verileri küçükten büyüğe sıralama
    let mut data_sorted = data.clone();
    data_sorted.sort_by(|a, b| a.cmp(b));
    //println!("{:?}", data_sorted);


    // Basit Seri
    println!("Basit Seri:");
    data_sorted.iter().for_each(|val| print!("{} ",val));

    //Frekans Serisi
    frekans_seri(data_sorted.clone());
    
    frekans_tablosu(data_sorted.clone());





    //frekans hesaplama
    //frekans_hesapla(data.clone());

   //println!("Vector: {:?}", data);
   

}
