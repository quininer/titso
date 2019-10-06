// gimli based

mpw = pwhash(salt, pw)

mkey = dec(mpw, store)

tag = hash(mkey, ptag)

itag = tag xor tag^n

itag -> hash(mkey, itag + rule) -> password
	-> enc(mkey, itag, "data") -> encrypt data

tag -> enc(mkey, tag, "hint") -> encrypt hint

===

distribution

0-9
a-zA-Z
,./;'[]=-\`
~!@#$%^&*()_+{}|:"<>?
custom ?

===

backup and sync

+ salt, secret
	* sss
+ key -> password and encrypt data
+ tag -> encrypt hint

===

roadmap

+ cli
+ wasm
+ simple gui
+ hardware
