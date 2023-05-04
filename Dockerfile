FROM ubuntu:latest

RUN apt-get update && apt-get -y install gcc libclang-dev npm autotools-dev automake curl fd-find build-essential libc6-dev libstdc++-11-dev wget

WORKDIR /app

# Install Rust. Use nightly (faster build times)
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain nightly

# install fftw
RUN apt-get -y install libfftw3-dev

# install sonic
RUN git clone https://github.com/waywardgeek/sonic
RUN cd sonic && make && make install

RUN apt-get update && apt-get -y install autoconf libtool pkgconf

# Install pcaudiolib
RUN git clone https://github.com/espeak-ng/pcaudiolib
RUN cd pcaudiolib && ./autogen.sh && ./configure --enable-static --disable-shared && make && make install

# Install alsa
RUN git clone https://github.com/alsa-project/alsa-lib
RUN cd alsa-lib && libtoolize --force --copy --automake && aclocal && autoheader && automake --force-missing --add-missing && autoconf && ./configure --enable-shared=no --enable-static=yes && make && make install

# Install espeak-ng
RUN git clone https://github.com/espeak-ng/espeak-ng
RUN cd espeak-ng && ./autogen.sh && ./configure --enable-static --disable-shared && make && make install

# Add Rust to the PATH
ENV PATH="/root/.cargo/bin:${PATH}"


