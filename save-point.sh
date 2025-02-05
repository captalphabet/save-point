#!/bin/bash

## This is a test script, replace cargo r with installed binary, will need to source the script not bash it
cargo r

SV_PATH='~/.config/save-point/_path.temp'


if [ -e ./_path.temp ]; then
  TEMP_PATH="$(cat $SV_PATH)"
  echo "path to go to: $TEMP_PATH"
  rm $SV_PATH
  cd $TEMP_PATH
    
else
    echo "File does not exist."
fi
