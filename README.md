# Rust secret_messages

follow the steps below to run the application.

Clone the repository:
```bash
git clone https://github.com/jabg97/secret_messages_rust.git
```

Build Docker's image:
```bash
docker-compose build
docker-compose up -d
```

When you want to have access to the source code, just run:
```bash
docker-compose exec rust bash
```

Then you must compilate and run the application:
```bash
rustc main.rs
./main
```

Finally you can run some unit tests:
```bash
cargo test
```