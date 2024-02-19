# git clone https://github.com/docker/docker-rust-hello

# docker init

# Run

```
docker build --tag docker-rust-image .
docker run --publish 8000:8000 docker-rust-image
```

# Test API

```
GET @ http://0.0.0.0:8000
```
