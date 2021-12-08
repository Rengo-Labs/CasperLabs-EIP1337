#!/bin/bash
(
  echo -n '00' && \
  casper-client account-address --public-key $1 | cut -d'-' -f3
) | xxd -r -p | base64
