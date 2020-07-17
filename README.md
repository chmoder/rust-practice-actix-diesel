# rust practice actix diesel
Example actix web server with redis and deadpool


### Setup:
1) Install redis server
2) Install rustup https://rustup.rs/
3) clone this repo
4) * TODO: more steps for diesel *
5) `cargo run --package rust-practice-actix-diesel --bin rust-practice-actix-diesel --release`

### Usage:
```
GET /{username}/{id}/index.html HTTP/1.1
Host: 127.0.0.1:8080
```

```shell script
siege -c1000 -r 50 -H  'http://127.0.0.1:8080/chmoder/1/index.html'
```
