#!/usr/bin/env bash

set -euo pipefail

rm -rf keys
mkdir keys
ssh-keygen -t rsa -b 4096 -f keys/id_rsa -q -N ""
openssl ecparam -name prime256v1 -genkey -noout -out keys/other-root.key
openssl req -x509 -new -key keys/other-root.key -out keys/other-root.crt -days 3650 -subj "/CN=Update CA"
openssl ecparam -name prime256v1 -genkey -noout -out keys/root.key
openssl req -x509 -new -key keys/root.key -out keys/root.crt -days 3650 -subj "/CN=Update CA"
openssl ecparam -name prime256v1 -genkey -noout -out keys/signer.key
openssl req -new -key keys/signer.key -out keys/signer.csr -subj "/CN=Update Signer"
openssl x509 -req -in keys/signer.csr \
    -CA keys/root.crt -CAkey keys/root.key -CAcreateserial \
    -out keys/signer.crt -days 365