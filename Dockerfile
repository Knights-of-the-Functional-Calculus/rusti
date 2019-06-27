FROM rust:latest

WORKDIR /src/rusti
COPY . .

ENV DISCORD_TOKEN ${DISCORD_TOKEN}

RUN cargo install --path .

CMD ["rusti"]