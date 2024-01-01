FROM node:20 AS client-builder

WORKDIR /app
COPY . .
RUN npm install && npm run build

FROM rust:1-buster as server-builder

WORKDIR /app
COPY . .
RUN apt -y update && apt install -y musl-tools musl-dev
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --release --target x86_64-unknown-linux-musl


FROM scratch

COPY --from=client-builder /app/dist/client/public /public
COPY --from=server-builder /app/dist/server/x86_64-unknown-linux-musl/release/kamaji /kamaji

EXPOSE 80

ENTRYPOINT ["/kamaji"]
