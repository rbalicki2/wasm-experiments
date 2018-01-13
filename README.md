# wasm experiments

Compile: `./scripts/compile.sh`

Run the server: `python -m SimpleHTTPServer 8000`

Navigate to localhost:8000 and execute in the console:

```
strPtr = ins.instance.exports.get_str()
console.log(convertToString(strPtr, 4))
// should print out ASDF

console.log(ins.instance.exports.add_one(41)); // 42

// alerts some jibberish
ins.instance.exports.alert_me()
```