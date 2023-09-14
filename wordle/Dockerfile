FROM docker.io/library/rust:1.72.0 AS build
COPY . /build/
WORKDIR /build/
RUN cargo build

FROM debian:bookworm-slim
COPY --from=build /build/target/debug/wordle /wordle
ENV HOST 0.0.0.0
ENV PORT 7437
EXPOSE 7437
CMD ["/wordle"]
