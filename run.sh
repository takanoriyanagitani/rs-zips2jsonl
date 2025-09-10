#!/bin/sh

geninput(){
	echo creating input zip files...

	mkdir -p ./sample.d

	jq -c -n '{name:"fuji",  height: 3.776}' > ./sample.d/z0j0.json
	jq -c -n '{name:"takao", height: 0.599}' > ./sample.d/z0j1.json

	jq -c -n '{name:"sky",   height: 0.634}' > ./sample.d/z1j0.json
	jq -c -n '{name:"tokyo", height: 0.333}' > ./sample.d/z1j1.json

	ls ./sample.d/z0j?.json | zip -@ -o -v ./sample.d/z0.zip
	ls ./sample.d/z1j?.json | zip -@ -o -v ./sample.d/z1.zip
}

test -f ./sample.d/z0.zip || geninput
test -f ./sample.d/z1.zip || geninput

which wazero | fgrep -q wazero || exec sh -c '
	echo wazero missing.
	exit 1
'

ls ./sample.d/*.zip |
	cut -d/ -f3- |
	sed -e 's,^,/guest.d/,' |
	wazero \
		run \
		-mount "${PWD}/sample.d:/guest.d:ro" \
		./rs-zips2jsonl.wasm |
	jq -c
