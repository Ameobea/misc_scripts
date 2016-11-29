#!/bin/bash

# Every .05 seconds, outputs the current WiFi signal strengh in dBm to stdout.
# Loops 5000 times before exiting.

for i in {1..5000}
do
    sudo iwconfig wlan0 | grep "Signal level=" | sed 's/Link Quality=.*Signal level=//; s/ dBm//; s/\s*//' && sleep .05
done
