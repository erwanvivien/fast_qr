import requests
import re

coeffs= [32,179,11,120,209,114,0,102,179,176,166,154,54,98,155,231,100,0,236,17,236,17,236,17,236,17,236,17,236,17,236,17,236,17,236,17,236,17,236,17,236,17,236,17,236,17,236,17]
coeffs = [str(e) for e in coeffs]
msg_coeff = '%2C'.join(coeffs)

version = 15

url = f"https://www.thonky.com/qr-code-tutorial/show-division-steps?msg_coeff={msg_coeff}&num_ecc_blocks={version}"
r = requests.get(url)

pattern = re.compile(r'<p>Discard the lead 0 term to get:</p>(.*)<h4')
text = r.text

for match in re.finditer(pattern, text):
    content = match.group(1)
    content = content.replace("<p></p>", "")
    # content = content.replace(r"x<sup>[0-9]*</sup>", "")

    content = re.sub(r"x<sup>[0-9]*</sup>", "", content)
    content = content.replace("<br /></p>", "")
    content = content.replace("<p>", "")
    content = content.replace(" +", ",")

    if content.find("discard-remaining-lead-0-terms") != -1:
        idx = content.index("</p></p>")
        content = content[idx + 8:]

    print(f"[{content}]")
