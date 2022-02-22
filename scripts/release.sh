#!/bin/bash

START_DIR="$PWD"
SCRIPT_DIR="$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
TARGET_DIR="$(dirname $SCRIPT_DIR)"
PROJECT="vulner"
REMOTE_URI="git@github.com:mrl5/vulner.git"
MASTER_BRANCH="master"

set -e -u -o pipefail
trap 'cleanup $? $LINENO' EXIT

main() {
    cd "$TARGET_DIR"
    local remote=$(get_remote $REMOTE_URI)
    git checkout $MASTER_BRANCH
    git pull $remote $MASTER_BRANCH --tags
    yarn install --frozen-lockfile
    git checkout -b $(uuidgen --random)
    bump_version
    git_push $remote
}

cleanup() {
    if [ "$1" != "0" ]; then
        echo "Error $1 occurred on $2"
    fi
    cd "$START_DIR"
}

get_remote() {
    local uri="$1"
    (git remote -v | grep -m 1 $uri || \
        (set_remote $uri && git remote -v | grep -m 1 $uri)) |
    cut -f1
}

set_remote() {
    local uri="$1"
    git remote add release $uri
}

get_recommended_bump() {
    yarn run -s recommended-bump
}

bump_version() {
    local recommended_bump=$(get_recommended_bump)
    cargo workspaces version \
        --include-merged-tags \
        --no-git-commit $recommended_bump
    yarn version --$recommended_bump
}

git_push() {
    local remote="$1"
    local tag=v$(get_current_version)
    local branch=release-$tag
    git branch -m $branch
    git push $remote $branch $tag
}

get_current_version() {
    yarn versions | grep $PROJECT | cut -d':' -f2 | cut -d"'" -f2
}

main
