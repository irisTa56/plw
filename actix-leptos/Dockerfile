FROM rust:bullseye as builder

RUN cargo install cargo-binstall
RUN cargo binstall cargo-leptos -y
RUN rustup target add wasm32-unknown-unknown

RUN mkdir -p /app
WORKDIR /app

COPY . .
RUN cargo leptos build --release -vv

FROM rust:bullseye as runner
# Copy the server binary to the /app directory
COPY --from=builder /app/target/server/release/example /app/
# /target/site contains our JS/WASM/CSS, etc.
COPY --from=builder /app/target/site /app/site

WORKDIR /app

ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT="site"
EXPOSE 8080

CMD ["/app/example"]
