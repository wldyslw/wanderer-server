# Wanderer-Server

Blog written in Rust with help of Rocket and Diesel (because I can).

## Deployment

Project requires nightly Rust.

```bash
# install rustup and proper Rust version
curl https://sh.rustup.rs -sSf | sh
rustup install nightly

# install diesel-cli, then init, migrate and seed the database
cargo install diesel_cli --no-default-features --features "postgres"
sudo -u postgres psql -f init.sql -v pass="'YOUR_PASSWORD'" # notice two quotes
diesel migration run
cargo run --bin seed

# run the project
cargo run
```
