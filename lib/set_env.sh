#!/bin/bash
export LIBRARY_PATH=$(pwd):$LIBRARY_PATH
export DYLD_LIBRARY_PATH=$(pwd):$DYLD_LIBRARY_PATH
echo "Environment variables set!"
