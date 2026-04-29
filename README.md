(# File Guardian

File Guardian เป็นโปรแกรม CLI Tool ที่เขียนด้วยภาษา Rust สำหรับตรวจสอบความเปลี่ยนแปลงของไฟล์ในโฟลเดอร์ เช่น ไฟล์ถูกแก้ไข ไฟล์ใหม่ถูกเพิ่ม หรือไฟล์เดิมถูกลบ

โปรเจกต์นี้เกี่ยวข้องกับ Security เพราะใช้แนวคิด File Integrity Checking โดยตรวจสอบความถูกต้องของไฟล์จากค่า SHA-256 Hash และเก็บ baseline แบบเข้ารหัสในไฟล์ `baseline.enc`

---

## Features

- Scan folder และสร้าง baseline จากค่า SHA-256 hash ของไฟล์
- เข้ารหัสไฟล์ baseline เป็น `baseline.enc`
- Check file integrity เพื่อตรวจจับไฟล์ที่ถูกแก้ไข เพิ่ม หรือลบ
- Generate report เป็นไฟล์ `report.txt`
- มี basic error handling เช่น command ผิด, path ผิด, folder ไม่มี, baseline ไม่มี และ password ผิด

---

## Project Structure

```text
file-guardian/
│
├── src/
│   └── main.rs
│
├── test_folder/
│   ├── config.txt
│   ├── notes.txt
│   └── data.txt
│
├── Cargo.toml
├── Cargo.lock
├── README.md
├── .gitignore
│
├── baseline.enc
└── report.txt

