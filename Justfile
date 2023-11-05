bump-version:
	#!/usr/bin/env bash
	set -euo pipefail

	get_next_version_number() {
		DATEPART=$(date +%y.%-m)
		ITERATION=0

		while true; do
			VERSION_STRING="${DATEPART}.${ITERATION}"
			if git rev-list "v$VERSION_STRING" > /dev/null 2>&1; then
				((ITERATION++))
			else
				echo "$VERSION_STRING"
				return
			fi
		done
	}

	VERSION=$(get_next_version_number)
	cargo release version ${VERSION} --execute --no-confirm
	cargo release commit --execute --no-confirm

release:
	cargo release tag --execute --no-confirm
	cargo release push --execute --no-confirm
