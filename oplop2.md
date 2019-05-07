# Weaknesses of Oplop v1 algorithm

- 8 character length is too short
- does not guarantee upper, lower, symbol
- method of guaranteeing digit is problematic
- use of MD5 is distracting/not best practice/possibly vulnerable to brute force of master password

# Proposed Oplop v2 algorithm

Features:

- 12 character length
- guarantee upper, lower, digit, symbol, with minimal loss of entropy
- replace md5 with bcrypt
- use SHA-256 for a pseudo-random number generator and to normalize inputs

Steps:

- compute the bcrypt digest from the nickname and master password
- start with the first 12 characters of the bcrypt digest
- replace a character with a random uppercase
- replace a different character with a random lowercase
- replace a different character with a random digit
- replace a different character with a random symbol

# Use of SHA-256 for PRNG

At several points in the algorithm it is necessary to choose a random value within certain constraints.
SHA-256 is ideal for this purpose, because it is fast to compute and its output is effectively random.

The first random value that is required will be derived from the hash of the bcrypt output.
Each subsequent random value will be derived from the hash of the previous hash.
That is, for each step where a hash is computed to provide a random value,
the hash value will be used both to derive the random value required for that step, 
and as the hash input for the next step that requires a random value.

# Note on entropy

For a simple way to satisfy the password policy constraints without compromising entropy,
the output of bcrypt will have four different random characters replaces with random uppercase, lowercase, digit, and symbol.
The symbol set will avoid characters that might not be widely supported, such as the space.


# Process

// ULDS - the requirement for upper, lower, digit, symbol characers

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
- Characters that are not easily input on a keyboard or that might be frequently 
# References

https://github.com/brettcannon/oplop
https://crypto.stackexchange.com/questions/3489/do-md5s-weaknesses-affect-oplop
http://www.cs.utexas.edu/~bwaters/publications/papers/www2005.pdf
https://people.csail.mit.edu/rivest/sampler.py
