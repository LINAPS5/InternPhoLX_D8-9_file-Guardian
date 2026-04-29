(# InternPhoLX_D8-9_file-Guardian

## Overview

File Guardian เป็นโปรแกรม CLI Tool ที่เขียนด้วยภาษา Rust สำหรับตรวจสอบความเปลี่ยนแปลงของไฟล์ในโฟลเดอร์ เช่น ไฟล์ถูกแก้ไข ไฟล์ใหม่ถูกเพิ่ม หรือไฟล์เดิมถูกลบ

โปรเจกต์นี้เกี่ยวข้องกับ Security เพราะใช้แนวคิด File Integrity Checking โดยตรวจสอบไฟล์จากค่า SHA-256 Hash และเก็บข้อมูล baseline ไว้ในไฟล์ `baseline.enc` ที่ถูกเข้ารหัสด้วย password

---

## Project Type

```text
Security / CLI Tool / Automation
```

---

## Requirements

โปรเจกต์นี้ตรงตาม Requirement ขั้นต่ำดังนี้

```text
มี input/output ชัดเจน
มีอย่างน้อย 3 features
มี error handling พื้นฐาน
ส่ง Git repository
อธิบาย design decision ได้
```

---

## Features

### 1. Scan Folder

โปรแกรมสามารถสแกนไฟล์ทั้งหมดในโฟลเดอร์ และ subfolder ได้

```bash
cargo run -- scan ./test_folder
```

โปรแกรมจะอ่านไฟล์ทั้งหมด แล้วสร้างค่า SHA-256 hash ของแต่ละไฟล์

---

### 2. Create Encrypted Baseline

หลังจากสแกนไฟล์ โปรแกรมจะสร้างไฟล์ baseline แบบเข้ารหัส

```text
baseline.enc
```

ไฟล์นี้ใช้เก็บข้อมูลชื่อไฟล์และ hash ของไฟล์  
ถ้าเปิดไฟล์นี้โดยตรง จะอ่านไม่รู้เรื่อง เพราะถูกเข้ารหัสไว้

---

### 3. Check File Integrity

โปรแกรมสามารถตรวจสอบว่าไฟล์เปลี่ยนไปจาก baseline เดิมหรือไม่

```bash
cargo run -- check ./test_folder
```

สถานะที่ตรวจจับได้มี 4 แบบ

```text
[UNCHANGED] = ไฟล์ไม่เปลี่ยน
[MODIFIED]  = ไฟล์ถูกแก้ไข
[NEW]       = ไฟล์ใหม่ถูกเพิ่มเข้ามา
[DELETED]   = ไฟล์เดิมถูกลบ
```

---

### 4. Generate Report

หลังจากตรวจสอบเสร็จ โปรแกรมจะสร้างไฟล์รายงาน

```text
report.txt
```

ในไฟล์นี้จะเก็บผลลัพธ์ทั้งหมด เช่น ไฟล์ไหนถูกแก้ไข ไฟล์ไหนถูกลบ และสรุปจำนวนแต่ละประเภท

---

### 5. Basic Error Handling

โปรแกรมมีการจัดการ error พื้นฐาน เช่น

```text
ใส่ command ไม่ครบ
ใส่ command ผิด
folder ไม่มีอยู่จริง
path ที่ใส่มาไม่ใช่ folder
ยังไม่มี baseline.enc
ใส่ password ผิด
ไม่สามารถอ่านไฟล์ได้
ไม่สามารถเขียน report ได้
```

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
```

---

## Important Files

### src/main.rs

ไฟล์โค้ดหลักของโปรแกรม ใช้รับ command จากผู้ใช้ และจัดการ logic ทั้งหมด เช่น scan, check, hash, encrypt และ generate report

### Cargo.toml

ไฟล์ตั้งค่าโปรเจกต์ Rust และ dependency

### test_folder/

โฟลเดอร์ตัวอย่างสำหรับใช้ Demo

### baseline.enc

ไฟล์ baseline ที่ถูกเข้ารหัส เกิดหลังจากรันคำสั่ง scan

### report.txt

ไฟล์รายงานผล เกิดหลังจากรันคำสั่ง check

---

## Dependencies

โปรเจกต์นี้ใช้ crate เพียงตัวเดียว

```toml
sha2 = "0.10"
```

ใช้สำหรับคำนวณค่า SHA-256 hash ของไฟล์

---

## Cargo.toml

```toml
[package]
name = "file-guardian"
version = "0.1.0"
edition = "2021"

[dependencies]
sha2 = "0.10"
```

---

## How to Run

### 1. Clone Repository

```bash
git clone https://github.com/LINAPS5/InternPhoLX_D8-9_file-Guardian.git
cd InternPhoLX_D8-9_file-Guardian
```

หรือถ้าทำในเครื่องอยู่แล้ว ให้เข้าโฟลเดอร์โปรเจกต์

```bash
cd ~/day8_9/file-guardian
```

---

### 2. Create Test Folder

```bash
mkdir -p test_folder
echo "hello config" > test_folder/config.txt
echo "my note" > test_folder/notes.txt
echo "important data" > test_folder/data.txt
```

---

### 3. Scan Folder

```bash
cargo run -- scan ./test_folder
```

โปรแกรมจะถาม password

```text
Enter baseline password:
```

ตัวอย่าง output

```text
Scanning folder: ./test_folder
[OK] config.txt
[OK] data.txt
[OK] notes.txt

Encrypted baseline saved to baseline.enc
```

หลังจากรันคำสั่งนี้ จะได้ไฟล์

```text
baseline.enc
```

---

### 4. Modify Files for Testing

ใช้คำสั่งนี้เพื่อจำลองว่าไฟล์ถูกเปลี่ยนแปลง

```bash
echo "hacked note" > test_folder/notes.txt
echo "new secret file" > test_folder/secret.txt
rm test_folder/config.txt
```

สิ่งที่เกิดขึ้นคือ

```text
notes.txt   ถูกแก้ไข
secret.txt  เป็นไฟล์ใหม่
config.txt  ถูกลบ
```

---

### 5. Check Folder

```bash
cargo run -- check ./test_folder
```

โปรแกรมจะถาม password เดิม

```text
Enter baseline password:
```

ตัวอย่าง output

```text
Checking folder: ./test_folder
[DELETED] config.txt
[UNCHANGED] data.txt
[MODIFIED] notes.txt
[NEW] secret.txt

Report saved to report.txt
```

หลังจากรันคำสั่งนี้ จะได้ไฟล์

```text
report.txt
```

---

## Example Report

ตัวอย่างข้อมูลใน `report.txt`

```text
===== File Guardian Report =====

[DELETED] config.txt
[UNCHANGED] data.txt
[MODIFIED] notes.txt
[NEW] secret.txt

===== Summary =====
Unchanged: 1
Modified: 1
New: 1
Deleted: 1
```

---

## Input / Output

### Input

โปรแกรมรับ input ผ่าน command line

```bash
cargo run -- scan ./test_folder
cargo run -- check ./test_folder
```

และรับ password จากผู้ใช้

```text
Enter baseline password:
```

---

### Output

โปรแกรมแสดงผลผ่าน terminal

```text
[UNCHANGED] data.txt
[MODIFIED] notes.txt
[NEW] secret.txt
[DELETED] config.txt
```

และสร้างไฟล์ output

```text
baseline.enc
report.txt
```

---

## Error Handling Examples

### 1. ไม่ใส่ command หรือ folder path

```bash
cargo run
```

Output

```text
Usage:
  cargo run -- scan <folder_path>
  cargo run -- check <folder_path>
Error: Missing command or folder path
```

---

### 2. ใส่ command ผิด

```bash
cargo run -- test ./test_folder
```

Output

```text
Usage:
  cargo run -- scan <folder_path>
  cargo run -- check <folder_path>
Error: Unknown command: test
```

---

### 3. Folder ไม่มีอยู่จริง

```bash
cargo run -- scan ./not_found
```

Output

```text
Error: Folder not found: ./not_found
```

---

### 4. Path ไม่ใช่ Folder

```bash
cargo run -- scan ./Cargo.toml
```

Output

```text
Error: Path is not a folder: ./Cargo.toml
```

---

### 5. Check ก่อน Scan

ถ้ายังไม่มีไฟล์ `baseline.enc` แล้วรัน

```bash
cargo run -- check ./test_folder
```

Output

```text
Error: baseline.enc not found. Please run scan first.
```

---

### 6. ใส่ Password ผิด

```bash
cargo run -- check ./test_folder
```

Output

```text
Error: Wrong password or invalid baseline file
```

---

## Design Decision

ผมเลือกทำโปรเจกต์ File Guardian เพราะเป็น CLI Tool ที่เกี่ยวข้องกับ Security โดยตรง ใช้สำหรับตรวจสอบความสมบูรณ์ของไฟล์ในโฟลเดอร์

หลักการทำงานคือ ครั้งแรกโปรแกรมจะสแกนไฟล์ทั้งหมดในโฟลเดอร์ แล้วคำนวณค่า SHA-256 hash ของแต่ละไฟล์ จากนั้นบันทึกค่าเหล่านี้เป็น baseline

เมื่อรันคำสั่ง check โปรแกรมจะคำนวณ hash ของไฟล์ปัจจุบัน แล้วนำไปเปรียบเทียบกับ baseline เดิม เพื่อดูว่าไฟล์แต่ละไฟล์อยู่ในสถานะ unchanged, modified, new หรือ deleted

ผมเลือกใช้ SHA-256 เพราะเหมาะกับการสร้าง fingerprint ของไฟล์ ถ้าเนื้อหาไฟล์เปลี่ยน ค่า hash ก็จะเปลี่ยนตาม ทำให้สามารถตรวจจับการแก้ไขไฟล์ได้

ผมเลือกเก็บ baseline เป็นไฟล์ `baseline.enc` แบบเข้ารหัส เพราะ baseline มีข้อมูลชื่อไฟล์และค่า hash หากเปิดอ่านได้โดยตรง อาจเปิดเผยโครงสร้างไฟล์ของระบบได้

โปรแกรมทำงานผ่าน CLI เพราะใช้งานง่าย ทดสอบง่าย และเหมาะกับเครื่องมือด้าน Security หรือ Automation

สำหรับระบบเข้ารหัส baseline ในโปรเจกต์นี้ ใช้วิธี XOR กับ key ที่สร้างจาก password เพื่อให้โค้ดเข้าใจง่าย แต่ถ้าใช้งานจริงควรใช้ encryption library ที่ปลอดภัยกว่า เช่น AES-GCM

---

## Demo Steps

### Step 1: สร้างไฟล์ตัวอย่าง

```bash
mkdir -p test_folder
echo "hello config" > test_folder/config.txt
echo "my note" > test_folder/notes.txt
echo "important data" > test_folder/data.txt
```

### Step 2: สร้าง baseline

```bash
cargo run -- scan ./test_folder
```

ใส่ password เช่น

```text
1234
```

### Step 3: เปลี่ยนแปลงไฟล์

```bash
echo "hacked note" > test_folder/notes.txt
echo "new secret file" > test_folder/secret.txt
rm test_folder/config.txt
```

### Step 4: ตรวจสอบไฟล์

```bash
cargo run -- check ./test_folder
```

ใส่ password เดิม

```text
1234
```

### Step 5: ดู report

```bash
cat report.txt
```

---

## Demo Commands

สามารถคัดลอกไปรันได้เลย

```bash
cd ~/day8_9/file-guardian

mkdir -p test_folder

echo "hello config" > test_folder/config.txt
echo "my note" > test_folder/notes.txt
echo "important data" > test_folder/data.txt

cargo run -- scan ./test_folder

echo "hacked note" > test_folder/notes.txt
echo "new secret file" > test_folder/secret.txt
rm test_folder/config.txt

cargo run -- check ./test_folder

cat report.txt


https://github.com/LINAPS5/InternPhoLX_D8-9_file-Guardian


---

## Git Commands

```bash
git init
git add .
git commit -m "first commit"
git branch -M main
git remote add origin https://github.com/LINAPS5/InternPhoLX_D8-9_file-Guardian.git
git push -u origin main
```

ถ้ามี remote origin อยู่แล้ว ให้ใช้

```bash
git remote set-url origin https://github.com/LINAPS5/InternPhoLX_D8-9_file-Guardian.git
git push -u origin main
```

---

## .gitignore

```gitignore
/target
baseline.enc
report.txt
```

---

## Summary

```text
Project Name: File Guardian
Language: Rust
Type: Security CLI Tool
Main Features: scan, check, encrypted baseline, report
Input: command + folder path + password
Output: terminal result + baseline.enc + report.txt
```

---

## Author

```text
InternPhoLX D8-9
```
