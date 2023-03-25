FROM ubuntu:jammy

# Install SSH Server
RUN apt update -y
RUN apt install sudo -y
RUN apt install openssh-server -y

# Remove temp
RUN apt clean 
RUN rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*


# Load entrypoint
COPY entrypoint.sh /
RUN chmod +x /entrypoint.sh

EXPOSE 22
ENTRYPOINT ["/entrypoint.sh"]