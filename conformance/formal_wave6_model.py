#!/usr/bin/env python3
import hashlib,json
def d(v): return hashlib.sha256(json.dumps(v,sort_keys=True,separators=(",",":")).encode()).hexdigest()
a={"hour":7,"minute":30,"days":[1,2,3,4,5]}
b={"days":[1,2,3,4,5],"minute":30,"hour":7}
c={"hour":7,"minute":31,"days":[1,2,3,4,5]}
assert d(a)==d(b), "schedule key order changed identity"
assert d(a)!=d(c), "schedule time change preserved identity"
print("alarm schedule semantic digest: ok")
