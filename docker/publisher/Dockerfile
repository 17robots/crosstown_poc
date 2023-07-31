# Build initial binary
from rust:slim-buster as builder
run apt update && apt install pkg-config openssl libssl-dev -y
workdir /

copy . . 

run cargo build --release --bin publisher

# Run binary
from scratch
WORKDIR project
copy --from=builder /project/publisher/target/release/publisher .
cmd ./publisher

