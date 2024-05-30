use std::{collections::HashMap,io::stdin};
use statrs::statistics::Statistics;
use std::ops::{Sub, Div, Add};

fn frekans_hesapla_i32(arr: Vec<i32>) -> HashMap<i32, i32> {
    let mut frekans: HashMap<i32, i32> = HashMap::new();

    for val in arr {
        let counter = frekans.entry(val).or_insert(0);
        *counter += 1;
    }

    frekans
}

fn frekans_hesapla_f64(arr: Vec<f64>, precision: usize) -> HashMap<String, i32> {
    let mut frekans: HashMap<String, i32> = HashMap::new();

    for val in arr {
        let key = format!("{:.1$}", val, precision);
        let counter = frekans.entry(key).or_insert(0);
        *counter += 1;
    }

    frekans
}

fn frekans_seri_i32(arr: Vec<i32>) {
    let frekanslar = frekans_hesapla_i32(arr.clone());
    let mut frekans_vec: Vec<(&i32, &i32)> = frekanslar.iter().collect();
    frekans_vec.sort_by(|a, b| a.0.cmp(b.0));

    println!("\nFrekans Serisi:");
    for (val, count) in frekans_vec {
        println!("{}: {}", val, count);
    }
}

fn frekans_seri_f64(arr: Vec<f64>, precision:usize ){
    let frekanslar = frekans_hesapla_f64(arr.clone(), precision);
    let mut frekans_vec: Vec<(&String, &i32)> = frekanslar.iter().collect();
    frekans_vec.sort_by(|a, b| a.0.partial_cmp(b.0).unwrap());

    println!("\nFrekans Serisi:");
    for (val, count) in frekans_vec {
        println!("{}: {}", val, count);
    }
}

fn frekans_tablosu_i32(arr: Vec<i32>)
where
    i32: Sub<Output = i32> + Add<i32, Output = i32>,

{
    let n = arr.len() as i32;
    println!("\n\nn: {}", n);
    let min = *arr.iter().min().unwrap();
    let max = *arr.iter().max().unwrap();
    let r = max - min;
    let k = (n as f32).sqrt().ceil() as i32;
    let h = ((r as f64) / (k as f64)).ceil();
    let h_int: i32 = h as i32;
    println!("n: {}, r: {}, k: {}, h: {}\n\n", n, r, k, h_int);

    println!("Sınıf Limitleri\nAlt \t Üst \t SON");
    let first = arr.first();
    let mut value = first.copied().unwrap();
    let mut son =  ((value as f64) + ((value as f64) + (h_int as f64) + -1.0))/2.0;
    for _ in 0..k {
        println!("{:?} \t {:?} \t {}", value, value + h_int - 1, son);
        value = value + h_int;
        son =  ((value as f64) + ((value as f64) + (h_int as f64) + -1.0))/2.0;
    }

    println!("Sınıf Sınırları\nAlt \t Üst \t Sınıf Frekansı\t Eklenik Frekans \t Oransal Frekans \t Oransal Eklenik Frekans");
    let mut value = first.copied().unwrap();
    let mut alt_sinir_i32 = (value as f64 - 0.5) as f64;
    let mut toplam_frekans = 0;
    let mut toplam_oransal_frekans = 0;

    let mut frekanslar: Vec<i32> = Vec::new();
    let mut sinirlar: Vec<(f64, f64)> = Vec::new();
    let mut eklenik_frekanslar: Vec<i32> = Vec::new();

    for _ in 0..k {
        let ust_sinir = alt_sinir_i32 +h;
        let frekans = arr.iter().filter(|&&x| (x as f64) >= alt_sinir_i32 && (x as f64) < ust_sinir).count() as i32;
        toplam_frekans += frekans;
        toplam_oransal_frekans += frekans;

        sinirlar.push((alt_sinir_i32,ust_sinir));
        frekanslar.push(frekans);
        eklenik_frekanslar.push(toplam_frekans);

        println!(
            "{:?} \t {:?} \t {:?} \t\t {} \t\t\t {}/{} \t\t\t {}/{}",
            alt_sinir_i32, ust_sinir, frekans,toplam_frekans, frekans, n, toplam_oransal_frekans, n
        );
        value = value + h_int;
        alt_sinir_i32 = (value as f64 - 0.5) as f64;
    }

    // Q1 hesaplama
    let q1_index = n as f64 / 4.0;
    let q1_sinif = eklenik_frekanslar.iter().position(|&x| x as f64 >= q1_index).unwrap();
    let q1_n1 = if q1_sinif == 0 { 0 } else { eklenik_frekanslar[q1_sinif - 1] };
    let q1_f = frekanslar[q1_sinif];
    let q1_l = sinirlar[q1_sinif].0;
    let q1 = q1_l + (((q1_index - q1_n1 as f64) * h) / q1_f as f64);

    // Q3 hesaplama
    let q3_index = 3.0 * n as f64 / 4.0;
    let q3_sinif = eklenik_frekanslar.iter().position(|&x| x as f64 >= q3_index).unwrap();
    let q3_n1 = if q3_sinif == 0 { 0 } else { eklenik_frekanslar[q3_sinif - 1] };
    let q3_f = frekanslar[q3_sinif];
    let q3_l = sinirlar[q3_sinif].0;
    let q3 = q3_l + (((q3_index - q3_n1 as f64) * h) / q3_f as f64);

    println!("Q1: {:.4}, Q3: {:.4}", q1, q3);
}

fn frekans_tablosu_f64(arr: Vec<f64>)
where
    f64: Sub<Output = f64> + Add<f64, Output = f64> + Div<f64, Output = f64> + From<i32> + Copy,
{
    let n = arr.len() as i32;
    println!("\n\nn: {}", n);
    let min = *arr.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max = *arr.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let r = max - min;
    let k = (n as f64).sqrt().ceil();
    let h = (r / k).ceil();
    println!("n: {}, r: {:.2}, k: {:.0}, h: {:.2}\n\n", n, r, k, h);

    println!("Sınıf Limitleri\nAlt \t Üst \t SON");
    let mut value = min;
    let mut son = (value + (value+h - 0.1))/2.0; // Initial value for son
    for _ in 0..k as i32{
        println!("{:.2} \t {:.2} \t {:.2}", value, value + h - 0.1, son);
        value = value + h;
        son = (value + (value+h - 0.1))/2.0; // Update son for the next iteration
    }

    println!("Sınıf Sınırları\nAlt \t Üst \t Sınıf frekansı \t Eklenik Frekans \t Oransal Frekans \t OEF");
    let mut alt_sinir_f64 = min - 0.05;
    let mut toplam_frekans = 0;
    let mut toplam_oransal_frekans = 0;

    let mut frekanslar: Vec<i32> = Vec::new();
    let mut sinirlar: Vec<(f64, f64)> = Vec::new();
    let mut eklenik_frekanslar: Vec<i32> =Vec::new(); 

    for _ in 0..k as i32 {
        let ust_sinir = alt_sinir_f64 + h;
        let frekans = arr.iter().filter(|&&x| x >= alt_sinir_f64 && x < ust_sinir).count() as i32;
        toplam_frekans += frekans;
        toplam_oransal_frekans += frekans;

        sinirlar.push((alt_sinir_f64,ust_sinir));
        frekanslar.push(frekans);
        eklenik_frekanslar.push(toplam_frekans);

        println!(
            "{:.2} \t {:.2} \t\t {frekans}\t\t {toplam_frekans} \t\t\t {frekans}/{n} \t\t\t{toplam_oransal_frekans}/{n}", 
            alt_sinir_f64, alt_sinir_f64 + h
        );
        alt_sinir_f64 = alt_sinir_f64 + h;
    }

    //Q1 Hesaplama
    let q1_index = n as f64 / 4.0;
    let q1_sinif = eklenik_frekanslar.iter().position(|&x| x as f64 >= q1_index).unwrap();
    let q1_n1 = if q1_sinif == 0 {0} else {eklenik_frekanslar[q1_sinif - 1]};
    let q1_f = frekanslar[q1_sinif];
    let q1_l = sinirlar[q1_sinif].0;
    let q1 = q1_l + (((q1_index - q1_n1 as f64) * h) / q1_f as f64);

    // Q3 hesaplama
    let q3_index = 3.0 * n as f64 / 4.0;
    let q3_sinif = eklenik_frekanslar.iter().position(|&x| x as f64 >= q3_index).unwrap();
    let q3_n1 = if q3_sinif == 0 { 0 } else { eklenik_frekanslar[q3_sinif - 1] };
    let q3_f = frekanslar[q3_sinif];
    let q3_l = sinirlar[q3_sinif].0;
    let q3 = q3_l + (((q3_index - q3_n1 as f64) * h) / q3_f as f64);

    println!("Q1: {:.2}, Q3: {:.}", q1, q3);
}

fn armegeo_i32(arr: &[i32]) {
    //Aritmetik Ortalama
    let data: Vec<f64> = arr.iter().map(|&x| x as f64).collect();
    let aritmetik_ort = data.clone().mean();
    println!("Aritmetik Ortalam: {:.3}",aritmetik_ort);

    //Medyan
    let median_data = if arr.len() % 2 == 0 {
        let mid = arr.len() / 2;
        (arr[mid - 1] + arr[mid]) as f64 / 2.0
    } else {
        arr[arr.len() / 2] as f64
    };
    println!("Medyan: {:?}",median_data);

    //Mod
    let mode_data = arr.iter()
        .fold(HashMap::new(), |mut acc, &num| {
            *acc.entry(num).or_insert(0) += 1;
            acc
        })
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(num, _)| num);
    println!("Mod: {:?}",mode_data.unwrap());

    let geometrik_ort = data.clone().geometric_mean();
    println!("Geometrik Ortalama: {:.3}", geometrik_ort);

    let harmonik_ort = data.clone().harmonic_mean();
    println!("Harmonik Ortalama: {:.3}", harmonik_ort);
    
    // n 
    let n = arr.len() as f64;

    //S^2 varyans ve Standart Sapma
    let mut var_pay = 0.0;
    for i in arr {
        var_pay += (*i as f64 - aritmetik_ort).powf(2.0);
    }
    println!("Örneklem Varyansı(s^2): {:.2}\nStandart Sapma: {:.2}",var_pay/(n - 1.0),(var_pay/(n - 1.0)).sqrt());

    //Ortalama Mutlak Sapma
    let mut oms_pay = 0.0;
    for a in arr{
        oms_pay += (*a as f64 - aritmetik_ort).abs();    
    }
    println!("OMS: {:.2}", oms_pay/(n));

    //M3
    let mut m3_pay = 0.0;
    for i in arr {
        m3_pay += (*i as f64 - aritmetik_ort).powf(3.0);
    }
    println!("M3: {:.2}", m3_pay/(n - 1.0));

    //M4
    let mut m4_pay = 0.0;
    for i in arr {
        m4_pay += (*i as f64 - aritmetik_ort).powf(4.0);
    }
    println!("M3: {:.2}", m4_pay/(n - 1.0));

    //Değişim Katsayısı
    let deg_kat = (var_pay/(n - 1.0)).sqrt() / aritmetik_ort;
    println!("Değişim Katsayısı: {:.2}", deg_kat);

}

fn armegeo_f64(arr: &[f64]) {
    // Aritmetik Ortalama
    let aritmetik_ort = arr.mean();
    println!("Aritmetik Ortalama: {:.2}", aritmetik_ort);

    // Medyan
    let median_data = if arr.len() % 2 == 0 {
        let mid = arr.len() / 2;
        (arr[mid - 1] + arr[mid]) / 2.0
    } else {
        arr[arr.len() / 2]
    };
    println!("Medyan: {:.2}", median_data);

    // Mod
    let mut sorted_arr = arr.to_vec();
    sorted_arr.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mut max_count = 0;
    let mut mode_data: Option<f64> = None;
    let mut current_count = 1;
    let mut current_num = sorted_arr[0];
    for &num in sorted_arr.iter().skip(1) {
        if num == current_num {
          current_count += 1;
        } else {
            if current_count > max_count {
                 max_count = current_count;
                mode_data = Some(current_num);
            }
            current_num = num;
            current_count = 1;
        }
    }
    // En son tekrar sayısını kontrol et
    if current_count > max_count {
        //max_count = current_count;
        mode_data = Some(current_num);
    }
    println!("Mod: {:?}", mode_data.unwrap());

    // Geometrik Ortalama
    let geometrik_ort = arr.geometric_mean();
    println!("Geometrik Ortalama: {:.2}", geometrik_ort);

    // Harmonik Ortalama
    let harmonik_ort = arr.harmonic_mean();
    println!("Harmonik Ortalama: {:.2}", harmonik_ort);
    
    // Eleman Sayısı (n)
    let n = arr.len() as f64;

    // Varyans ve Standart Sapma
    let mut var_pay = 0.0;
    for &i in arr {
        var_pay += (i - aritmetik_ort).powf(2.0);
    }
    println!("Örneklem Varyansı(s^2): {:.2}\nStandart Sapma: {:.2}", var_pay / (n - 1.0), (var_pay / (n - 1.0)).sqrt());

    // Ortalama Mutlak Sapma (OMS)
    let mut oms_pay = 0.0;
    for &a in arr {
        oms_pay += (a - aritmetik_ort).abs();
    }
    println!("OMS: {:.2}", oms_pay / n);

    // M3
    let mut m3_pay = 0.0;
    for &i in arr {
        m3_pay += (i - aritmetik_ort).powf(3.0);
    }
    println!("M3: {:.2}", m3_pay / (n - 1.0));

    // M4
    let mut m4_pay = 0.0;
    for &i in arr {
        m4_pay += (i - aritmetik_ort).powf(4.0);
    }
    println!("M4: {:.2}", m4_pay / (n - 1.0));

    // Değişim Katsayısı
    let deg_kat = (var_pay / (n - 1.0)).sqrt() / aritmetik_ort;
    println!("Değişim Katsayısı: {:.2}", deg_kat);
}

fn main() {
    let mut input = String::new();

    println!("Veri tipi seçiniz (Tam sayı için 1, Ondalıklı sayı için 2'ye basın): ");
    stdin().read_line(&mut input).unwrap();
    let veri_tipi: i32 = input.trim().parse().unwrap();

    if veri_tipi == 1 {
        println!("Tam sayı verilerini giriniz (Verileri boşluk ile ayırınız): ");
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let data: Vec<i32> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();

        let mut data_sorted = data.clone();
        data_sorted.sort_by(|a, b| a.cmp(b));
        println!("Basit Seri:");
        data_sorted.iter().for_each(|val| print!("{} ", val));

        frekans_seri_i32(data_sorted.clone());
        frekans_tablosu_i32(data_sorted.clone());

        armegeo_i32(&data_sorted.clone());
    } else if veri_tipi == 2 {
        println!("Ondalıklı sayı verilerini giriniz (Verileri boşluk ile ayırınız): ");
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let data: Vec<f64> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();

        println!("Veri hassasiyetini giriniz (örneğin, 2 ondalık basamak için 2 giriniz): ");
        let mut precision_input = String::new();
        stdin().read_line(&mut precision_input).unwrap();
        let precision: usize = precision_input.trim().parse().unwrap();

        let mut data_sorted = data.clone();
        data_sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        println!("Basit Seri:");
        data_sorted.iter().for_each(|val| print!("{:.2} ", val));

        frekans_seri_f64(data_sorted.clone(), precision);
        frekans_tablosu_f64(data_sorted.clone());

        armegeo_f64(&data_sorted.clone());
    } else {
        println!("Geçersiz seçim!");
    }
}
