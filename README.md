# Epistemology

A simple way to run a llama.cpp executable via a local private API.

The goal of this project is to make a completely clear and visible way to run a server locally. The code for how this runs is as minimal as possible.


```
cargo install epistemology
```

example:
```bash
cd /directory-where-you-unzipped-llama.cpp/
epistemology ./magic.gguf

Listening with GET and POST on http://localhost:8080/prompt
Examples:
    * http://localhost:8080/prompt?prompt=hello
    * curl -X POST -d "hello" http://localhost:8080/prompt
```
