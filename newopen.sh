#!/bin/sh

# flags
pre_flags() {
    ## verbose: set shell trace
    ## see: https://phoenixnap.com/kb/linux-set#:~:text=The%20set%20command%20is%20a,and%20Korn%20shell%20(%20ksh%20).
    test "$verbose" && set -x
}

# environment variables
handle_envs() {
    : ${CODE_PATH:=$(which code-insiders)} # replace "code-insiders" with "code" for stable version of VS Code
    : ${CODE_CMD:=${CODE_PATH} --no-sandbox --unity-launch %F} # use KDE Plasma grammar, where "%F" represents a list of files
}

# useful paths
handle_paths() {
    olddir=$(pwd)
    rootdir=$(dirname "$0")
    pkgpath="$1"
    pkgdir=$(dirname "$pkgpath")
    pkgbase=$(basename "$pkgpath")
    codeworkspacefile="${pkgbase}.code-workspace"
}

# prepare for initialize
pre_stuff() {
    ## change directory to root directory
    cd "$rootdir"

    ## try to create the parent directory
    mkdir -p "$pkgdir"
    cd "$pkgdir"
}

# initialize with cargo
init_cargo() {
    cargo new "$pkgbase"
    _ret=$?
    if [ "$_ret" -ne 0 ]; then
        echo "Failed to initialize a Cargo project with exit code ${_ret}. Exiting..."
        exit 1
    fi
    cd "$pkgbase"
}

# do some post-initialization stuff
post_init_cargo() {
    ## remove `.git` if necessary
    rm -rf .git

    ## create "VS Code-related" configurations
    ### `*.code-workspace`
    echo -e "{\n\t\"folders\": [\n\t\t{\n\t\t\t\"path\": \".\"\n\t\t}\n\t],\n\t\"settings\": {\n\t}\n}" > "$codeworkspacefile"

    ### `.vscode/extensions.json`
    mkdir -p .vscode
    echo -e "{\n    \"recommendations\": [\n        \"rust-lang.rust-analyzer\",\n        \"serayuzgur.crates\",\n        \"dustypomerleau.rust-syntax\",\n        \"VisualStudioExptTeam.vscodeintellicode\"\n    ]\n}" > .vscode/extensions.json
}

# open VS Code
open_code() {
    eval "${CODE_CMD/\%F/${codeworkspacefile}}" # trailing "&" is not required
}

post_stuff() {
    ## change directory to old directory
    cd "${olddir}"
}

# reset flag effects
post_flags() {
    test "$verbose" && set +x
}

# main function
# see: https://stackoverflow.com/a/13350100/12002560
main() {
    pre_flags       "$@"
    handle_envs     "$@"
    handle_paths    "$@"
    pre_stuff       "$@"
    init_cargo      "$@"
    post_init_cargo "$@"
    open_code       "$@"
    post_stuff      "$@"
    post_flags      "$@"
}

if [ "$1" != "--source-only" ]; then
    main "$@"
fi
