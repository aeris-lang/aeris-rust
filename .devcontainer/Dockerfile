FROM rust:1.87-bookworm

RUN apt-get update && apt-get install -y \
  wget \
  lsb-release \
  software-properties-common \
  gnupg2 \
  cmake \
  pkg-config \
  git \
  && apt-get clean

RUN rustup component add rustfmt

RUN wget -qO- https://apt.llvm.org/llvm.sh | bash -s -- 18 all
ENV LLVM_SYS_180_PREFIX=/usr/lib/llvm-18
ENV LLVM_SYS_180_LIB_DIR=/usr/lib/llvm-18/lib
ENV LLVM_HOME=/usr/lib/llvm-18
ENV PATH="$LLVM_HOME/bin:$PATH"
ENV LD_LIBRARY_PATH="$LLVM_HOME/lib:$LD_LIBRARY_PATH"

WORKDIR /aeris
