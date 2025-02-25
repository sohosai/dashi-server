#!/bin/bash

# ping
/app/target/release/ping
# healthcheck and optional initialization
/app/target/release/init
# start dashi-server
/app/target/release/presentation
