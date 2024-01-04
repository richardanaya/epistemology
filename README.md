# Epistemology

License: MIT

A simple way to run a [llama.cpp](https://github.com/ggerganov/llama.cpp) executable via a local private HTTP API.

Privacy goals:
* server is stateless
* always run on localhost only
* never write logs
* never put prompts in console logs

The goal of this project is to make a completely clear and visible way to run a server locally. The code for how this runs is as minimal as possible so you can understand exactly what you are running.


```
cargo install epistemology
```

example:
```bash
epistemology -p ~/llama/main -m ./magic.gguf

Listening with GET and POST on http://localhost:8080/api/text-completion
Examples:
    * http://localhost:8080/api/text-completion?prompt=famous%20qoute:
    * curl -X POST -d "famous qoute:" http://localhost:8080/api/text-completion
```

You can also run your own web interface from a static path

```bash
epistemology -p ~/llama/main -m ./magic.gguf -u ./my-web-interface

Serving UI on http://localhost:8080/ from ./my-web-interface
Listening with GET and POST on http://localhost:8080/api/text-completion
Examples:
    * http://localhost:8080/api/text-completion?prompt=famous%20qoute:
    * curl -X POST -d "famous qoute:" http://localhost:8080/api/text-completion
```

You can also constrain the output grammar with *.gbnf files for things like JSON output

```bash
epistemology -p ~/llama/main -m ./magic.gguf -g ./json.gbnf

Listening with GET and POST on http://localhost:8080/text-completion
Examples:
    * http://localhost:8080/api/text-completion?prompt=famous%20qoute:
    * curl -X POST -d "famous qoute:" http://localhost:8080/api/text-completion
```
