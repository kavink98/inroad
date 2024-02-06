#!/bin/sh
echo ">> Deploying contract"

near deploy --accountId $NEAR_ACCOUNT --wasmFile ./build/project_factory.wasm