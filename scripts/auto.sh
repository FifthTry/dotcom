# shellcheck disable=SC2155
export PROJ_ROOT=$(pwd)

export LANG=en_US.UTF-8
export LC_ALL=en_US.UTF-8


function pushd2() {
    PUSHED=$(pwd)
    cd "${PROJDIR}""$1" >> /dev/null || return
}

function popd2() {
    cd "${PUSHED:-$PROJDIR}" >> /dev/null || return
    unset PUSHED
}


function build-ft2-wasm() {
    pushd2 "${PROJ_ROOT}/ft2" || return 1
    # cargo clean
    cargo build --target wasm32-unknown-unknown --release || return 1
    cp target/wasm32-unknown-unknown/release/ft2.wasm ../frontend/ || return 1
    popd2
}

function upload-frontend-debug() {
    pushd2 "${PROJ_ROOT}/frontend" || return 1

    mkdir -p "${PROJ_ROOT}/tejar-cache"

    rm frontend/.gitignore
    DEBUG_USE_TEJAR_FOLDER="${PROJ_ROOT}/tejar-cache" \
      FIFTHTRY_SITE_WRITE_TOKEN="fifthtry-write-token" \
      DEBUG_API_FIFTHTRY_COM="http://127.0.0.1" \
      clift upload localhost
    echo '*.wasm' > frontend/.gitignore
    popd2
}

# For use only inside Github Actions
#
# Github restricts forwarding from 127.0.0.1
# since it's a privileged port
function upload-frontend-github() {
    pushd2 "${PROJ_ROOT}/frontend" || return 1

    mkdir -p "${PROJ_ROOT}/tejar-cache"
    rm frontend/.gitignore

    DEBUG_USE_TEJAR_FOLDER="${PROJ_ROOT}/tejar-cache" \
    FIFTHTRY_SITE_WRITE_TOKEN="fifthtry-write-token" \
    DEBUG_API_FIFTHTRY_COM="http://127.0.0.1:8001" \
    clift upload localhost
    echo '*.wasm' > frontend/.gitignore

    popd2
}

function upload-frontend-prod() {
    pushd2 "${PROJ_ROOT}/frontend" || return 1

    rm frontend/.gitignore
    FIFTHTRY_SITE_WRITE_TOKEN=$(cat ../token.txt) \
      clift upload ft
    echo '*.wasm' > frontend/.gitignore

    popd2
}

function update-frontend() {
    pushd2 "${PROJ_ROOT}/frontend" || return 1
    rm -rf .packages
    fastn update
    git status
    popd2
}

