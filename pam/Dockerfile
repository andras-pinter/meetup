FROM rust:slim as builder

RUN DEBIAN_FRONTEND=noninteractive apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get install -yyq libpam0g-dev && \
    apt-get purge

ADD . /pam

WORKDIR /pam
RUN cargo build --release


FROM ubuntu:latest

RUN DEBIAN_FRONTEND=noninteractive apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get install -yyq sudo && \
    apt-get purge

COPY --from=builder /pam/target/release/libpam.so /root

RUN sed -i "2s|^|auth   sufficient    /root/libpam.so\n|" /etc/pam.d/sudo
RUN echo "\
Defaults	env_reset,timestamp_timeout=0\n\
Defaults	mail_badpass\n\
Defaults	secure_path=\"/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin\"\n\
root	ALL=(ALL:ALL) ALL\n\
%sudo	ALL=(ALL:ALL) ALL" >> /etc/sudoers

RUN useradd -ms /bin/bash meetup
RUN echo 'meetup  ALL=(ALL) ALL' >>  /etc/sudoers
USER meetup
WORKDIR /home/meetup

ENTRYPOINT ["/bin/bash"]
