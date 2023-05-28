#!/bin/bash

INIT="/var/init"
if [ ! -e $INIT ]; then
        touch $INIT
        echo "-- Verify SSH Server --"
        ssh-keygen -A
        echo "-- Add Docker --"
        chmod 666 /var/run/docker.sock
        chgrp docker /var/run/docker.sock
        echo "-- Start SSH Server --"
        /usr/sbin/sshd -D
        echo "-- Stopped SSH Server --"
else
        echo "-- Add Docker --"
        chmod 666 /var/run/docker.sock
        chgrp docker /var/run/docker.sock
        echo "-- Start SSH Server --"
        /usr/sbin/sshd -D
        echo "-- Stopped SSH Server --"
fi
