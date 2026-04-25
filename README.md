# Tugas Besar 2 IF2211 Strategi Algoritma
### Pemanfaatan Algoritma BFS dan DFS dalam Mekanisme Penelusuran CSS pada pohon Document Object Model
 
Aplikasi penelusuran pohon *Document Object Model* (DOM) berbasis web menggunakan algoritma BFS dan DFS untuk pencarian elemen berdasarkan *CSS Selector*. Proyek ini dibangun sepenuhnya menggunakan bahasa pemrograman Rust dengan arsitektur *Client-Server*.
 
## Penjelasan Singkat Algoritma
 
Aplikasi ini mengimplementasikan dua algoritma penelusuran utama untuk menemukan elemen pada pohon DOM:
 
1. **Breadth-First Search (BFS):**
   Menelusuri pohon secara melebar, level demi level. Algoritma ini menggunakan struktur data antrean (*Queue*) berprinsip FIFO. BFS sangat efektif untuk menemukan elemen yang terletak pada tingkat kedalaman yang dangkal (dekat dengan root) dalam waktu yang lebih cepat.
2. **Depth-First Search (DFS):**
   Menelusuri pohon secara mendalam pada satu cabang hingga mencapai daun (*leaf*) sebelum melakukan *backtracking*. Implementasi dilakukan secara iteratif menggunakan *Stack* (LIFO). Untuk memastikan urutan kunjungan sesuai dengan struktur dokumen HTML (kiri ke kanan), anak-anak simpul dimasukkan ke dalam stack dalam urutan terbalik.
*Fitur tambahan meliputi penelusuran paralel menggunakan **Rayon** (Multithreading) dan pencarian leluhur terendah menggunakan **LCA Binary Lifting**.*
 
## Requirement Program & Instalasi
 
Sebelum menjalankan aplikasi secara lokal (*native*), pastikan perangkat Anda telah terpasang:
 
- **Rust (Stable):** [Install Rust](https://www.rust-lang.org/tools/install)
- **Trunk:** Alat untuk memaketkan aplikasi web Rust WASM (`cargo install --locked trunk`)
- **WASM Target:** Tambahkan target kompilasi web (`rustup target add wasm32-unknown-unknown`)
- **Docker & Docker Compose:** *(Opsional, untuk menjalankan via kontainer)*
 
## Langkah-Langkah Build & Run
 
### 1. Menggunakan Docker 
 
Cara termudah untuk menjalankan aplikasi tanpa konfigurasi manual:
 
```bash
# Masuk ke direktori utama
cd Tubes2_cRUSTacean
 
# Bangun dan jalankan kontainer
docker-compose up --build
```
 
 
### 2. Menggunakan Native Cargo
 
Jika ingin menjalankan tanpa Docker:
 
**Terminal 1 — Backend:**
```bash
cd crates/backend
cargo run
```
 
**Terminal 2 — Frontend:**
```bash
cd crates/frontend
trunk serve
```

 
## Checklist Penilaian
 
| No | Poin | Ya | Tidak |
|----|------|----|-------|
| 1  | Aplikasi berhasil dikompilasi tanpa kesalahan |✓| |
| 2  | Aplikasi berhasil dijalankan |✓| |
| 3  | Aplikasi dapat menerima input URL web, pilihan algoritma, CSS selector, dan jumlah hasil |✓| |
| 4  | Aplikasi dapat melakukan scraping terhadap web pada input |✓ | |
| 5  | Aplikasi dapat menampilkan visualisasi pohon DOM |✓| |
| 6  | Aplikasi dapat menelusuri pohon DOM dan menampilkan hasil penelusuran |✓| |
| 7  | Aplikasi dapat menandai jalur tempuh oleh algoritma |✓| |
| 8  | Aplikasi dapat menyimpan jalur yang ditempuh algoritma dalam traversal log |✓| |
| 9  | **[Bonus]** Membuat video | | |
| 10 | **[Bonus]** Deploy aplikasi |✓| |
| 11 | **[Bonus]** Implementasi animasi pada penelusuran pohon |✓| |
| 12 | **[Bonus]** Implementasi multithreading |✓| |
| 13 | **[Bonus]** Implementasi LCA Binary Lifting |✓| |
 
 
## Author (Identitas Pembuat)
 
**Kelompok cRUSTacean**
 
| Nama                       | NIM      | Handle                                               |
|----------------------------|----------|------------------------------------------------------|
| Mikhael Benrael Tampubolon | 13524009 | [@MikhaelBenrael](https://github.com/MikhaelBenrael) |
| Muhammad Iqbal Raihan      | 13524011 | [@Gixgine-budi](https://github.com/Gixgine-budi)     |
| Muhammad Akmal             | 13524099 | [@m-akma1](https://github.com/m-akma1)               |
 
Program Studi Teknik Informatika - Sekolah Teknik Elektro dan Informatika
Institut Teknologi Bandung
 