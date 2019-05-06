# Weaknesses of Oplop v1 algorithm

- 8 character length is too short
- does not guarantee upper, lower, symbol
- method of guaranteeing digit is problematic
- use of MD5 is distracting/not best practice/possibly vulnerable to brute force of master password

# Proposed Oplop v2 algorithm

- 12 character length
- guarantee upper, lower, digit, symbol, without loss of entropy
- replace md5 with bcrypt to protect against brute force and other possible attacks
- use SHA-256 for a pseudo-random number generator and to normalize inputs

# Use of SHA-256 for PRNG

At several points in the algorithm it is necessary to choose a random value within certain constraints.
SHA-256 is ideal for this purpose, because it is fast to compute and its output is effectively random.

The first random value that is required will be derived from the hash of the bcrypt output.
Each subsequent random value will be derived from the hash of the previous hash.
That is, for each step where a hash is computed to provide a random value,
the hash value will be used both to derive the random value required for that step, 
and as the hash input for the next step that requires a random value.

# Process

// ULDS - the requirement for upper, lower, digit, symbol characers

let nickname, master = user_provided_inputs()

let b = bcrypt(password, nickname)

// or if it is better to normalize those inputs before bcrypt
let b = bcrypt(hash(password), hash(nickname))

// the value that will be mutated to ensure the requirement for upper, lower, digit, symbol
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

- Anything that requires remembering or recording additional information for each nickname, such as ability to specify a length or salt

# References

https://github.com/brettcannon/oplop
https://crypto.stackexchange.com/questions/3489/do-md5s-weaknesses-affect-oplop

