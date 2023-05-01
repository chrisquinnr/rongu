# RONGU
Rust-powered ephemeral key-value storage


## Usage
Store key value pairs using a POST request with a JSON payload.

```
curl -X POST -H "Content-Type: application/json" -d '{"key": "my-key", "value": "my-value"}' http://127.0.0.1:8080/post
```

Retrieve the value using the key
```
http://127.0.0.1:8080/get/my-key
```
