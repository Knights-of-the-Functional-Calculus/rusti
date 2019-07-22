FROM rust:latest

WORKDIR /src/rusti
COPY . .

ENV DISCORD_TOKEN ${DISCORD_TOKEN}
ENV TRAVIS_TOKEN ${TRAVIS_TOKEN}

RUN cargo install --path .

CMD ["rusti"]