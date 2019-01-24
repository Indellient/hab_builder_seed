#!/bin/bash

set -u

BLDR="${1}"
NAME="${2}"
TOKEN="${3}"
API="/v1/depot/origins"
DEFAULT_PACKAGE_VISIBILITY="public"

JSON=$(mktemp)

cat <<EOF > ${JSON}
{"name":"${NAME}","default_package_visibility":"${DEFAULT_PACKAGE_VISIBILITY}"}
EOF

curl -vvv ${BLDR}${API} \
  -H 'content-type: application/json' \
  -H "Authorization: Bearer ${TOKEN}" \
  -d @${JSON} \
  --compressed

rm ${JSON}
