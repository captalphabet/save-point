#!/bin/bash

## This is a test script, replace cargo r with installed binary, will need to source the script not bash it
cargo r


if [ -e ./_path.temp ]; then
  TEMP_PATH="$(cat ./_path.temp)"
  echo "path to go to: $TEMP_PATH"
  rm ./_path.temp
  cd $TEMP_PATH
    
else
    echo "File does not exist."
fi
