FROM ubuntu:16.04

EXPOSE 8000

RUN apt-get update && \    
    apt-get -y install wget curl && \
    useradd build && \
    mkdir -p /home/build && \
    chown -R build:build /home/build

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
RUN /root/.cargo/bin/rustup target add x86_64-unknown-linux-musl && \
    /root/.cargo/bin/rustup install nightly && \
    /root/.cargo/bin/rustup default nightly

ENV PATH=$PATH:/home/build/.cargo/bin:/root/.cargo/bin

COPY . /home/build

WORKDIR /home/build
CMD ./build.sh
