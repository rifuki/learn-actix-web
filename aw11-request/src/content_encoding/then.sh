#!/bin/bash

curl -X POST\
     -H "Content-Encoding: gzip"\
     --data-binary "@hello.txt.gz"\
    http://localhost/content_encoding\
    --output hello.txt

echo "output content-encoding is: " && cat ./hello.txt