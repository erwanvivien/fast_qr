import requests
import re
import random
import ast

headers = {
    'user-agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/75.0.3770.142 Safari/537.36'
}


def generate_test(seed=10, coeffs=None, version=None):
    random.seed(seed)

    len_coeffs = random.randint(1, 50)
    coeffs = [random.randint(0, 255) for _ in range(
        len_coeffs)] if coeffs is None else coeffs
    msg_coeff = '%2C'.join([str(e) for e in coeffs])

    version = random.randint(7, 30) if version is None else version

    url = f"https://www.thonky.com/qr-code-tutorial/show-division-steps?msg_coeff={msg_coeff}&num_ecc_blocks={version}"
    r = requests.get(url, headers=headers)

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
            content = content[idx + 8:] + ", 0"

        last = content
        # print(f"[{content}]")

    res = ast.literal_eval(f'[{last}]')
    print(len(res))
    if len(res) != version:
        res = [0] * (version - len(res)) + res

    return (f"""
#[test]
fn error_code_computation_seed_{seed}() {{
    let tmp1 = Vec::from({coeffs});
    let tmp2 = crate::polynomials::GENERATOR_POLYNOMIALS[{version}];
    let div = crate::polynomials::division(&tmp1, &tmp2);
    assert_eq!(
        div,
        Vec::from({res})
    )
}}
""")


if __name__ == '__main__':
    # for i in range(450, 500):
    res = generate_test(
        0, coeffs=[35, 37, 251, 189, 8, 169, 15, 34, 59, 137, 187, 114, 134], version=26)
    if res:
        print(res)
