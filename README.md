// gimli based

mpw = pwhash(salt, pw)

mkey = dec(mpw, store)

tag = hash(mkey, ptag)

key = tag | tag^n

key -> hash(mkey, key + n) -> password
	-> enc(mkey, key, "data") -> encrypt data

tag -> enc(mkey, tag, "hint") -> encrypt hint

===

distribution

a-z0-9
A-Z
,./;'[]=-\`
~!@#$%^&*()_+{}|:"<>?
中文

===

backup and sync

+ salt, store
+ key -> password and encrypt data
+ tag -> encrypt hint
