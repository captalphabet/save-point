#!/bin/bash

## This is a test script, replace cargo r with installed binary, will need to source the script not bash it
save-point ## or save-point or <COMPILED BINARY IN PATH REF>

SV_PATH='~/.config/save-point/_path.temp' ## replace the SV_PATH with the full path to the .config dir
## The above needs to be full path no '~' for home dir


if [ -e $SV_PATH ]; then ## check actual path in SV_PATH
  TEMP_PATH="$(cat $SV_PATH)"
  # echo "path to go to: $TEMP_PATH"
  rm $SV_PATH
  cd $TEMP_PATH
    
else
    echo "File does not exist."
fi
