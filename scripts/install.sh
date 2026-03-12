#!/bin/sh

set -eu

REPO_SLUG=${AYORU_REPO_SLUG:-BrandonLee28/ayoru}
DEFAULT_INSTALL_DIR=${AYORU_INSTALL_DIR:-"$HOME/.local/bin"}

normalize_arch() {
    case "$1" in
        arm64|aarch64)
            printf 'aarch64\n'
            ;;
        x86_64|amd64)
            printf 'x86_64\n'
            ;;
        *)
            printf '%s\n' "$1"
            ;;
    esac
}

asset_name() {
    os=$1
    arch=$(normalize_arch "$2")
    printf 'ayoru-%s-%s.tar.gz\n' "$os" "$arch"
}

normalize_os() {
    case "$1" in
        Darwin|darwin)
            printf 'darwin\n'
            ;;
        Linux|linux)
            printf 'linux\n'
            ;;
        *)
            printf '%s\n' "$1"
            ;;
    esac
}

require_supported_target() {
    os=$(normalize_os "$1")
    arch=$(normalize_arch "$2")

    case "$os:$arch" in
        darwin:aarch64|darwin:x86_64|linux:aarch64|linux:x86_64)
            printf '%s:%s\n' "$os" "$arch"
            ;;
        *)
            printf 'unsupported platform: %s %s\n' "$1" "$2" >&2
            return 1
            ;;
    esac
}

release_asset_url() {
    version=$1
    os_arch=$(require_supported_target "$2" "$3")
    os=${os_arch%%:*}
    arch=${os_arch#*:}

    printf 'https://github.com/%s/releases/download/%s/%s\n' \
        "$REPO_SLUG" \
        "$version" \
        "$(asset_name "$os" "$arch")"
}

shell_profile_for() {
    shell_name=$1
    home_dir=$2

    case "$shell_name" in
        zsh)
            printf '%s/.zprofile\n' "$home_dir"
            ;;
        bash)
            printf '%s/.bash_profile\n' "$home_dir"
            ;;
        *)
            printf '%s/.profile\n' "$home_dir"
            ;;
    esac
}

path_export_line() {
    install_dir=$1
    printf 'export PATH="%s:$PATH"\n' "$install_dir"
}

append_path_if_missing() {
    profile=$1
    install_dir=$2
    line=$(path_export_line "$install_dir")

    mkdir -p "$(dirname "$profile")"
    touch "$profile"

    if ! grep -Fqx "$line" "$profile"; then
        printf '%s\n' "$line" >> "$profile"
    fi
}

repo_source_dir() {
    candidate=$1

    if [ -e "$candidate/.git" ] && [ -f "$candidate/Cargo.toml" ]; then
        printf '%s\n' "$candidate"
        return 0
    fi

    return 1
}

repo_clone_url() {
    printf 'https://github.com/%s.git\n' "$REPO_SLUG"
}

script_dir() {
    script_dir_for_path "$0" "$(pwd)"
}

script_dir_for_path() {
    script_path=$1
    current_dir=$2

    case "$script_path" in
        /*)
            dirname "$script_path"
            ;;
        */*)
            path_dir=${script_path%/*}
            case "$path_dir" in
                ./*)
                    path_dir=${path_dir#./}
                    ;;
            esac
            printf '%s/%s\n' "$current_dir" "$path_dir"
            ;;
        *)
            return 1
            ;;
    esac
}

local_repo_root() {
    dir=$(script_dir) || return 1
    repo_source_dir "$(CDPATH= cd -- "$dir/.." && pwd)"
}

need_cmd() {
    if ! command -v "$1" >/dev/null 2>&1; then
        printf 'missing required command: %s\n' "$1" >&2
        return 1
    fi
}

download_to() {
    url=$1
    output=$2

    if command -v curl >/dev/null 2>&1; then
        curl -fsSL "$url" -o "$output"
        return 0
    fi

    if command -v wget >/dev/null 2>&1; then
        wget -qO "$output" "$url"
        return 0
    fi

    printf 'missing required command: curl or wget\n' >&2
    return 1
}

install_binary() {
    source_bin=$1
    install_dir=$2

    mkdir -p "$install_dir"
    cp "$source_bin" "$install_dir/ayoru"
    chmod 755 "$install_dir/ayoru"
}

release_download_url() {
    version=${1:-latest}
    os_arch=$(require_supported_target "$(uname -s)" "$(uname -m)")
    os=${os_arch%%:*}
    arch=${os_arch#*:}
    asset=$(asset_name "$os" "$arch")

    if [ "$version" = "latest" ]; then
        printf 'https://github.com/%s/releases/latest/download/%s\n' "$REPO_SLUG" "$asset"
        return 0
    fi

    release_asset_url "$version" "$os" "$arch"
}

install_from_release() {
    version=${1:-latest}
    install_dir=$2
    tmpdir=$(mktemp -d)
    archive=$tmpdir/ayoru.tar.gz
    bin_dir=$tmpdir/bin
    mkdir -p "$bin_dir"

    trap 'rm -rf "$tmpdir"' EXIT INT TERM

    download_to "$(release_download_url "$version")" "$archive"
    tar -xzf "$archive" -C "$bin_dir"
    install_binary "$bin_dir/ayoru" "$install_dir"
}

build_from_repo() {
    repo_dir=$1
    install_dir=$2

    need_cmd cargo
    cargo build --release --manifest-path "$repo_dir/Cargo.toml"
    install_binary "$repo_dir/target/release/ayoru" "$install_dir"
}

install_from_source() {
    install_dir=$1

    if repo_dir=$(repo_source_dir "$(pwd)" 2>/dev/null); then
        build_from_repo "$repo_dir" "$install_dir"
        return 0
    fi

    if repo_dir=$(local_repo_root 2>/dev/null); then
        build_from_repo "$repo_dir" "$install_dir"
        return 0
    fi

    need_cmd git
    need_cmd cargo

    tmpdir=$(mktemp -d)
    trap 'rm -rf "$tmpdir"' EXIT INT TERM
    git clone --depth 1 "$(repo_clone_url)" "$tmpdir/repo" >/dev/null 2>&1
    build_from_repo "$tmpdir/repo" "$install_dir"
}

detect_shell_name() {
    shell_path=${SHELL:-}

    if [ -n "$shell_path" ]; then
        basename "$shell_path"
        return 0
    fi

    printf 'sh\n'
}

ensure_path_entry() {
    install_dir=$1

    case ":$PATH:" in
        *":$install_dir:"*)
            return 0
            ;;
    esac

    profile=$(shell_profile_for "$(detect_shell_name)" "$HOME")
    append_path_if_missing "$profile" "$install_dir"
    printf 'Added %s to PATH in %s\n' "$install_dir" "$profile"
}

print_success() {
    install_dir=$1
    printf 'Installed ayoru to %s/ayoru\n' "$install_dir"
    if command -v "$install_dir/ayoru" >/dev/null 2>&1; then
        "$install_dir/ayoru" --version || true
    fi
}

main() {
    mode=auto
    version=latest
    install_dir=$DEFAULT_INSTALL_DIR

    while [ "$#" -gt 0 ]; do
        case "$1" in
            --from-source)
                mode=source
                ;;
            --release-version)
                shift
                version=$1
                ;;
            --install-dir)
                shift
                install_dir=$1
                ;;
            *)
                printf 'unknown argument: %s\n' "$1" >&2
                exit 1
                ;;
        esac
        shift
    done

    if [ "$mode" = "source" ]; then
        install_from_source "$install_dir"
    else
        if ! install_from_release "$version" "$install_dir"; then
            printf 'Release install unavailable, falling back to source build.\n' >&2
            install_from_source "$install_dir"
        fi
    fi

    ensure_path_entry "$install_dir"
    print_success "$install_dir"
}

if ! (return 0 2>/dev/null); then
    main "$@"
fi
