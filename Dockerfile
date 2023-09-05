ARG NODE_VERSION=18

FROM rust:1-bookworm as builder


WORKDIR /usr/src/espeak-rs

RUN apt-get update && apt-get install -y libespeak-ng-dev libclang-dev nodejs  ca-certificates curl gnupg

ARG NODE_VERSION

# Install Node
RUN mkdir -p /etc/apt/keyrings && \
    curl -fsSL https://deb.nodesource.com/gpgkey/nodesource-repo.gpg.key | gpg --dearmor -o /etc/apt/keyrings/nodesource.gpg && \
    echo "deb [signed-by=/etc/apt/keyrings/nodesource.gpg] https://deb.nodesource.com/node_${NODE_VERSION}.x nodistro main" > /etc/apt/sources.list.d/nodesource.list && \
    apt-get update && \
    apt-get install -y nodejs


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
