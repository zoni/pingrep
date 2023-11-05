github_repo_owner := "zoni"
github_repo_name := "pingrep"

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
	set -xeuo pipefail

	git push --force origin HEAD:release-new-version
	COMMIT_TITLE=$(git show --no-patch --format=%s HEAD)
	CHANGES=$(cargo release changes 2>&1)
	EXISTING_PR=$(gh api -X GET repos/{{github_repo_owner}}/{{github_repo_name}}/pulls -f base=main -F head={{github_repo_owner}}:release-new-version -q '.[].url')
	if [[ $EXISTING_PR != "" ]]; then
		gh api \
			--method PATCH \
			"${EXISTING_PR}" \
			-f title="${COMMIT_TITLE}" \
			-f body="${CHANGES}"
	else
		gh api \
			--method POST \
			repos/{{github_repo_owner}}/{{github_repo_name}}/pulls \
			-f title="${COMMIT_TITLE}" \
			-f body="${CHANGES}" \
			-f head="release-new-version" \
			-f base="main" \
			-F maintainer_can_modify=true
	fi

create-release-tag:
	#!/usr/bin/env bash

	# Create the release tag. It's only purpose here is to make it easy to then
	# grab the tag value, because it's a pain to get it from cargo metadata.
	cargo release tag --execute --no-confirm
	VERSION=$(git tag --list 'v*' --sort version:refname | tail -1)
	SHA_REF=$(git rev-parse HEAD)

	# Create the tag via the GitHub API. This part is critical. If the tag
	# above is pushed through git, then GitHub ignores the event. Only when
	# it's created via the API like this does the release workflow get
	# triggered.
	gh api \
		--method POST \
		/repos/{{github_repo_owner}}/{{github_repo_name}}/git/refs \
		-f ref="refs/tags/${VERSION}" \
		-f sha="${SHA_REF}"
