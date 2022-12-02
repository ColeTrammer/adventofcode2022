#!/bin/bash

day=$1

cp -r template day${day}
sed -i day${day}/Cargo.toml -e 's/name = "template"/name = "day'${day}'"/g'