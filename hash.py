import hashlib

h = hashlib.pbkdf2_hmac('sha256', b'pass', b'salt', 1000000)
print(bytearray(h))
