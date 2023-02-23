FROM rust:latest as build-rust

WORKDIR /usr/src/lyng2/server
RUN mkdir emptyLogDir
COPY server/Cargo.toml server/Cargo.lock .
COPY server/src src

RUN \
    --mount=type=cache,target=~/.cargo/registry \
    --mount=type=cache,target=server/target \
    cargo run --release --bin export_schema > schema.gql
RUN \
    --mount=type=cache,target=~/.cargo/registry \
    --mount=type=cache,target=server/target \
    cargo build --release --bin lyng2


FROM node:latest as build-react

WORKDIR /usr/src/lyng2/client
RUN mkdir src && mkdir src/chat
COPY client/public public/
COPY client/bin/generate-graphql-client.ts bin/
COPY client/package.json client/package-lock.json client/tsconfig.json client/operations.gql .
COPY --from=build-rust /usr/src/lyng2/server/schema.gql .
RUN \
    --mount=type=cache,target=~/.npm \
    DOCKER=true npm ci

COPY client/src src/
RUN npm run build


FROM gcr.io/distroless/cc-debian11

WORKDIR /usr/bin/lyng2
COPY --from=build-rust /usr/src/lyng2/server/emptyLogDir server/logs
COPY --from=build-rust /usr/src/lyng2/server/target/release/lyng2 server/lyng2
COPY --from=build-react /usr/src/lyng2/client/build client/build

WORKDIR server
EXPOSE 80
CMD ["./lyng2", "0.0.0.0:80"]