FROM ubuntu:24.04

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y \
    build-essential \
    cmake \
    git \
    pkg-config \
    libboost-all-dev \
    libboost-filesystem-dev \
    libboost-system-dev \
    libboost-thread-dev \
    python3 \
    python3-pip \
    iproute2 \
    iputils-ping \
    net-tools \
    tcpdump \
    vim \
    nano \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /tmp

RUN git clone --depth 1 https://github.com/COVESA/vsomeip.git && \
    cd vsomeip && \
    cmake -B build -DCMAKE_INSTALL_PREFIX=/usr/local -DENABLE_SIGNAL_HANDLING=1 && \
    cmake --build build -j2 && \
    cmake --install build && \
    ldconfig

WORKDIR /app

COPY . /app

COPY docker/middleware-entrypoint.sh /usr/local/bin/middleware-entrypoint.sh
RUN sed -i 's/\r$//' /usr/local/bin/middleware-entrypoint.sh && \
    chmod +x /usr/local/bin/middleware-entrypoint.sh

ENTRYPOINT ["/usr/local/bin/middleware-entrypoint.sh"]
CMD ["sleep", "infinity"]