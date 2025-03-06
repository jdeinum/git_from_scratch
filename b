[package]
name = "git_from_scratch"
version = "0.1.0"
edition = "2024"

[dependencies]
anyhow = "1.0.96"
assert_cmd = "2.0.16"
bytes = "1.10.0"
clap = { version = "4.5.30", features = ["derive"] }
flate2 = "1.0.35"
hex = "0.4.3"
hex-literal = "1.0.0"
itertools = "0.14.0"
memchr = "2.7.4"
nom = "8.0.0"
predicates = "3.1.3"
sha1 = "0.10.6"
sha2 = "0.10.8"
temp_testdir = "0.2.3"
tempfile = "3.17.1"
thiserror = "2.0.11"
tracing = "0.1.41"
tracing-subscriber = "0.3.19"


[[bin]]
name = "git"
path = "src/main.rs"
