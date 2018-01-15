#!/bin/sh -ex
protoc --rust_out=src src/protos/matcha_pb.proto
