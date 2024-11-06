การคำนวณค่า SHA-1 ของหลาย ๆ ไฟล์พร้อมกันและต้องการได้ค่า SHA-1 รวม ของไฟล์ทั้งหมดในโฟลเดอร์ภายในครั้งเดียวทำได้โดยการรวมเนื้อหาของไฟล์ทั้งหมดเข้าด้วยกันเป็นไฟล์ชั่วคราว แล้วคำนวณค่า SHA-1 จากไฟล์รวมนี้ หรือคำนวณค่า SHA-1 รวม จากไฟล์ทั้งหมดโดยใช้ PowerShell สร้างการ hash ในครั้งเดียว

นี่คือวิธีการทำให้ได้ค่า SHA-1 เดียวที่คำนวณจากหลายไฟล์ในโฟลเดอร์โดยใช้ PowerShell:

# วิธีใช้ PowerShell ให้ได้ค่า SHA-1 รวมของไฟล์ทั้งหมดในโฟลเดอร์

1. เปิด PowerShell ด้วยสิทธิ์ Administrator
2. ใช้คำสั่งต่อไปนี้:
```
# ตั้งค่าโฟลเดอร์ที่ต้องการ
$folderPath = "C:\Game All\Beta\Kuro Game Beta\Wuthering Waves(Beta)\Wuthering Waves Game\Client\Saved\Resources\1.4.0\Resource"

# สร้างออบเจกต์ SHA1
$sha1 = [System.Security.Cryptography.SHA1CryptoServiceProvider]::Create()

# รวมเนื้อหาไฟล์ทั้งหมดเข้าด้วยกัน
$combinedHash = [System.Text.StringBuilder]::new()
Get-ChildItem -Path $folderPath -File | ForEach-Object {
    $fileStream = [System.IO.File]::OpenRead($_.FullName)
    $hashBytes = $sha1.ComputeHash($fileStream)
    $fileStream.Close()
    $combinedHash.Append([BitConverter]::ToString($hashBytes) -replace "-", "")
}

# คำนวณค่า SHA-1 ของเนื้อหาทั้งหมดรวมกัน
$finalHash = $sha1.ComputeHash([System.Text.Encoding]::UTF8.GetBytes($combinedHash.ToString()))
$finalHashString = [BitConverter]::ToString($finalHash) -replace "-", ""

Write-Output "SHA-1 รวมของไฟล์ทั้งหมดคือ: $finalHashString"
```

# อธิบายการทำงานของโค้ด
- สร้างออบเจกต์ $sha1 เพื่อใช้คำนวณค่า SHA-1
- วนลูปผ่านไฟล์ทั้งหมด ในโฟลเดอร์ที่ระบุไว้ใน $folderPath
- อ่านและคำนวณค่า SHA-1 ของแต่ละไฟล์ จากนั้นรวมค่า Hash ของแต่ละไฟล์เข้าด้วยกันเป็นข้อความเดียว
- คำนวณค่า SHA-1 จากข้อความรวม เพื่อสร้างค่า Hash เดียวที่ครอบคลุมทุกไฟล์ในโฟลเดอร์
