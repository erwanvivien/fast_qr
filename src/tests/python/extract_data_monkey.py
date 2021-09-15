import requests
import json

value = "salut"


def qr_money():
    url = "https://api.qrcode-monkey.com//qr/custom"

    headers = {
        "Content-Type": "text/plain",
        "origin": "https://www.qrcode-monkey.com",
        "authority": "api.qrcode-monkey.com"
    }

    data = {
        "data": f"{value}",
        "config": {
            "body": "square",
        },
        "size": 500,
        "download": "imageUrl",
        "file": "png"
    }

    r = requests.post(url, headers=headers, data=json.dumps(data))
    if r.status_code != 200:
        return "failure"
    return "https://" + r.json()["imageUrl"][2:]


print(qr_money())
