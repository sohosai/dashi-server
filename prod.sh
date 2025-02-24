#!/bin/bash

# pre-ping (check only RDB)
/app/target/release/pre-ping
#migration
/app/target/release/migration
# ping
/app/target/release/ping
# healthcheck and optional initialization
/app/target/release/healthcheck
# start dashi-server
/app/target/release/presentation
