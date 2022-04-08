import requests

url = "https://api-free.deepl.com/v2/translate"
headers = {
    'Host': 'api-free.deepl.com',
    'Content-Type': 'application/x-www-form-urlencoded',
}
#headers["Host"] = "api-free.deepl.com"
#headers["Content-Type"] = "application/x-www-form-urlencoded"

#data = """
# {
#    "auth_key=819ee1f7-f31b-1bc8-03a6-5e5a33b7bbd5:fx&text=Hello, world&target_lang=DE"
# }
#"""

payload = """
{
  "auth_key": "819ee1f7-f31b-1bc8-03a6-5e5a33b7bbd5:fx",
  "text": "Hello",
  "target_lang": "JA"
}
"""

resp = requests.post(url, headers=headers, data=payload)
print(resp.text)
print(resp.status_code)

#> POST /v2/translate HTTP/2
#> Host: api-free.deepl.com
#> User-Agent: curl/7.64.1
#> Accept: */*
#> Content-Length: 82
#> Content-Type: application/x-www-form-urlencoded
