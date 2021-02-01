# Wanderer-Server

Blog written in Rust with help of Rocket and Diesel (because I can).

## Deployment

Project requires nightly Rust.

```bash
# install rustup and proper Rust version
curl https://sh.rustup.rs -sSf | sh
rustup install nightly

# create .env file with required variables (replace values below with your own)

echo "POSTGRES_URL=postgres://wanderer:YOUR_PASSWORD@localhost/wanderer
     REDIS_URL=redis://wanderer:YOUR_PASSWORD@localhost/wanderer
     SECRET_KEY=b05f1470-bdf8-422a-bd82-45142bb46548
     ADMIN_PASSWORD=12345678" > .env

# install diesel-cli, then init, migrate and seed the database
cargo install diesel_cli --no-default-features --features "postgres"
sudo -u postgres psql -f init.sql -v pass="'YOUR_PASSWORD'" # use password from .env file, notice two quotes
diesel migration run
cargo run --bin seed

# run the project
cargo run
```
