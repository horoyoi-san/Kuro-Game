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
----------------------------------------------------------------------------------------------------------------------------------
# การใช้สคริปต์ Python ที่ใช้ในการดึงค่า ChangeList จากไฟล์ index.json หรือ resource.json:
```
import requests

# URL สำหรับไฟล์ index.json หรือ resource.json
index_json_url = "https://prod-cn-alicdn-gamestarter.kurogame.com/pcstarter/prod/game/G152/10008_Pa0Q0EMFxukjEqX33pF9Uyvdc8MaGPSz/index.json"
resource_json_url = "https://pcdownload-huoshan.aki-game.com/pcstarter/prod/game/G152/1.4.0/iQPhVvIx0vVjUoykmdLNMSHN0sfNWeij/resource.json"

# ฟังก์ชันในการโหลดไฟล์ JSON และดึงข้อมูล ChangeList
def get_changelist_from_json(url):
    response = requests.get(url)
    if response.status_code == 200:
        data = response.json()
        # ตรวจสอบว่าในไฟล์ JSON มีคีย์ 'ChangeList' หรือไม่
        changelist = data.get('ChangeList')
        if changelist:
            return changelist
        else:
            print("ไม่พบค่า 'ChangeList' ในไฟล์ JSON")
    else:
        print("ไม่สามารถโหลดไฟล์ JSON ได้")

# ทดสอบด้วย index.json หรือ resource.json
changelist = get_changelist_from_json(index_json_url)
if changelist:
    print(f"ค่า ChangeList จาก index.json: {changelist}")

# หากต้องการทดสอบ resource.json
# changelist = get_changelist_from_json(resource_json_url)
# if changelist:
#     print(f"ค่า ChangeList จาก resource.json: {changelist}")

```
ผลลัพธ์ที่คาดว่าจะได้:
```
# Output ที่คาดว่าจะได้:
ค่า ChangeList จาก index.json: 2585778

```
# สรุป:
ผลลัพธ์ที่ได้จากการรันสคริปต์ Python ข้างต้นคือ ค่า ChangeList ที่อยู่ในไฟล์ index.json หรือ resource.json ที่เราใช้ดึงข้อมูล
จากตัวอย่างนี้, ค่าของ ChangeList คือ 2585778 ซึ่งแสดงถึงเวอร์ชันของเกมหรือการอัปเดตล่าสุดที่คุณจะต้องใช้ในการอัปเดต config.json หรือระบบอื่น ๆ


