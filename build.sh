#!/bin/bash
OPENSSL_INCLUDE_DIR=/usr/local/opt/openssl/include \
  DEP_OPENSSL_INCLUDE=/usr/local/opt/openssl/include \
  OPENSSL_LIB_DIR=/usr/local/opt/openssl/include \
  cargo build
