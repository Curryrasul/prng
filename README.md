### Pseudo random numbers distribution ###
***Program gets file with names, number of tickets and seed as arguments and outputs uniform distribution to stdout***

#### Клонирование репозитория и запуск ####
 ```bash
git clone git@github.com:curryrasul/prng.git && cd prng/
```

 ```bash
cargo run --release
```

**To build:**
```bash
cargo build --release
```

**Run after build:**
```bash
./target/release/prng --file students.txt --numbilets 30 --parameter 10
```
