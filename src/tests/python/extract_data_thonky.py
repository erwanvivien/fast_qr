from PIL import Image
import wget
import time

import os
import sys


class RedirectStdStreams(object):
    def __init__(self, stdout=None, stderr=None):
        self._stdout = stdout or sys.stdout
        self._stderr = stderr or sys.stderr

    def __enter__(self):
        self.old_stdout, self.old_stderr = sys.stdout, sys.stderr
        self.old_stdout.flush()
        self.old_stderr.flush()
        sys.stdout, sys.stderr = self._stdout, self._stderr

    def __exit__(self, exc_type, exc_value, traceback):
        self._stdout.flush()
        self._stderr.flush()
        sys.stdout = self.old_stdout
        sys.stderr = self.old_stderr


text = "HELLO WORLD"
quality = 'q'
url = f"https://www.thonky.com/qr-encoder/encoder.php?s=${text.replace(' ', '%20')}&ec={quality}"

devnull = open(os.devnull, 'w')
with RedirectStdStreams(stdout=devnull, stderr=devnull):
    image_filename = wget.download(url)
devnull.close()

img = Image.open(image_filename)
px = img.load()
for i in range(64):
    # print(px[i, i])
    if px[i, i] == (0, 0, 0):
        break

module_size = i // 4
qr_code_side = img.height // module_size - 8
version = (qr_code_side - 17) // 4

# print(f"{module_size = }")
# print(f"{qr_code_side = }")
print(f"{version = }")


start = 4 * module_size + module_size // 2

qr = [([False] * qr_code_side) for _ in range(qr_code_side)]

y = start
for i in range(qr_code_side):
    x = start
    for j in range(qr_code_side):
        if px[x, y] == (0, 0, 0) or px[x, y] == (0, 0, 0, 255):
            qr[i][j] = True

        x += module_size
    y += module_size


print(f"{text = }")
print(str(qr).replace("True", "true").replace("False", "false"))


time.sleep(0.1)
os.remove(image_filename)
