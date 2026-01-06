
// Program kalkulator Zakat Fitrah menggunakan Rust
// Author : Gesang Paudra Jaya

//import dependensi standar
use std::io::{self, BufRead};
use thousands::Separable;


fn main() {
    println!("Hello, world!");

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    println!("Selamat Datang Di Program Kalkulator Zakat Fitrah Berbasis Rust");
    println!("Author : Gesang Paudra Jaya");
    println!("");
    println!("Masukkan jumlah jiwa dalam keluarga anda :");
    let jiwa_line = lines.next().unwrap().unwrap();
    let jumlah_jiwa:u32 = jiwa_line.trim().parse().expect("Error : Input harus angka!");

    println!("=============================");
    println!("Masukkan harga beras per kilogram saat ini :");
    let beras_line = lines.next().unwrap().unwrap();
    let harga_beras:f64 = beras_line.trim().parse().expect("Error : Input harus angka!");

    let total_nilai_zakat = nilai_zakat(jumlah_jiwa, harga_beras);
    println!("=============================");
    println!("Jumlah jiwa dalam keluarga : {}", jumlah_jiwa);
    println!("Total nilai zakat fitrah yang harus dikeluarkan : Rp{}", total_nilai_zakat.separate_with_commas());
    println!("Jumlah zakat fitrah yang harus dikeluarkan : {} Kg beras", jumlah_kg(jumlah_jiwa));
    println!("Jumlah zakat fitrah yang harus dikeluarkan : {} Lt beras", jumlah_liter(jumlah_jiwa));

}

fn jumlah_kg(jumlah_jiwa:u32) -> f64 {
    // Zakat fitrah adalah 2.5 kg beras per jiwa
    (jumlah_jiwa as f64) * 2.5
}

fn jumlah_liter(jumlah_jiwa:u32) -> f64 {
    // Zakat fitrah adalah 3.5 Liter beras per jiwa
    (jumlah_jiwa as f64) * 3.5
}

fn nilai_zakat(jumlah_jiwa:u32, harga_beras:f64) -> f64 {
    // Total nilai zakat fitrah
    let total_beras = jumlah_kg(jumlah_jiwa);
    total_beras * harga_beras
}
