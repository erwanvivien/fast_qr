import requests
import re
import time
import random

values = list(range(7, 30))

for nb in values:
    url = f"https://www.thonky.com/qr-code-tutorial/generator-polynomial-tool?degree={nb}"
    r = requests.get(url, allow_redirects=True)

    pattern1 = re.compile(r'<div id="poly-container">(\n|.)*<div itemscope')
    pattern2 = re.compile(r'<p>(\n|.)*</p>')

    for match in re.finditer(pattern1, r.text):
        text = match.group(0)
        for m in re.finditer(pattern2, text):
            content = m.group(0)
            content = content.replace("<sup>", "")
            content = content.replace("</sup>", "")
            content = content.replace("</p>", "")
            content = content.split("\n")[1]
            content = content.replace("&alpha;", "α")

            print(f"""
#[test]
fn generator_polynomial_{nb}() {{
    let poly = crate::polynomials::generator({nb});
    let poly_string = crate::polynomials::generated_to_string(&poly);
    assert_eq!(
        poly_string,
        "{content}"
    )
}}
""")
    # time.sleep(0.1)
