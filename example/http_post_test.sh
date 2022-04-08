#!/bin/sh

curl --verbose https://api-free.deepl.com/v2/translate \
	-d auth_key=819ee1f7-f31b-1bc8-03a6-5e5a33b7bbd5:fx \
	-d "text=Hello"  \
	-d "target_lang=JA"
