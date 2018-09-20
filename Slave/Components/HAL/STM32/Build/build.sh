#!/bin/bash

# arm-none-eabi-as -march=armv7-m Startup/Startup.s

arm-none-eabi-gcc -c -g -march=armv7-m -I Include Source/halPorts.c
arm-none-eabi-gcc -c -g -march=armv7-m -I Include Source/halClocks.c
arm-none-eabi-gcc -c -g -march=armv7-m -I Include Source/halConfigure.c
ar crs hal.a halPorts.o halClocks.o halConfigure.o