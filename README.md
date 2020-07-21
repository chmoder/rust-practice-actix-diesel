# rust practice actix diesel
Example actix web server with postgres and diesel


### Setup:
1) Install redis server
2) Install rustup https://rustup.rs/
3) clone this repo
4) Add a file named `.env` to the project
5) Add the connection url to the file like: (DATABASE_URL=postgres://diesel_demo:password@192.168.1.2/diesel_demo)
6) diesel migration run
5) `cargo run --package rust-practice-actix-diesel --bin rust-practice-actix-diesel --release`

### Usage:
```
GET /med-supp?zip5=68154&plan=F&age=65&gender=Female&tobacco=Non-Tobacco HTTP/1.1
Host: 127.0.0.1:8080
```

```shell script
siege -c1000 -r 50 -H  'http://127.0.0.1:8080/med-supp?zip5=68154&plan=F&age=65&gender=Female&tobacco=Non-Tobacco'
```
