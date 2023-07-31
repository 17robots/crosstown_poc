# Build initial binary
from rust:slim-buster as builder
run apt update && apt install pkg-config openssl libssl-dev -y
workdir /

copy . . 

run cargo build --release --bin subscriber

# Run binary
from scratch
WORKDIR project
copy --from=builder /project/subscriber/target/release/publisher .
cmd ./subscriber

