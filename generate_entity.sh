#!/bin/bash

# SPDX-FileCopyrightText: 2022 perillamint
#
# SPDX-License-Identifier: CC0-1.0

set -e
cd "$(dirname "$0")"

sea-orm-cli generate entity -v -o src/entity --with-serde both

for file in src/entity/*.rs; do
    reuse addheader --copyright "perillamint" --license GPL-3.0-or-later --template rust "$file"
done
