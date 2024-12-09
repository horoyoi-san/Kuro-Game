import requests
import time
import json

# กำหนด Webhook URLs
webhook_urls = {
    "Teat": 'https://discord.com/api/webhooks/1291725154937999444/CeBZotZNDREE7KM7mFx7DJ--Z2TD8tKKmfgZ8gqPUrLs2Bs2rALXjm6HPqv_VKNxGfQJ',
    "OMG Leak": 'https://discord.com/api/webhooks/1288821857469988965/zLuca-BLv_K1OwlSDh-zPhKw4W8yhI13uNHJkbVY-wG8I4aEG0wIF8m2RyGWPV5BT41M',
    "NotLeak": 'https://discord.com/api/webhooks/1290276844213506098/GUgxJKlfomvt90B_kgaZmOunWiAQ6qHkOc8BuvQXHenMyeJpA6PsyTfshHslY_VjUoJp'
}

# ตัวแปรสำหรับเก็บข้อมูลล่าสุด
last_data_1 = None
last_data_2 = None

# ฟังก์ชันในการตรวจสอบและส่งข้อความไปยัง Discord Webhook
def check_for_updates():
    global last_data_1, last_data_2

    url_1 = "https://prod-volcdn-gamestarter.kurogame.net/pcstarter/prod/starter/50013_HiDX7UaJOXpKl3pigJwVxhg5z1wllus5/G153/index.json"
    url_2 = "https://prod-alicdn-gamestarter.kurogame.com/pcstarter/prod/game/G153/50013_HiDX7UaJOXpKl3pigJwVxhg5z1wllus5/index.json"

    # ตรวจสอบข้อมูลจาก URL ที่ 1
    response_1 = requests.get(url_1)
    if response_1.status_code == 200:
        data_1 = response_1.json()
        if data_1 != last_data_1:
            send_webhooks(data_1, url_1, "Wuthering Waves BETA OS (Starter)", last_data_1)
            last_data_1 = data_1
    else:
        print(f"ไม่สามารถดึงข้อมูลจาก URL 1: {response_1.status_code}, {response_1.text}")

    # ตรวจสอบข้อมูลจาก URL ที่ 2
    response_2 = requests.get(url_2)
    if response_2.status_code == 200:
        data_2 = response_2.json()
        if data_2 != last_data_2:
            send_webhooks(data_2, url_2, "Wuthering Waves BETA OS (Game)", last_data_2)
            last_data_2 = data_2
    else:
        print(f"ไม่สามารถดึงข้อมูลจาก URL 2: {response_2.status_code}, {response_2.text}")

# ฟังก์ชันในการส่งข้อมูลไปยัง Discord Webhook หลายอัน
def send_webhooks(data, url, title, last_data):
    for webhook_key in webhook_urls:
        send_webhook(data, url, title, webhook_key, last_data)

# ฟังก์ชันในการส่งข้อมูลไปยัง Discord Webhook หนึ่งอัน
def send_webhook(data, url, title, webhook_key, last_data):
    embed_fields = []

    # คำนวณการเปลี่ยนแปลงของ "p"
    current_p = data["default"].get("p", None)
    last_p = last_data["default"].get("p", None) if last_data else None

    # แสดง diff ถ้ามีการเปลี่ยนแปลง
    if current_p and current_p != last_p:
        embed_fields.append({
            "name": "Diff:",
            "value": f"```diff\n- P: {last_p}\n+ P: {current_p}\n```" if last_p else f"```diff\n+ P: {current_p}\n```",
            "inline": False
        })

    # ข้อมูลเพิ่มเติม
    embed_fields.extend([
        {
            "name": "Version",
            "value": data["default"].get("version", "No data"),
            "inline": True
        },
        {
            "name": "Installer",
            "value": json.dumps(data["default"].get("installer", "No data"), ensure_ascii=False),
            "inline": False
        },
        {
            "name": "Resources",
            "value": json.dumps(data["default"].get("resources", "No data"), ensure_ascii=False),
            "inline": False
        }
    ])

    # ส่งข้อมูลไปยัง Webhook
    webhook_data = {
        "embeds": [
            {
                "title": title,
                "description": f"{url}",  # แสดงลิงก์เท่านั้น
                "color": 16771840,  # https://convertingcolors.com/decimal-color-16711680.html?search=Decimal(16711680)
                "fields": embed_fields,
                "image": {
                    "url": "https://cdn.discordapp.com/attachments/1292097230924283965/1312381157286871040/2.1.png?ex=674c49b2&is=674af832&hm=930d613cd8f5b73646d753618bafa5dc048d9c6d8d91de379b5467ac3bcc9297&"  # เพิ่มรูปภาพที่ด้านล่าง
                }
            }
        ]
    }

    # เพิ่มรูปภาพที่ด้านบนขวา
    webhook_data["embeds"][0]["thumbnail"] = {
        "url": "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcQkmsLi-PweF4K3vppsBMmbrQ2zFikTpYHdNg&s"  # เพิ่มลิงก์รูปภาพที่ด้านบนขวา
    }

    webhook_url = webhook_urls.get(webhook_key)
    if webhook_url:
        response = requests.post(webhook_url, json=webhook_data)
        if response.status_code == 204:
            print(f"ส่งข้อความ {title} ไปยัง Discord ({webhook_key}) เรียบร้อยแล้ว!")
        else:
            print(f"ไม่สามารถส่งข้อความ {title} ได้ที่ Webhook {webhook_key}: {response.status_code}, {response.text}")

# ตรวจสอบข้อมูลทุก 60 วินาที
while True:
    check_for_updates()
    time.sleep(1)
