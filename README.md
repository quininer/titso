// gimli based

mpw = pwhash(salt, pw)

mkey = dec(mpw, store)

store-tag = hash(mkey, tag, "store")
aead-tag = hash(mkey, tag, "aead")
kdf-tag = hash(mkey, tag, "kdf")

store-tag
	-> hash(mkey, kdf-tag + rule) -> password
	-> enc(mkey, aead-tag, "data") -> encrypt data

store-tag
	-> enc(mkey, aead-tag, "hint") -> encrypt hint

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
