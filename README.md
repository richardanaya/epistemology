# Epistemology

A simple way to run a llama.cpp executable via a local private API.

The goal of this project is to make a completely clear and visible way to run a server locally. The code for how this runs is as minimal as possible.


```
cargo install epistemology
```

example:
```bash
epistemology -p ~/llama/main -m ./magic.gguf

Listening with GET and POST on http://localhost:8080/text-completion
Examples:
    * http://localhost:8080/text-completion?prompt=hello
    * curl -X POST -d "hello" http://localhost:8080/text-completion
```

You can also run your own web interface from a static path

```bash
epistemology -p ~/llama/main -m ./magic.gguf -u ./my-web-interface

Serving UI on http://localhost:8080/ui/ from ./my-web-interface
Listening with GET and POST on http://localhost:8080/text-completion
Examples:
    * http://localhost:8080/text-completion?prompt=hello
    * curl -X POST -d "hello" http://localhost:8080/text-completion
```

You can also constrain the output grammar with *.gbnf files for things like JSON output

```bash
epistemology -p ~/llama/main -m ./magic.gguf -g ./json.gbnf

Listening with GET and POST on http://localhost:8080/text-completion
Examples:
    * http://localhost:8080/text-completion?prompt=hello
    * curl -X POST -d "hello" http://localhost:8080/text-completion
```