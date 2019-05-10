require 'openssl'

h = OpenSSL::KDF.pbkdf2_hmac "pass", salt: "salt", iterations: 1_000_000, length: 24, hash: 'sha256'

puts h.bytes.inspect
