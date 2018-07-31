FROM ubuntu
LABEL maintainer="youta1119 <dev.youta1119@gmail.com>"
LABEL version="1.0"
LABEL description="llvm clang"

RUN mkdir /root/work
WORKDIR /root/work

RUN  apt-get -y update && \
    apt-get -y install make gcc curl gnupg1 gnupg2 && \
    echo "deb http://apt.llvm.org/xenial/ llvm-toolchain-xenial-6.0 main" >> /etc/apt/sources.list && \
    echo "deb-src http://apt.llvm.org/xenial/ llvm-toolchain-xenial-6.0 main" >> /etc/apt/sources.list && \
    curl  https://apt.llvm.org/llvm-snapshot.gpg.key| apt-key add - && \
    apt-get -y install clang-6.0 lldb-6.0 lld-6.0

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain nightly
ENV PATH $PATH:/root/.cargo/bin
RUN cargo install xargo &&\
    rustup component add rust-src