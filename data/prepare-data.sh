#!/bin/sh

OUTPUT_DIR=`dirname "$0"`
mkdir -p "${OUTPUT_DIR}/routes"

curl 'https://rtw-api.oneworld.com/api/MapData' \
    -H 'Accept: application/json, text/plain, */*'\
    -H 'Content-Type: application/json' \
    -H 'Referer: https://rtw.oneworld.com/rtw/' \
    --data-raw '{"GetMapData_Input":{"customerCode":"ONWIBE2","customerSubCode":"OERTW","lang":"EN","mode":"GEN","productCode":"ONWRTWIBE"}}' \
    -o owe-map-data.json

curl https://raw.githubusercontent.com/lukes/ISO-3166-Countries-with-Regional-Codes/master/all/all.json \
     -o country-code.json

download_fr24() {
    for AIRPORT in $*; do
        curl -q https://www.flightradar24.com/data/airports/${AIRPORT}/routes | grep -o 'arrRoutes=\(\[.\+\]\)' | cut -c 11- > "${OUTPUT_DIR}/routes/${AIRPORT}.json"
        sleep 5
    done
}

# download_fr24 hnd nrt hkg kul lhr hel mad doh amm syd mel bne clt ord dfw lax mia jfk lga phl phx dca
download_fr24 cmb
