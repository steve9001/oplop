# Weaknesses of Oplop v1 algorithm

- 8 character length is too short
- does not guarantee upper, lower, symbol
- method of guaranteeing digit is problematic
- use of MD5 is distracting/not best practice/possibly vulnerable to brute force of master password

# Proposed Oplop v2 algorithm

- 12 character length
- guarantee upper, lower, digit, symbol, without loss of entropy
- replace md5 with PBKDF such as bcrypt or scrypt

For inputs of nickname and master password

let b = bcrypt(password, nickname)
// or let b = bcrypt(hash(password), hash(nickname))
let base = b[0..11]

select four random* unique values from 0 to 11 (indexes into base)
for each index set the value of base at that index to a random* upper, lower, digit, or symbol

random* values are deterministically derived from b[12..15]
for example for random uppercase, convert b[12] to integer and mod 26 to get an index into [ABC...XYZ]
for random digit convert b[14] into integer and mod 10

# Not proposed for Oplop v2

- Anything that requires remembering or recording additional information for each nickname, such as ability to specify a length or salt

# References

https://github.com/brettcannon/oplop
https://crypto.stackexchange.com/questions/3489/do-md5s-weaknesses-affect-oplop

