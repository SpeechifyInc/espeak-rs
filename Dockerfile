FROM rust:1-bookworm as builder

WORKDIR /usr/src/espeak-rs

RUN apt-get update && apt-get install -y libespeak-ng-dev libclang-dev nodejs
# Install Node
RUN curl -sL https://deb.nodesource.com/setup_18.x | bash && apt-get install -y nodejs

COPY package*.json .
RUN --mount=type=cache,target=/usr/src/espeak-rs/node_modules \
    --mount=type=cache,target=~/.npm \
    npm ci
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/src/espeak-rs/target \
    --mount=type=cache,target=/usr/src/espeak-rs/node_modules \
    --mount=type=cache,target=~/.npm \
    npm run build

FROM scratch as binaries
COPY --from=builder /usr/src/espeak-rs/*.node .
COPY --from=builder /usr/src/espeak-rs/index.js .
COPY --from=builder /usr/src/espeak-rs/index.d.ts .
