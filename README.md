# What is this

Writing a sample of axum application base on [isucon/isucon11-qualify](https://github.com/isucon/isucon11-qualify) for my study purpose.

# How to run test

```
$ docker-compose up -d
$ MYSQL_DBNAME=isucondition_test ./sql/init.sh
$ cargo test --workspace -- --test-threads=1 --nocapture
```
