#!/bin/bash

function SubmitFile () {
  curl -X POST \
  -H "Content-Type: multipart/related" \
  --form "data=@example.png;type=image/png" http://localhost:8080
}

SubmitFile & SubmitFile & SubmitFile &
SubmitFile & SubmitFile & SubmitFile &
SubmitFile & SubmitFile & SubmitFile
