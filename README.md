# Epistemology

A simple way to run a [llama.cpp](https://github.com/ggerganov/llama.cpp) executable via a local private HTTP API.

Privacy goals:
* server is stateless
* always run on localhost only
* never write logs
* never put prompts in console logs
* **MIT** license so you can modify this to your specific needs at whim

The goal of this project is to make a completely clear and visible way to run a server locally. The code for how this runs is as minimal as possible so you can understand exactly what you are running.

```
cargo install epistemology
```

example:
```bash
epistemology -p ~/llama/main -e ./magic.gguf

Listening with GET and POST on http://localhost:8080/api/text-completion
Examples:
    * http://localhost:8080/api/text-completion?prompt=famous%20qoute:
    * curl -X POST -d "famous quote:" http://localhost:8080/api/text-completion
```

You can also run your own web interface from a static path

```bash
epistemology -p ~/llama/main -e ./magic.gguf -u ./my-web-interface

Serving UI on http://localhost:8080/ from ./my-web-interface
Listening with GET and POST on http://localhost:8080/api/text-completion
Examples:
    * http://localhost:8080/api/text-completion?prompt=famous%20qoute:
    * curl -X POST -d "famous quote:" http://localhost:8080/api/text-completion
```

You can also constrain the output grammar with *.gbnf files for things like JSON output

```bash
epistemology -p ~/llama/main -e ./magic.gguf -g ./json.gbnf

Listening with GET and POST on http://localhost:8080/text-completion
Examples:
    * http://localhost:8080/api/text-completion?prompt=famous%20qoute:
    * curl -X POST -d "famous quote:" http://localhost:8080/api/text-completion
```

# Constraining to JSON Schema

Constraining AI output to structured data can make it much more useful for programatic usage. This project uses a sister project [GBNF-rs](https://github.com/richardanaya/gbnf) for using JSON schema file as grammars.

Let's assume you have a file called "schema.json" that has JSON schema inside it

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "https://example.com/product.schema.json",
  "title": "Product",
  "description": "Famouse quote and person generator",
  "type": "object",
  "properties": {
    "quote": {
      "description": "A famous quote most people would know",
      "type": "string"
    },
    "firstName": {
      "description": "The authors's first name.",
      "type": "string"
    },
    "lastName": {
      "description": "The authors's last name.",
      "type": "string"
    },
    "age": {
      "description": "Age in years which must be equal to or greater than zero.",
      "type": "number"
    }
  }
}
```

```bash
epistemology -p ~/llama/main -e ./magic.gguf -j ./schema.json
```

We can now ask the AI questions and now get answers constrained to our JSON format. Since a lot of metadata is lost during conversion to an AI grammar, we should re-iterate in the system prompt what we want to guide the generation better.

```json
HTTP POST http://localhost:8080/api/text-completion

[system]
I am Argyle, an intellegent assistant, I structure my responses according to JSON schema

{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "https://example.com/product.schema.json",
  "title": "Product",
  "description": "Famouse quote and person generator",
  "type": "object",
  "properties": {
    "quote": {
      "description": "A famous quote most people would know from the author's book",
      "type": "string"
    },
    "firstName": {
      "description": "The authors's first name.",
      "type": "string"
    },
    "lastName": {
      "description": "The authors's last name.",
      "type": "string"
    },
    "age": {
      "description": "Age in years which must be equal to or greater than zero.",
      "type": "number"
    }
  }
}
[me]user
Generate me a famous quote?
[assistant] 
```

Output

```json
{ 
  "quote" : “The sky above the port was the color of television, tuned to a dead channel.”,
  "firstName" : "William",
  "lastNameName" : "William",
  "age": 75.0
}
```
