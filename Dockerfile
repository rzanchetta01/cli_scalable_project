FROM rust
WORKDIR /src/bin

COPY ./src/. /src/bin/src
COPY Cargo.toml /src/bin
COPY Cargo.lock /src/bin


RUN rustc src/main.rs
CMD ["main.exe"]
run ls -la

