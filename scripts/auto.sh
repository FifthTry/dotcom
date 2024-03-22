# shellcheck disable=SC2155
export PROJ_ROOT=$(pwd)

export LANG=en_US.UTF-8
export LC_ALL=en_US.UTF-8
export DATABASE_URL=${DATABASE_URL:-postgresql://127.0.0.1/fifthtry}
export FASTN_DB_URL=${FASTN_DB_URL:-postgresql://127.0.0.1/fifthtry}

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
    cp ../target/wasm32-unknown-unknown/release/ft2.wasm ../frontend/ || return 1
    popd2
}

function upload-frontend-debug() {
    pushd2 "${PROJ_ROOT}/frontend" || return 1

    mkdir -p "${PROJ_ROOT}/../ft/tejar-cache"

    rm .gitignore
    DEBUG_USE_TEJAR_FOLDER="${PROJ_ROOT}/../ft/tejar-cache" \
      FIFTHTRY_SITE_WRITE_TOKEN="fifthtry-write-token" \
      DEBUG_API_FIFTHTRY_COM="http://127.0.0.1" \
      clift upload localhost
    echo '*.wasm' > .gitignore
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

    if [ -z "$(git status --porcelain)" ]; then
      echo "working directory clean"
    else
      echo "working directory dirty"
      return 1
    fi

    cd ../../ft || return 1

    if [ -z "$(git status --porcelain)" ]; then
      echo "ft working directory clean"
    else
      echo "ft working directory dirty"
      return 1
    fi

    ft_hash="$(git describe --tags --always)"
    expected_hash="1623c3eb"  # <-- update this line when updating ft

    if [[ "$ft_hash" == "$expected_hash" ]]; then
      echo "ft has correct hash"
    else
      echo "ft commit hash is $ft_hash, expected $expected_hash"
      return 1
    fi

    cd ../dotcom/frontend || return 1

    git describe --tags --always >> dotcom.commit.hash
    rm .gitignore

    echo "building latest wasm"
    build-ft2-wasm || return 1

    FIFTHTRY_SITE_WRITE_TOKEN=$(cat ../token.txt) \
      echo "going to upload"  # clift upload ft

    echo '*.wasm' > .gitignore
    rm dotcom.commit.hash

    popd2
}

function reload-auto() {
    pushd2 "${PROJ_ROOT}" || return 1
    source scripts/auto.sh
    popd2
}

function update-frontend() {
    pushd2 "${PROJ_ROOT}/frontend" || return 1
    rm -rf .packages
    fastn update
    git status
    popd2
}

