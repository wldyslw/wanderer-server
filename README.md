# Wanderer-Server

Blog written in Rust with help of Rocket and Diesel (because I can).

## Deployment

Project requires Docker.

```bash
# create .env file with required variables (replace values below with your own)
# ADMIN_USERNAME is optional, default value is "Admin"

echo "POSTGRES_PASSWORD=12345678 \
ADMIN_USERNAME=\"John Doe\" \
ADMIN_PASSWORD=12345678" > .env

# run the application
docker-compose up
```
