FROM ubuntu:23.04 as builder

# Set the DEBIAN_FRONTEND environment variable to 'noninteractive' to avoid prompts
# In particular, this prevents tzdata from asking for the timezone
ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update &&  \
    apt-get -y install gcc  \
    libclang-dev  \
    npm \
    autotools-dev \
    automake  \
    curl \
    fd-find  \
    build-essential  \
    libc6-dev  \
    libstdc++-10-dev  \
    wget  \
    git  \
    libfftw3-dev  \
    autoconf  \
    libtool  \
    pkgconf &&  \
    apt-get clean

WORKDIR /app

# Install Rust. Use nightly (faster build times)
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain nightly

# install sonic
RUN git clone https://github.com/waywardgeek/sonic && \
    cd sonic &&  \
    make &&  \
    make install &&  \
    cd .. &&  \
    rm -rf sonic

# Install pcaudiolib
RUN git clone https://github.com/espeak-ng/pcaudiolib &&  \
    cd pcaudiolib &&  \
    ./autogen.sh &&  \
    ./configure CFLAGS="-O3 -march=native -flto -fPIC" --enable-static --disable-shared &&  \
    make &&  \
    make install &&  \
    cd .. &&  \
    rm -rf pcaudiolib

# Install alsa
RUN git clone https://github.com/alsa-project/alsa-lib &&  \
    cd alsa-lib &&  \
    libtoolize --force --copy --automake &&  \
    aclocal &&  \
    autoheader &&  \
    automake --force-missing --add-missing &&  \
    autoconf &&  \
    ./configure CFLAGS="-O3 -march=native -flto -fPIC" --enable-shared=no --enable-static=yes &&  \
    make &&  \
    make install &&  \
    cd .. &&  \
    rm -rf alsa-lib

# Install espeak-ng
RUN git clone https://github.com/espeak-ng/espeak-ng &&  \
    cd espeak-ng &&  \
    ./autogen.sh &&  \
    ./configure CFLAGS="-O3 -march=native -flto -fPIC" --enable-static --disable-shared &&  \
    make &&  \
    make install &&  \
    cd .. &&  \
    rm -rf espeak-ng

# Add Rust to the PATH
ENV PATH="/root/.cargo/bin:${PATH}"

RUN npm install -g @napi-rs/cli

COPY . .

RUN cargo test && \
    cargo clean

RUN npm ci && \
    npm install && \
    npm run build && \
    strip *.node && \
    rm -rf node_modules


FROM scratch as binaries

COPY --from=builder /app/*.node ./