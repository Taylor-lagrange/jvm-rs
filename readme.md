# jvm-rs

## boot cofig

```json
"args": [
  "-x",
  "./resources",//stimulate class path(too much jar in real classpath, but most time we only need rt.jar)
  "--class",
  "PrintArgs",//the class file you want run
  "--args", // the args be pass to program
  "hello",
  "world"
],
```