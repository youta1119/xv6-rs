FROM ubuntu
LABEL maintainer="youta1119 <dev.youta1119@gmail.com>"
LABEL version="1.0"
LABEL description="xv6 rs compile"

RUN mkdir /root/work
WORKDIR /root/work
RUN apt-get -y update &&\
    apt-get -y install curl\
                        build-essential\
                        gdb\
                        gcc-multilib\
                        m4\
                        m4-doc\
                        libncurses5-dev\
                        git\
                        python\
                        pkg-config\
                        libpixman-1-dev\
                        zlib1g-dev\
                        libglib2.0-dev\
                        libtool-bin\
                        libsdl1.2-dev\
                        curl\ 
                        gnupg1\
                        gnupg2 &&\
    echo "deb http://apt.llvm.org/xenial/ llvm-toolchain-xenial-6.0 main" >> /etc/apt/sources.list &&\
    echo "deb-src http://apt.llvm.org/xenial/ llvm-toolchain-xenial-6.0 main" >> /etc/apt/sources.list &&\
    curl  https://apt.llvm.org/llvm-snapshot.gpg.key| apt-key add - &&\
    apt-get -y install clang-6.0 lldb-6.0 lld-6.0

#make gmp
ARG GMP_VERSION="5.0.5"
RUN curl -SL ftp://ftp.gmplib.org/pub/gmp-${GMP_VERSION}/gmp-${GMP_VERSION}.tar.bz2 | tar xj -C ./
RUN cd ./gmp-${GMP_VERSION} &&\
    ./configure --prefix=/usr/local &&\
    make -s -j8 &&\
    make check -s -j8 &&\
    make install &&\
    rm -rf ../gmp-${GMP_VERSION} &&\
    ldconfig
    
#make mpfr
ARG MPFR_VIRSION="3.1.3"
RUN curl -SL http://www.mpfr.org/mpfr-${MPFR_VIRSION}/mpfr-${MPFR_VIRSION}.tar.bz2 | tar xj -C ./
RUN cd ./mpfr-${MPFR_VIRSION} &&\
    ./configure --prefix=/usr/local  &&\
    make -s -j8 &&\
    make check -s -j8 &&\
    make install &&\
    rm -rf ../mpfr-${MPFR_VIRSION} &&\
    ldconfig

#make mpc
ARG MPC_VIRSION="0.9"   
RUN curl -SL http://www.multiprecision.org/downloads/mpc-${MPC_VIRSION}.tar.gz | tar zx -C ./
RUN cd ./mpc-${MPC_VIRSION} &&\
    ./configure -target=i386-jos-elf --disable-nls --prefix=/usr/local &&\
    make -s -j8 &&\ 
    make check -s -j8 &&\ 
    make install &&\ 
    rm -rf ../mpc-${MPC_VIRSION} &&\ 
    ldconfig

#make binutils
ARG BINUTILS_VERSION="2.21.1"
RUN curl -SL https://ftp.gnu.org/gnu/binutils/binutils-${BINUTILS_VERSION}.tar.bz2 | tar xj -C ./
RUN cd ./binutils-${BINUTILS_VERSION} &&\
    ./configure --prefix=/usr/local --target=i386-jos-elf --disable-werror  &&\
    make -s -j8 &&\
    make install &&\ 
    rm -rf ../binutils-${BINUTILS_VERSION}

#make gcc
ARG GCC_VERSION="4.6.4"
RUN curl -SL http://ftpmirror.gnu.org/gcc/gcc-${GCC_VERSION}/gcc-core-${GCC_VERSION}.tar.bz2 | tar xj -C ./
RUN cd ./gcc-${GCC_VERSION} &&\
    mkdir gcc-build &&\ 
    cd gcc-build &&\
    ../configure --prefix=/usr/local --target=i386-jos-elf --disable-werror \
            --disable-libssp --disable-libmudflap --with-newlib --without-headers --enable-languages=c  &&\
    make -s -j8 all-gcc  &&\
    make install-gcc  &&\
    make -s -j8 all-target-libgcc  &&\
    make install-target-libgcc &&\
    rm -rf ../../gcc-${GCC_VERSION}

#make gdb
ARG GDB_VERSION="7.3.1"
RUN curl -SL http://ftpmirror.gnu.org/gdb/gdb-${GDB_VERSION}.tar.bz2 | tar xj -C ./  
RUN cd ./gdb-${GDB_VERSION} &&\
    ./configure --prefix=/usr/local --target=i386-jos-elf --program-prefix=i386-jos-elf- --disable-werror  &&\
    make -s -j8 all &&\
    make install &&\
    rm -rf ../gdb-${GDB_VERSION}

#make qemu
RUN git clone http://web.mit.edu/ccutler/www/qemu.git -b 6.828-2.3.0
RUN cd qemu &&\
    ./configure --disable-kvm --disable-werror --target-list="i386-softmmu x86_64-softmmu" &&\
    make &&\
    make install &&\
    rm -rf ../qemu

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain nightly
ENV PATH $PATH:/root/.cargo/bin
RUN cargo install xargo
RUN rustup component add rust-src