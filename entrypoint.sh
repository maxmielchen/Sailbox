#!/bin/bash

INIT="/var/init"
SSHD_PID=""

handle_stop() {
    echo "[INFO] Container stop requested. Stopping SSH server..."
    if [ -n "$SSHD_PID" ] && kill -0 "$SSHD_PID" 2>/dev/null; then
        kill "$SSHD_PID"
        wait "$SSHD_PID"
        echo "[INFO] SSH server stopped. Container will now exit."
    fi
    exit 0
}

trap handle_stop SIGTERM SIGINT

if [ ! -e $INIT ]; then
    touch $INIT
    echo "-- Verify SSH Server --"
    ssh-keygen -A
    echo "-- Add Docker --"
    chmod 666 /var/run/docker.sock
    chgrp docker /var/run/docker.sock
    echo "-- Start SSH Server --"
    /usr/sbin/sshd -D &
    SSHD_PID=$!
    wait $SSHD_PID
    echo "-- Stopped SSH Server --"
else
    echo "-- Add Docker --"
    chmod 666 /var/run/docker.sock
    chgrp docker /var/run/docker.sock
    echo "-- Start SSH Server --"
    /usr/sbin/sshd -D &
    SSHD_PID=$!
    wait $SSHD_PID
    echo "-- Stopped SSH Server --"
fi
