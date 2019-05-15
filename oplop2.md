I've been working on an update of the Oplop algorithm, tentatively calling it Oplop v2.
The intent is to improve the generated passwords to be adequate for several years, the way Oplop v1 has been.
Implementations can support both versions to facilitate user migration, and new users should start with v2.

I'd be interested in any comments on the plan, and if you'd rather it not be associated with Oplop then let me know.

# Weaknesses of Oplop v1 algorithm

- 8 character length is too short
- does not guarantee upper, lower, symbol
- method of guaranteeing digit is problematic
- use of MD5 is distracting/not best practice/vulnerable to brute force of master password

# Proposed Oplop v2 algorithm

- 12 character length
- guarantee upper, lower, digit and symbol for password policy
- replace md5 with either SHA-256 or PBKDF2
- same user experience as v1, except for a possible delay depending on hashing algorithm

# Not proposed for Oplop v2

- Anything that requires remembering or recording additional information for each nickname, such as ability to specify a length or salt
- Characters that are not easily input on a keyboard or that might be disallowed by poorly-implemented systems

# Note on hashing algorithm

Because of the user experience of the Oplop system, the benefits of using a proper password hashing algorithm are diminished.
A per-user salt cannot be used without breaking one of the main benefits 
PBKDF2, bcrypt, scrypt, and Argon2 all have parameters that allow tuning the difficulty to thwart brute force attacks.
Parameter values that 


The reason not to use a proper password hashing algorithm
I'm undecided about whether to use a prop
Whether to use a proper password hashing algorithm such as PBKDF2 
Should a proper password hashing algorithm be used, or is SHA-256 sufficient?

This is a conservative choice to replace MD5. Alternatives including bcrypt, scrypt and Argon2 are no more widely established and don't offer significant benefits for the threat models that Oplop is mainly concerned with.
What they all have in common is the ability to tune the difficulty, which is intended to be used over time in response to advances in computing.
But this feature isn't helpful for Oplop2, because a change in any parameters will require a new version of Oplop that is not backward compatible (i.e. the same nickname and master password will produce a different result if these parameters are changed).
So Oplop2 should choose difficulty parameters that will give some benefitare reasonable for today.

which seeks to be finalized 
Oplop2 has to choose 
- PBKDF2 with HMAC-SHA-256
 - widespread standardized support in Ruby, JS, Python
- bcrypt
- scrypt
- Argon2

Steps:

- compute the digest from the nickname and master password
- start with the first 12 characters of the digest
- replace a random character with a random uppercase
- replace a different random character with a random lowercase
- replace a different random character with a random digit
- replace a different random character with a random symbol

# Note on choosing random values

A simple pseudo-random number generator (PRNG) seeded with bytes from the digest is used to enforce the password policy constraints.
The implementation is demonstrated to have a uniform distribution adequate for our purposes.

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

