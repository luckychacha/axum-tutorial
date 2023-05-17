axum-tutorial

### install

```bash

cargo install cargo-watch

```

### test commands

- server:

```bash

cargo watch -q -c -w src/ -x run

```



-  client

```bash

cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"

```