#!/bin/bash
set -uo pipefail
trap 's=$?; echo "$0: Error on line "$LINENO": $BASH_COMMAND"; exit $s' ERR
IFS=$'\n\t'

src="${1?"Missing source device"}"
dev_name="${2?"Missing dest device name"}"
dest="$(dirname $(readlink -f $0))/sysfs/${dev_name}"

[[ ! -e "$dest" ]] || { echo "$dest already exists, not overwriting"; exit 1; }
echo "$src -> $dest"

cp_if_exists() {
    for file in "${@}"; do
        [[ -e "${src}/${file}" ]] && cp "${src}/${file}" "${dest}/${file}" || true
    done
}

mkdir "${dest}"
cp_if_exists removable size ro hidden dev

if [[ -d "${src}/device/" ]] ; then
    mkdir "${dest}/device"
    cp_if_exists device/model device/vendor
    if [[ -d "${src}/device/device/" ]] ; then
        mkdir "${dest}/device/device"
        cp_if_exists device/device/model device/device/vendor
    fi
fi
