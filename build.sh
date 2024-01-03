#!/bin/bash

WORKING_DIR='./bt'
EXTRAS_DIR='./extras/bash_utils'

echo 'Building...'
cd $WORKING_DIR
VERSION=`cat Cargo.toml | grep -m 1 "version" | cut -d'"' -f 2`
cargo build --release
cd ..

echo 'Packaging...'
TEMP_DIR=bipw_ubuntu_amd64_v$VERSION
mkdir -p $TEMP_DIR
cp $WORKING_DIR/target/release/bt $TEMP_DIR/
cp $WORKING_DIR/.env $TEMP_DIR/
cp $EXTRAS_DIR/bt.service $TEMP_DIR/
cp $EXTRAS_DIR/install.sh $TEMP_DIR/
tar -cJf $TEMP_DIR.tar.xz  $TEMP_DIR/

echo 'Cleaning...'
rm -r $TEMP_DIR
