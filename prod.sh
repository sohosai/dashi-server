#!/bin/bash

# ping
/app/target/release/ping

ping_exit_code=$?
# initialize if not initialized
if [ $ping_exit_code -eq 0 ] && [ ! -f "/app/init/initialized" ]; then
    # initialize
    echo "ENTRYPOINT: run initialization"
    /app/target/release/init
    
    # create initialized marker
    echo "ENTRYPOINT: create initialized marker"
    touch /app/init/initialized
    exit 0
else
    echo "ENTRYPOINT: skip initialization"
fi

# start dashi-server
if [ $ping_exit_code -eq 0 ] && [ -f "/app/init/initialized" ]; then
    # start dashi-server
    echo "ENTRYPOINT: start dashi-server"
    /app/target/release/presentation
else
    # failed to initialize or create initialized marker
    exit 1
fi
