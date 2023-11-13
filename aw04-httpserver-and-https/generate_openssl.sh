#!/bin/bash
if [ -f cert.pem ]; then
    rm -rf cert.pem
fi

if [ -f key.pem ]; then
    rm -rf key.pem
fi

if [ ! -f cert.pem ] && [ ! -f key.pem ]; then
    openssl req -x509\
     -newkey rsa:4096\
     -keyout key.pem\
     -out cert.pem\
     -days 365\
     -sha256\
     -subj "/C=ID/ST=DKI/L=Jakarta/O=RustLang/OU=Org/CN=localhost" 
fi