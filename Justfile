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

create-release-pr:
	#!/usr/bin/env bash
	set -euo pipefail

	git push --force origin HEAD:release-new-version
	COMMIT_TITLE=$(git show --no-patch --format=%s HEAD)
	CHANGES=$(cargo release changes 2>&1)
	EXISTING_PR=$(gh api -X GET repos/{owner}/{repo}/pulls -f base=main -F head={owner}:release-new-version -q '.[].url')
	if [[ $EXISTING_PR != "" ]]; then
		gh api \
		--method PATCH \
		"${EXISTING_PR}" \
		-f title="${COMMIT_TITLE}" \
		-f body="${CHANGES}"
	else
		gh api \
		--method POST \
		repos/{owner}/{repo}/pulls \
		-f title="${COMMIT_TITLE}" \
		-f body="${CHANGES}"
		-f head="release-new-version" \
		-f base="main" \
		-F maintainer_can_modify=true
	fi

release:
	cargo release tag --execute --no-confirm
	cargo release push --execute --no-confirm
