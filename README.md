# SHA-1 checksum of files for Wuthering Waves (beta) game will be applied to the Wuthering Waves project.
https://github.com/horoyoi-san/Kuro-Game/blob/Wutheing-waves-1.4-BETA/assets/hotpatch/prod/Windows/config.json


การตรวจสอบค่า SHA-1 ของไฟล์สำหรับเกม Wuthering Waves (Beta) หรือไฟล์ใด ๆ บน Windows โดยใช้คำสั่ง CertUtil ทำตามขั้นตอนดังนี้:
1. เปิด Command Prompt:
 - กดปุ่ม Windows + R บนคีย์บอร์ด จากนั้นพิมพ์ cmd แล้วกด Enter
2. ใช้คำสั่ง CertUtil เพื่อคำนวณค่า SHA-1 ของไฟล์:

```
CertUtil -hashfile "CertUtil -hashfile "C:\Wuthering Waves(Beta)\Wuthering Waves Game\Wuthering Waves.exe" SHA1" SHA1
```
แทนที่ ```C:\Wuthering Waves(Beta)\Wuthering Waves Game\Wuthering Waves.exe``` ด้วยที่อยู่ของไฟล์ที่ต้องการตรวจสอบ SHA-1
3. ระบบจะแสดงค่า SHA-1 ของไฟล์ในรูปแบบของชุดตัวอักษรและตัวเลขยาว ๆ ซึ่งสามารถใช้ตรวจสอบว่าไฟล์ดังกล่าวตรงกับค่าใน JSON หรือไม่
![image](https://github.com/user-attachments/assets/43d446da-15b5-4ac5-b7f1-b5c65a9076c3)



---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
การคำนวณค่า SHA-1 ของหลาย ๆ ไฟล์พร้อมกันและต้องการได้ค่า SHA-1 รวม ของไฟล์ทั้งหมดในโฟลเดอร์ภายในครั้งเดียวทำได้โดยการรวมเนื้อหาของไฟล์ทั้งหมดเข้าด้วยกันเป็นไฟล์ชั่วคราว แล้วคำนวณค่า SHA-1 จากไฟล์รวมนี้ หรือคำนวณค่า SHA-1 รวม จากไฟล์ทั้งหมดโดยใช้ PowerShell สร้างการ hash ในครั้งเดียว

นี่คือวิธีการทำให้ได้ค่า SHA-1 เดียวที่คำนวณจากหลายไฟล์ในโฟลเดอร์โดยใช้ PowerShell:

# วิธีใช้ PowerShell ให้ได้ค่า SHA-1 รวมของไฟล์ทั้งหมดในโฟลเดอร์

1. เปิด PowerShell ด้วยสิทธิ์ Administrator
2. ใช้คำสั่งต่อไปนี้:
```
# ตั้งค่าโฟลเดอร์ที่ต้องการ
$folderPath = "C:\Wuthering Waves(Beta)\Wuthering Waves Game\Client\Saved\Resources\1.4.0\Resource"

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

![image](https://github.com/user-attachments/assets/f0d2dd4f-893c-4087-bcbc-d6b060e0453d)

--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

การหาค่า IndexSha1 สำหรับไฟล์ทั้งหมดภายใต้โฟลเดอร์:
```
C:\Wuthering Waves(Beta)\Wuthering Waves Game\Client\Saved\Resources\1.4.0
```
นี่คือขั้นตอนการคำนวณค่า IndexSha1 โดยรวมค่า SHA-1 ของไฟล์ทั้งหมดในโฟลเดอร์นั้น:
# การคำนวณค่า Index SHA-1 ของไฟล์ทั้งหมดในโฟลเดอร์โดยใช้ PowerShell
1. เปิด PowerShell ด้วยสิทธิ์ Administrator
2. ใช้คำสั่งนี้เพื่อคำนวณค่า SHA-1 รวม (IndexSha1) ของไฟล์ทั้งหมดภายใต้โฟลเดอร์:
```
# ตั้งค่าโฟลเดอร์ที่ต้องการ
$folderPath = "C:Wuthering Waves(Beta)\Wuthering Waves Game\Client\Saved\Resources\1.4.0\Resource_??"

# สร้างออบเจกต์ SHA1
$sha1 = [System.Security.Cryptography.SHA1CryptoServiceProvider]::Create()

# รวมค่า Hash ของไฟล์ทั้งหมดในโฟลเดอร์
$combinedHash = [System.Text.StringBuilder]::new()
Get-ChildItem -Path $folderPath -File -Recurse | Sort-Object FullName | ForEach-Object {
    $fileStream = [System.IO.File]::OpenRead($_.FullName)
    $hashBytes = $sha1.ComputeHash($fileStream)
    $fileStream.Close()
    $combinedHash.Append([BitConverter]::ToString($hashBytes) -replace "-", "")
}

# คำนวณค่า SHA-1 จากข้อมูลรวมทั้งหมดเพื่อสร้าง Index SHA-1
$finalHash = $sha1.ComputeHash([System.Text.Encoding]::UTF8.GetBytes($combinedHash.ToString()))
$finalHashString = [BitConverter]::ToString($finalHash) -replace "-", ""

Write-Output "ค่า Index SHA-1 ของโฟลเดอร์คือ: $finalHashString"

```
# อธิบายการทำงานของโค้ด
 - ตั้งค่า $folderPath เป็นโฟลเดอร์ที่ต้องการคำนวณค่า Index SHA-1
 - วนลูปผ่านไฟล์ทั้งหมดในโฟลเดอร์ ด้วย -Recurse เพื่อลงลึกไปยังทุกไฟล์และโฟลเดอร์ย่อย
 - คำนวณค่า SHA-1 ของแต่ละไฟล์ และรวมค่า SHA-1 ของแต่ละไฟล์เข้าด้วยกันใน $combinedHash
 - คำนวณค่า SHA-1 ของ $combinedHash ที่สร้างขึ้น ซึ่งจะได้ค่า Index SHA-1 ของไฟล์ทั้งหมดในโฟลเดอร์
![image](https://github.com/user-attachments/assets/dcee038b-cb5a-4263-b542-1ef01ff8c68e)
----------------------------------------------------------------------------------------------------------------
การหาหมายเลข ChangeList หรือ Changelog ID ที่เกี่ยวข้องกับการอัปเดตในเกมหรือแอปพลิเคชันนั้น มักจะขึ้นอยู่กับเครื่องมือและกระบวนการที่ใช้ในการพัฒนาและจัดการเวอร์ชันของโปรเจ็กต์ วิธีการหาหมายเลขนี้จะขึ้นอยู่กับว่าคุณใช้ ระบบควบคุมเวอร์ชัน (Version Control) เช่น Git หรือ SVN หรือใช้ระบบ CI/CD (Continuous Integration/Continuous Deployment) ในการจัดการการอัปเดต
# ขั้นตอนการหาหมายเลข ChangeList
1. การใช้ระบบควบคุมเวอร์ชัน (เช่น Git)

 - หากคุณเป็นส่วนหนึ่งของทีมพัฒนาและใช้ระบบ Git, การหาหมายเลข ChangeList สามารถทำได้โดยการ ตรวจสอบการเปลี่ยนแปลงใน commit หรือ การ merge ใน Git ซึ่งแต่ละการเปลี่ยนแปลงจะมีหมายเลข commit hash ที่เป็นเอกลักษณ์
 - ในกรณีนี้หมายเลข ChangeList อาจจะถูกกำหนดเป็น commit ID หรือ revision ID
ตัวอย่างการหา ChangeList ใน Git:
**เปิด Git log เพื่อดูประวัติการ commit**
```
git log
```
หรือใช้คำสั่ง git log --oneline เพื่อแสดงรายการ commit สั้นๆ:
```
git log --oneline
```
จะได้ผลลัพธ์คล้ายกับ:
```
34f9ac3 Update game mechanics
3cc0321 Fix bug in character animation
9a7c123 Add new levels
```
จากนี้คุณสามารถใช้ commit ID เช่น 34f9ac3 หรือใช้ หมายเลขเวอร์ชันที่กำหนด ที่บ่งบอกการเปลี่ยนแปลงในแต่ละเวอร์ชันของเกม
# 2. การใช้ระบบ CI/CD (Continuous Integration/Continuous Deployment)

 - หากทีมพัฒนาของคุณใช้ระบบ CI/CD อย่าง Jenkins, GitLab CI, หรือ GitHub Actions, ในหลายกรณี หมายเลข ChangeList จะถูกสร้างขึ้นโดยอัตโนมัติในกระบวนการ build และ deploy ในแต่ละรอบการอัปเดต
 - หมายเลขนี้อาจจะมาจาก หมายเลข build หรือ หมายเลขเวอร์ชัน ที่ระบุว่าเป็นการอัปเดตชุดไหนจากการทำงานในรอบนั้นๆ
# ตัวอย่างการหาหมายเลข ChangeList ใน CI/CD:

 - หากใช้ Jenkins, หมายเลข Build ID หรือ Build Number จะสามารถใช้เป็นหมายเลข ChangeList ได้
 - ใน GitLab CI, คุณอาจจะดูจากหมายเลข Pipeline ID หรือ Commit SHA ที่ระบุถึงการเปลี่ยนแปลงในรอบการ build หรือ deployment
 - สำหรับ GitHub Actions, การตั้งค่า GITHUB_SHA หรือหมายเลข commit hash ก็สามารถใช้เป็น ChangeList
# 3. การใช้ระบบการจัดการโปรเจ็กต์ (เช่น Jira, Trello, Asana)

 - หากทีมพัฒนาของคุณใช้ระบบการจัดการโปรเจ็กต์เพื่อบันทึก issue หรือ task ในกระบวนการพัฒนา, หมายเลข ChangeList อาจจะมาจาก หมายเลข issue หรือ หมายเลข task ที่เกี่ยวข้องกับการอัปเดตหรือฟีเจอร์ที่ได้รับการพัฒนา
 - บางครั้ง ChangeList อาจจะถูกตั้งขึ้นจาก หมายเลขเวอร์ชันของการปล่อย หรือ หมายเลข release ที่ทีมพัฒนาทำการออกใหม่
# 4. การค้นหาจากข้อมูลการปล่อย (Release Notes)

 - ถ้าคุณไม่มีการเข้าถึงระบบที่ใช้ในการพัฒนา, คุณสามารถหาหมายเลข ChangeList จาก Release Notes หรือ Changelog ที่ทีมพัฒนาอาจจะปล่อยออกมา
 - Release Notes จะมักจะระบุรายละเอียดของการเปลี่ยนแปลงทั้งหมดในแต่ละเวอร์ชัน รวมถึงการเพิ่มฟีเจอร์ใหม่ๆ หรือการแก้ไขบั๊ก
 - บางครั้ง Release Notes จะระบุหมายเลข ChangeList หรือหมายเลข commit ID ของการอัปเดตนั้นๆ

# สรุปการหา ChangeList
การหาหมายเลข ChangeList จะต้อง:

 - ตรวจสอบในระบบควบคุมเวอร์ชัน (Git, SVN) หากคุณมีการเข้าถึงโค้ด
 - ตรวจสอบในระบบ CI/CD ถ้าใช้เครื่องมืออัตโนมัติในการ build และ deploy
 - ดูจาก Release Notes หรือ Changelog ถ้าไม่มีการเข้าถึงระบบควบคุมเวอร์ชัน
ถ้าคุณเป็นผู้พัฒนาหรือเจ้าของโปรเจ็กต์, คุณจะต้องตั้งระบบให้มีการ บันทึกหมายเลขการเปลี่ยนแปลง อย่างชัดเจนเพื่อติดตามและระบุการอัปเดตในแต่ละครั้ง.


