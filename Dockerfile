FROM rustembedded/cross:arm-unknown-linux-gnueabihf

RUN apt-get install -y --no-install-recommends curl && \
    cd /usr/arm-linux-gnueabihf && \
    rm -rf * && \
    curl -sL https://github.com/raspberrypi/tools/archive/master.tar.gz \
    | tar xz --strip-components=3 tools-master/arm-bcm2708/arm-rpi-4.9.3-linux-gnueabihf && \
    apt-get purge --auto-remove -y curl
