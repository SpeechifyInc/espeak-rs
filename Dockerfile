FROM ubuntu:20.04

# Set the DEBIAN_FRONTEND environment variable to 'noninteractive' to avoid prompts
# In particular, this prevents tzdata from asking for the timezone
ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get -y install gcc libclang-dev npm autotools-dev automake curl fd-find build-essential libc6-dev libstdc++-10-dev wget git libfftw3-dev autoconf libtool pkgconf && apt-get clean

WORKDIR /app

# Install Rust. Use nightly (faster build times)
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain nightly

# install sonic
RUN git clone https://github.com/waywardgeek/sonic && cd sonic && make && make install && cd .. && rm -rf sonic

# Install pcaudiolib
RUN git clone https://github.com/espeak-ng/pcaudiolib && cd pcaudiolib && ./autogen.sh && ./configure --enable-static --disable-shared && make && make install && cd .. && rm -rf pcaudiolib

# Install alsa
RUN git clone https://github.com/alsa-project/alsa-lib && cd alsa-lib && libtoolize --force --copy --automake && aclocal && autoheader && automake --force-missing --add-missing && autoconf && ./configure --enable-shared=no --enable-static=yes && make && make install && cd .. && rm -rf alsa-lib

# Install espeak-ng
RUN git clone https://github.com/espeak-ng/espeak-ng && cd espeak-ng && ./autogen.sh && ./configure --enable-static --disable-shared && make && make install && cd .. && rm -rf espeak-ng

# Add Rust to the PATH
ENV PATH="/root/.cargo/bin:${PATH}"

# Install testing framework
RUN cargo install cargo-nextest
