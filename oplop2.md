I've been working on an update of the Oplop algorithm, tentatively calling it either Oplop v2 or Oplop 2019 (more on that below).
The intent is to improve the generated passwords to be adequate for several years, the way Oplop v1 has been.
Implementations can support both versions to facilitate user migration, and new users should start with the newest version.

I'd be interested in any comments on the plan, and if you'd rather it not be associated with Oplop then let me know.

# Strengths of Oplop v1

- reasonably secure unique password for each account
- mentally scalable and sustainable for user
- no reliance on third party application or storage

# Weaknesses of Oplop v1

- 8 character length is too short
- does not guarantee upper, lower, symbol
- method of guaranteeing digit is problematic
- use of MD5 is distracting/not best practice/potentially vulnerable to brute force

# Proposed Oplop v2

- preserve the user experience of v1
- 12 character length
- guarantee upper, lower, digit and symbol for password policy
- replace md5 with a better key derivation function such as PBKDF2 or bcrypt

# Proposal for future upgrades

As an alternative to being a one-time update of the algorithm, this could be the start of a sequence of updates.
For example, using a hash function such as bcrypt with tunable difficulty parameters will require fixing the values of those parameters,
with all implementations using the same values. The purpose of those parameters is to allow increasing difficulty in response to Moore's law or whatever. A future update of the Oplop algorithm could include adjustments to those parameters.

A standard upgrage workflow that does not excessively burden the user experience could be used repeatedly.
So for example today's proposed update could be called Oplop 2019, and use bcrypt with certain parameter values.
The next update might be Oplop 2024 using different parameters or a different hashing algorithm, and maybe also produce longer passwords with
more permitted characters in them. These can be decided in the future based on what makes sense at that time.
Users can fairly easily recall e.g., that they have migrated everything from Oplop v1 to Oplop 2019, or that are in the process of migrating from Oplop 2019 to Oplop 2024.

# Choosing a new hashing function

PBKDF2, bcrypt, scrypt, and Argon2 all seem to be reasonable candidates from a security perspective. Possibly a more important consideration
is the availability and quality of implementations across many programming languages, including the ease of using them in an Oplop implementation.
The older and more established hash functions such as PBKDF2 seem to score better in those respects.

# Choosing random values

A simple pseudo-random number generator (PRNG) seeded with bytes from the digest is used to enforce the password policy constraints.
The implementation is demonstrated to have a uniform distribution adequate for our purposes.
https://gist.github.com/blixt/f17b47c62508be59987b

# Proposed steps for Oplop 2019

- compute the digest from the nickname and master password
- start with the first 12 characters of the digest
- replace a random character with a random uppercase
- replace a different random character with a random lowercase
- replace a different random character with a random digit
- replace a different random character with a random symbol

# References

https://github.com/brettcannon/oplop
https://crypto.stackexchange.com/questions/3489/do-md5s-weaknesses-affect-oplop
http://www.cs.utexas.edu/~bwaters/publications/papers/www2005.pdf
https://gist.github.com/blixt/f17b47c62508be59987b






# Process

let nickname, master = user_provided_inputs()

let d = digest(master, nickname)

// the 12-character length value that will be mutated to ensure the requirement for upper, lower, digit, symbol
let base = d[0..11]

select four random distinct indices into base, e.g. [2, 5, 0, 6]
replace the value of base at the first index with a random uppercase
replace the value of base at the second index with a random lowercase
replace the value of base at the third index with a random digit
replace the value of base at the fourth index with a random symbol

---

random* values are deterministically derived from b[12..15]
for example for random uppercase, convert b[12] to integer and mod 26 to get an index into [ABC...XYZ]
for random digit convert b[14] into integer and mod 10

# References

https://github.com/brettcannon/oplop
https://crypto.stackexchange.com/questions/3489/do-md5s-weaknesses-affect-oplop
http://www.cs.utexas.edu/~bwaters/publications/papers/www2005.pdf
https://people.csail.mit.edu/rivest/sampler.py
https://gist.github.com/blixt/f17b47c62508be59987b

