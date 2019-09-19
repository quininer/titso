// gimli based

mpw = pwhash(salt, pw)

mkey = dec(mpw, store)

tag = hash(mkey, ptag)

key = tag xor tag^n

key -> hash(mkey, key + rule) -> password
	-> enc(mkey, key, "rule" + "data") -> encrypt data

tag -> enc(mkey, tag, "hint") -> encrypt hint

===

distribution

0-9
a-zA-Z
,./;'[]=-\`
~!@#$%^&*()_+{}|:"<>?

===

backup and sync

+ salt, store
+ key -> password and encrypt data
+ tag -> encrypt hint
