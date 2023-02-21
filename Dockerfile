FROM rust:latest as build-rust

WORKDIR /usr/src/lyng2/server
COPY server .
RUN rm -rf logs && mkdir logs
RUN cargo run --bin export_schema > schema.gql
RUN cargo build --release --bin lyng2

FROM node:latest as build-react

WORKDIR /usr/src/lyng2/client
COPY client .
COPY --from=build-rust /usr/src/lyng2/server/schema.gql .

RUN DOCKER=true npm ci
RUN npm run build


FROM gcr.io/distroless/cc-debian11

WORKDIR /usr/bin/lyng2

COPY --from=build-rust /usr/src/lyng2/server/target/release/lyng2 server/lyng2
COPY --from=build-rust /usr/src/lyng2/server/logs server/logs
COPY --from=build-react /usr/src/lyng2/client/build client/build

WORKDIR server
EXPOSE 80
CMD ["./lyng2", "0.0.0.0:80"]