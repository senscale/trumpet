#!/bin/bash

# Generates protocol buffers HDFS / YARN client stubs
#
# Requirements:
#  - Install protoc
#  - Install rust-protobuf
#  - Add protoc-gen-rust to PATH
#  - Download Hadoop source code
#
#
# Usage:
#  generate-proto.sh [hadoop-home] dst

base=`dirname $0`
cd $base; base=`pwd`

OUTPUT_BASE=$base/src/proto
PROTO_BASE=$base/proto

COMMON_PROTO_FILES="IpcConnectionContext.proto RpcHeader.proto"
HDFS_PROTO_FILES="ClientNamenodeProtocol.proto"


if [[ $# == 0 ]]; then
    if [[ "x$HADOOP_HOME" == "x" ]]; then
        echo "Pass Hadoop home as argument, or define HADOOP_HOME environment variable"
        exit 1
    fi

    INPUT_BASE=$HADOOP_HOME
else
    INPUT_BASE=$1
    shift
fi

HDFS_PROTO=$HADOOP_HOME/src/hadoop-hdfs-project/hadoop-hdfs/src/main/proto
COMMON_PROTO=$HADOOP_HOME/src/hadoop-common-project/hadoop-common/src/main/proto
YARN_PROTO=$HADOOP_HOME/src/hadoop-yarn-project/hadoop-yarn/src/main/proto

mkdir -p $OUTPUT_BASE/hdfs $OUTPUT_BASE/common
mkdir -p $PROTO_BASE/hdfs $PROTO_BASE/common


# Common
for proto in $COMMON_PROTO_FILES; do
    cp $COMMON_PROTO/$proto $PROTO_BASE/common
    protoc --rust_out $OUTPUT_BASE/common $COMMON_PROTO/$proto --proto_path $COMMON_PROTO
done


# HDFS
for proto in $HDFS_PROTO_FILES; do
    cp $HDFS_PROTO/$proto $PROTO_BASE/hdfs
    protoc --rust_out $OUTPUT_BASE/hdfs $HDFS_PROTO/$proto --proto_path $HDFS_PROTO \
           --proto_path $COMMON_PROTO
done
