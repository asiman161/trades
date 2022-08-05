FROM rust:1.61.0 as build

RUN USER=root cargo new --bin robot
WORKDIR /robot

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# 3. Build only the dependencies to cache them
RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src
RUN rm ./target/release/deps/robot*
RUN cargo build --release

COPY ./src ./src

# 5. Build for release.
RUN rm ./target/release/deps/robot*
#RUN cargo install --path .
RUN cargo build --release

CMD ["./target/release/robot"]
