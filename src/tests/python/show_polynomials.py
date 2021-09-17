import requests
import re
import random


def generate_test(seed=10):
    random.seed(seed)

    len_coeffs = random.randint(1, 50)
    coeffs = [random.randint(0, 255) for _ in range(len_coeffs)]
    msg_coeff = '%2C'.join([str(e) for e in coeffs])

    version = random.randint(7, 30)
    rdm = random.randint(0, 1000000000)

    url = f"https://www.thonky.com/qr-code-tutorial/show-division-steps?msg_coeff={msg_coeff}&num_ecc_blocks={version}"
    r = requests.get(url)

    pattern = re.compile(r'<p>Discard the lead 0 term to get:</p>(.*)<h4')
    text = r.text

    last = []
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

        last = content
        # print(f"[{content}]")

    for _ in range(last.count(','), version - 1):
        last += ", 0"

    return (f"""
#[test]
fn error_code_computation_{rdm}() {{
    let tmp1 = Vec::from({coeffs});
    let tmp2 = crate::polynomials::GENERATOR_POLYNOMIALS[{version}];
    let div = crate::polynomials::division(&tmp1, &tmp2);
    assert_eq!(
        div,
        Vec::from([{last}])
    )
}}
""")


for i in range(200, 300):
    res = generate_test(i)
    if res:
        print(res)
