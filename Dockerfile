FROM node:16

RUN apt-get update && apt-get -y install gcc libespeak-ng-dev libclang-dev

USER node

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/home/node/.cargo/bin:$PATH"
