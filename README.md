# A fast(er) REST API for IMDB data

This is a *fork* of [*A fast REST API for IMDB data*][go], which switches the Go stack by a Rust stack to see if any improvements can be done. This project has the following goals:

- [ ] Test if a solution (for the same problem) in Rust can be faster than a solution in Go.
- [ ] Ensure that it can be ran on Rust stable.
- [ ] Compare if syntax / code maintainability gets any harder than in Go or C++.
- [ ] Check if Rust is a viable toolkit for backend development.

If you want to get more information about the project's context and the database, check out the main [Go project][go].

## Quickstart
To use the API, first ingest the IMDB data into a MongoDB. For example, I've ingested it using PySpark into a local MongoDB instance. Then, clone this repo, install [Rust 2018 stable][install], and run `cargo run`. If you want to run it as a CLI app, then run `cargo install --path .`.

If you are using an external MongoDB database, you can change the host settings at `.cliApp.yaml`.

## MongoDB Collections

| Collection Name | Documents  | Avg. Document Size | Total Document Size | Num. Indexes | Total Index Size |      |
| :-------------: | :--------- | :----------------: | :-----------------: | :----------: | :--------------: | :--- |
|      crew       | 10,244,172 |      139.9 B       |       1.3 GB        |      2       |     274.9 MB     |      |
|      film       | 7,009,774  |      293.0 B       |       1.9 GB        |      2       |     235.2 MB     |      |
|   principals    | 40,314,920 |      121.4 B       |       4.6 GB        |      1       |     389.7 MB     |      |

[go]: https://github.com/espetro/a-fast-imdb-rest-api
[install]: https://www.rust-lang.org/tools/install