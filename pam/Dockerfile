FROM rust:slim

RUN DEBIAN_FRONTEND=noninteractive apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get install -yyq libpam0g-dev libpam0g sudo && \
    apt-get purge

ADD . /pam

WORKDIR /pam
RUN cargo build --release

RUN sed -i "2s|^|auth   sufficient    /pam/target/release/libpam.so\n|" /etc/pam.d/sudo
RUN echo "\
Defaults	env_reset,timestamp_timeout=0\n\
Defaults	mail_badpass\n\
Defaults	secure_path=\"/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin\"\n\
root	ALL=(ALL:ALL) ALL\n\
%sudo	ALL=(ALL:ALL) ALL\n\
@includedir /etc/sudoers.d" >> /etc/sudoers

RUN useradd -ms /bin/bash meetup
RUN echo 'meetup  ALL=(ALL) ALL' >>  /etc/sudoers
USER meetup
WORKDIR /home/meetup

ENTRYPOINT ["/bin/bash"]
