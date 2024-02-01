FROM rust:alpine as base
WORKDIR /usr/src/app
RUN apk --no-cache add curl musl-dev openssl-dev \
  && cargo install cargo-chef

FROM base as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM base as builder
WORKDIR /usr/src/app
COPY --from=planner /usr/src/app/recipe.json /usr/src/app/recipe.json
RUN cargo chef cook --release --recipe-path /usr/src/app/recipe.json
COPY . .
RUN cargo build --release

FROM alpine
RUN addgroup -S appgroup && adduser -S appuser -G appgroup
COPY --from=builder /usr/src/app/target/release/battlesnake .
COPY --from=builder /usr/src/app/public /public
USER appuser
EXPOSE 8888
CMD ["./battlesnake"]
