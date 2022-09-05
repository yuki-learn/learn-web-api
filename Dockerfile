# build
FROM rust:1.58.1 AS build

WORKDIR /app
COPY ./startup.sh ./startup.sh
COPY ./todo-app/server .
RUN cargo install diesel_cli --no-default-features --features postgres
RUN cargo build --release
RUN chmod 777 startup.sh
CMD bash startup.sh