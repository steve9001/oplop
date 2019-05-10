# Weaknesses of Oplop v1 algorithm

- 8 character length is too short
- does not guarantee upper, lower, symbol
- method of guaranteeing digit is problematic
- use of MD5 is distracting/not best practice/possibly vulnerable to brute force of master password

# Proposed Oplop v2 algorithm

Features:

- 12 character length
- guarantee upper, lower, digit and symbol for password policy
- replace md5 with PBKDF2, bcrypt, scrypt, or Argon2
- same user experience as v1, except for a small delay

- PBKDF2 with HMAC-SHA-256
 - widespread standardized support in Ruby, JS, Python
- bcrypt
- scrypt
- Argon2

Steps:

- compute the bcrypt digest from the nickname and master password
- start with the first 12 characters of the bcrypt digest
- replace a random character with a random uppercase
- replace a different random character with a random lowercase
- replace a different random character with a random digit
- replace a different random character with a random symbol

# Note on choosing random values

A simple pseudo-random number generator (PRNG) seeded with bytes from the bcrypt output is used to enforce the password policy constraints.
The implementation is demonstrated to have a uniform distribution adequate for our purposes.

# Note on password policy constraints

For a simple way to satisfy the password policy constraints without compromising entropy,
the output of bcrypt will have four different random characters replaces with random uppercase, lowercase, digit, and symbol.
The symbol set will avoid characters that might not be widely supported, such as the space.


# Process

let nickname, master = user_provided_inputs()

let b = bcrypt(master, nickname)

// or if it is better to normalize those inputs before bcrypt
let b = bcrypt(hash(master), hash(nickname))

// the 12-character length value that will be mutated to ensure the requirement for upper, lower, digit, symbol
let base = b[0..11]

select four random distinct indices into base, e.g. [2, 5, 0, 6]
replace the value of base at the first index with a random uppercase
replace the value of base at the second index with a random lowercase
replace the value of base at the third index with a random digit
replace the value of base at the fourth index with a random symbol

# choosing random values
choosing the four random distinct indices into base will require indefinitely many invocations of the hash function


---

random* values are deterministically derived from b[12..15]
for example for random uppercase, convert b[12] to integer and mod 26 to get an index into [ABC...XYZ]
for random digit convert b[14] into integer and mod 10

# Not proposed for Oplop v2

- Anything that requires remembering or recording additional information for each nickname, such as ability to specify a length
- Characters that are not easily input on a keyboard or that might be disallowed by poorly-implemented systems

# References

https://github.com/brettcannon/oplop
https://crypto.stackexchange.com/questions/3489/do-md5s-weaknesses-affect-oplop
http://www.cs.utexas.edu/~bwaters/publications/papers/www2005.pdf
https://people.csail.mit.edu/rivest/sampler.py
https://gist.github.com/blixt/f17b47c62508be59987b

