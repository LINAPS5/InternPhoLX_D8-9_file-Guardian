// ===============================
// File Guardian
// โปรแกรมตรวจสอบความเปลี่ยนแปลงของไฟล์
//
// ความสามารถหลัก:
// 1. scan  = สแกนไฟล์ในโฟลเดอร์ แล้วสร้าง baseline.enc
// 2. check = ตรวจสอบว่าไฟล์ถูกแก้ไข เพิ่ม หรือลบหรือไม่
// 3. baseline.enc ถูกเข้ารหัสแบบง่ายด้วย password
// 4. สร้าง report.txt เพื่อเก็บผลลัพธ์
//
// วิธีรัน:
// cargo run -- scan ./test_folder
// cargo run -- check ./test_folder
// ===============================

use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

// ชื่อไฟล์ baseline ที่ใช้เก็บข้อมูล hash ของไฟล์
// ไฟล์นี้จะถูกเข้ารหัส ทำให้เปิดอ่านตรง ๆ ไม่ได้
const BASELINE_FILE: &str = "baseline.enc";

// ชื่อไฟล์ report ที่ใช้เก็บผลลัพธ์หลังจากตรวจสอบ
const REPORT_FILE: &str = "report.txt";

// ข้อความหัวไฟล์ ใช้เช็กว่าถอดรหัส baseline ถูกต้องหรือไม่
const BASELINE_HEADER: &str = "FILE_GUARDIAN_BASELINE_V1";

fn main() {
    // เริ่มโปรแกรมจากฟังก์ชัน run()
    // ถ้าเกิด error จะแสดงข้อความออกมาทางหน้าจอ
    if let Err(error) = run() {
        eprintln!("Error: {}", error);
    }
}

fn run() -> Result<(), String> {
    // รับ argument จาก command line
    // ตัวอย่าง:
    // cargo run -- scan ./test_folder
    //
    // args[0] = ชื่อโปรแกรม
    // args[1] = command เช่น scan หรือ check
    // args[2] = path ของ folder
    let args: Vec<String> = env::args().collect();

    // ถ้าผู้ใช้ใส่ argument ไม่ครบ ให้แสดงวิธีใช้งาน
    if args.len() < 3 {
        show_usage();
        return Err("Missing command or folder path".to_string());
    }

    let command = &args[1];
    let folder_path = Path::new(&args[2]);

    // ตรวจสอบว่า folder มีอยู่จริงไหม
    if !folder_path.exists() {
        return Err(format!("Folder not found: {}", folder_path.display()));
    }

    // ตรวจสอบว่า path ที่ใส่มาเป็น folder จริงไหม
    if !folder_path.is_dir() {
        return Err(format!("Path is not a folder: {}", folder_path.display()));
    }

    // เลือกทำงานตาม command ที่ผู้ใช้ใส่เข้ามา
    if command == "scan" {
        scan_folder(folder_path)?;
    } else if command == "check" {
        check_folder(folder_path)?;
    } else {
        show_usage();
        return Err(format!("Unknown command: {}", command));
    }

    Ok(())
}

fn show_usage() {
    // แสดงวิธีใช้งานโปรแกรม
    println!("Usage:");
    println!("  cargo run -- scan <folder_path>");
    println!("  cargo run -- check <folder_path>");
}

fn scan_folder(folder_path: &Path) -> Result<(), String> {
    // ฟังก์ชันนี้ใช้สำหรับสแกนไฟล์ทั้งหมดใน folder
    // แล้วสร้าง baseline.enc เพื่อเก็บค่า hash เริ่มต้น

    println!("Scanning folder: {}", folder_path.display());

    // อ่านไฟล์ทั้งหมดใน folder และ subfolder
    let files = get_all_files(folder_path)?;

    // ตัวแปรนี้ใช้เก็บข้อมูล baseline ก่อนนำไปเข้ารหัส
    let mut baseline_data = String::new();

    // ใส่ header ไว้บรรทัดแรก
    // ตอน check จะใช้ header นี้ตรวจสอบว่า password ถูกไหม
    baseline_data.push_str(BASELINE_HEADER);
    baseline_data.push('\n');

    // วนอ่านไฟล์ทีละไฟล์
    for file in files {
        // แปลง path ให้เป็น path แบบสั้น
        // เช่น test_folder/config.txt -> config.txt
        let relative_path = get_relative_path(folder_path, &file)?;

        // คำนวณ hash ของไฟล์
        let hash = calculate_file_hash(&file)?;

        println!("[OK] {}", relative_path);

        // เก็บข้อมูลในรูปแบบ:
        // path|hash
        baseline_data.push_str(&format!("{}|{}\n", relative_path, hash));
    }

    // ให้ผู้ใช้ใส่ password สำหรับเข้ารหัส baseline
    let password = ask_password()?;

    // เข้ารหัส baseline ด้วย password
    let encrypted_data = encrypt_text(&baseline_data, &password);

    // เขียนข้อมูลที่เข้ารหัสแล้วลงไฟล์ baseline.enc
    fs::write(BASELINE_FILE, encrypted_data)
        .map_err(|_| "Cannot write baseline.enc".to_string())?;

    println!();
    println!("Encrypted baseline saved to {}", BASELINE_FILE);

    Ok(())
}

fn check_folder(folder_path: &Path) -> Result<(), String> {
    // ฟังก์ชันนี้ใช้ตรวจสอบว่าไฟล์เปลี่ยนไปจาก baseline หรือไม่

    println!("Checking folder: {}", folder_path.display());

    // ถ้ายังไม่มี baseline.enc ให้แจ้ง error
    if !Path::new(BASELINE_FILE).exists() {
        return Err("baseline.enc not found. Please run scan first.".to_string());
    }

    // โหลด baseline เดิมจากไฟล์ baseline.enc
    // ในขั้นตอนนี้โปรแกรมจะถาม password เพื่อถอดรหัส
    let old_baseline = load_baseline()?;

    // อ่านไฟล์ปัจจุบันทั้งหมดใน folder
    let current_files = get_all_files(folder_path)?;

    // HashMap ใช้เก็บข้อมูลไฟล์ปัจจุบัน
    // key   = path ของไฟล์
    // value = hash ของไฟล์
    let mut current_baseline: HashMap<String, String> = HashMap::new();

    for file in current_files {
        let relative_path = get_relative_path(folder_path, &file)?;
        let hash = calculate_file_hash(&file)?;

        current_baseline.insert(relative_path, hash);
    }

    // ตัวแปร report ใช้เก็บข้อความที่จะเขียนลง report.txt
    let mut report = String::new();
    report.push_str("===== File Guardian Report =====\n\n");

    // ตัวนับผลลัพธ์แต่ละประเภท
    let mut unchanged_count = 0;
    let mut modified_count = 0;
    let mut new_count = 0;
    let mut deleted_count = 0;

    // HashSet ใช้จำว่าไฟล์ไหนตรวจไปแล้ว
    // เพื่อเอาไว้แยกไฟล์ใหม่ที่เพิ่มเข้ามา
    let mut checked_files: HashSet<String> = HashSet::new();

    // ตรวจไฟล์จาก baseline เดิม
    for (path, old_hash) in &old_baseline {
        checked_files.insert(path.clone());

        // เช็กว่าไฟล์เดิมยังมีอยู่ใน folder ปัจจุบันไหม
        if let Some(current_hash) = current_baseline.get(path) {
            // ถ้า hash เหมือนเดิม แปลว่าไฟล์ไม่เปลี่ยน
            if old_hash == current_hash {
                let line = format!("[UNCHANGED] {}\n", path);
                print!("{}", line);
                report.push_str(&line);
                unchanged_count += 1;
            } else {
                // ถ้า hash ไม่เหมือนเดิม แปลว่าไฟล์ถูกแก้ไข
                let line = format!("[MODIFIED] {}\n", path);
                print!("{}", line);
                report.push_str(&line);
                modified_count += 1;
            }
        } else {
            // ถ้าไฟล์เคยมีใน baseline แต่ตอนนี้ไม่มีแล้ว แปลว่าถูกลบ
            let line = format!("[DELETED] {}\n", path);
            print!("{}", line);
            report.push_str(&line);
            deleted_count += 1;
        }
    }

    // ตรวจหาไฟล์ใหม่
    // ถ้ามีไฟล์ใน current_baseline แต่ไม่เคยอยู่ใน checked_files
    // แปลว่าเป็นไฟล์ใหม่
    for path in current_baseline.keys() {
        if !checked_files.contains(path) {
            let line = format!("[NEW] {}\n", path);
            print!("{}", line);
            report.push_str(&line);
            new_count += 1;
        }
    }

    // เพิ่มสรุปผลลง report
    report.push_str("\n===== Summary =====\n");
    report.push_str(&format!("Unchanged: {}\n", unchanged_count));
    report.push_str(&format!("Modified: {}\n", modified_count));
    report.push_str(&format!("New: {}\n", new_count));
    report.push_str(&format!("Deleted: {}\n", deleted_count));

    // เขียน report ลงไฟล์ report.txt
    fs::write(REPORT_FILE, report)
        .map_err(|_| "Cannot write report.txt".to_string())?;

    println!();
    println!("Report saved to {}", REPORT_FILE);

    Ok(())
}

fn get_all_files(folder_path: &Path) -> Result<Vec<PathBuf>, String> {
    // ฟังก์ชันนี้ใช้รวมไฟล์ทั้งหมดใน folder
    // คืนค่าเป็น Vec<PathBuf>

    let mut files = Vec::new();

    read_folder(folder_path, &mut files)?;

    // เรียงไฟล์ เพื่อให้ผลลัพธ์ออกมาลำดับเดิมทุกครั้ง
    files.sort();

    Ok(files)
}

fn read_folder(folder_path: &Path, files: &mut Vec<PathBuf>) -> Result<(), String> {
    // ฟังก์ชันนี้อ่าน folder แบบ recursive
    // หมายถึงถ้าเจอ subfolder ก็จะเข้าไปอ่านต่อ

    let entries = fs::read_dir(folder_path)
        .map_err(|_| format!("Cannot read folder: {}", folder_path.display()))?;

    for entry in entries {
        let entry = entry.map_err(|_| "Cannot read folder item".to_string())?;
        let path = entry.path();

        if path.is_dir() {
            // ถ้าเป็น folder ให้เข้าไปอ่านต่อ
            read_folder(&path, files)?;
        } else if path.is_file() {
            // ถ้าเป็นไฟล์ ให้ตรวจชื่อไฟล์ก่อน
            let file_name = path
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("");

            // ไม่เอา baseline.enc และ report.txt มาคิด hash
            // เพราะเป็นไฟล์ที่โปรแกรมสร้างเอง
            if file_name != BASELINE_FILE && file_name != REPORT_FILE {
                files.push(path);
            }
        }
    }

    Ok(())
}

fn calculate_file_hash(file_path: &Path) -> Result<String, String> {
    // ฟังก์ชันนี้ใช้คำนวณ SHA-256 hash ของไฟล์
    //
    // ถ้าเนื้อหาไฟล์เปลี่ยน
    // ค่า hash ก็จะเปลี่ยน
    // จึงใช้ตรวจสอบความเปลี่ยนแปลงของไฟล์ได้

    let file_data = fs::read(file_path)
        .map_err(|_| format!("Cannot read file: {}", file_path.display()))?;

    let mut hasher = Sha256::new();

    // ส่งข้อมูลไฟล์เข้าไปคำนวณ hash
    hasher.update(file_data);

    let result = hasher.finalize();

    // แปลง hash เป็น string แบบ hex
    Ok(format!("{:x}", result))
}

fn get_relative_path(base_path: &Path, file_path: &Path) -> Result<String, String> {
    // ฟังก์ชันนี้ใช้ตัด path ด้านหน้าออก
    //
    // ตัวอย่าง:
    // base_path = ./test_folder
    // file_path = ./test_folder/config.txt
    // result    = config.txt

    let relative_path = file_path
        .strip_prefix(base_path)
        .map_err(|_| "Cannot create relative path".to_string())?;

    // replace("\\", "/") ทำให้ path ใช้ได้ทั้ง Windows และ Linux
    Ok(relative_path.to_string_lossy().replace("\\", "/"))
}

fn load_baseline() -> Result<HashMap<String, String>, String> {
    // ฟังก์ชันนี้ใช้โหลด baseline.enc
    // จากนั้นถอดรหัส แล้วแปลงกลับมาเป็น HashMap

    let password = ask_password()?;

    // อ่าน baseline.enc เป็น bytes
    let encrypted_data = fs::read(BASELINE_FILE)
        .map_err(|_| "Cannot read baseline.enc".to_string())?;

    // ถอดรหัสข้อมูลด้วย password
    let decrypted_text = decrypt_text(&encrypted_data, &password)?;

    let mut lines = decrypted_text.lines();

    // อ่านบรรทัดแรกเพื่อเช็ก header
    let header = lines.next().unwrap_or("");

    // ถ้า header ไม่ตรง แปลว่า password ผิด หรือไฟล์ baseline ไม่ถูกต้อง
    if header != BASELINE_HEADER {
        return Err("Wrong password or invalid baseline file".to_string());
    }

    let mut baseline = HashMap::new();

    // อ่านข้อมูลที่เหลือ
    // รูปแบบแต่ละบรรทัดคือ:
    // path|hash
    for line in lines {
        if let Some((path, hash)) = line.split_once('|') {
            baseline.insert(path.to_string(), hash.to_string());
        }
    }

    Ok(baseline)
}

fn ask_password() -> Result<String, String> {
    // ฟังก์ชันนี้ใช้รับ password จากผู้ใช้
    // หมายเหตุ: เวอร์ชันง่ายนี้ password จะมองเห็นตอนพิมพ์
    // ถ้าต้องการซ่อน password ต้องใช้ crate rpassword เพิ่ม

    print!("Enter baseline password: ");

    // flush ใช้บังคับให้ข้อความด้านบนแสดงก่อนรับ input
    io::stdout()
        .flush()
        .map_err(|_| "Cannot show password input".to_string())?;

    let mut password = String::new();

    io::stdin()
        .read_line(&mut password)
        .map_err(|_| "Cannot read password".to_string())?;

    let password = password.trim().to_string();

    if password.is_empty() {
        return Err("Password cannot be empty".to_string());
    }

    Ok(password)
}

fn create_key(password: &str) -> Vec<u8> {
    // ฟังก์ชันนี้ใช้สร้าง key จาก password
    // โดยนำ password ไป hash ด้วย SHA-256
    //
    // ผลลัพธ์จะได้ key เป็น bytes
    // แล้วนำ key นี้ไปใช้ XOR กับข้อมูล baseline

    let mut hasher = Sha256::new();

    hasher.update(password.as_bytes());

    let result = hasher.finalize();

    result.to_vec()
}

fn encrypt_text(text: &str, password: &str) -> Vec<u8> {
    // ฟังก์ชันเข้ารหัสข้อความ
    //
    // ขั้นตอน:
    // 1. สร้าง key จาก password
    // 2. แปลงข้อความเป็น bytes
    // 3. นำข้อมูลไป XOR กับ key

    let key = create_key(password);
    let data = text.as_bytes();

    xor_data(data, &key)
}

fn decrypt_text(encrypted_data: &[u8], password: &str) -> Result<String, String> {
    // ฟังก์ชันถอดรหัส
    //
    // XOR มีคุณสมบัติว่า:
    // encrypt และ decrypt ใช้ฟังก์ชันเดียวกันได้
    //
    // data XOR key = encrypted
    // encrypted XOR key = data เดิม

    let key = create_key(password);
    let decrypted_data = xor_data(encrypted_data, &key);

    String::from_utf8(decrypted_data)
        .map_err(|_| "Wrong password or corrupted baseline file".to_string())
}

fn xor_data(data: &[u8], key: &[u8]) -> Vec<u8> {
    // ฟังก์ชัน XOR ข้อมูลกับ key
    //
    // data คือข้อมูลที่ต้องการเข้ารหัส/ถอดรหัส
    // key คือรหัสที่สร้างจาก password
    //
    // ถ้า key สั้นกว่าข้อมูล จะวนใช้ key ซ้ำด้วย %
    //
    // หมายเหตุ:
    // วิธีนี้เหมาะสำหรับโปรเจกต์ฝึกเรียนรู้
    // แต่ถ้าใช้งานจริงควรใช้ AES-GCM หรือ encryption library ที่ปลอดภัยกว่า

    let mut result = Vec::new();

    for i in 0..data.len() {
        let data_byte = data[i];
        let key_byte = key[i % key.len()];

        result.push(data_byte ^ key_byte);
    }

    result
}
