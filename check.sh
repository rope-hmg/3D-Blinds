pushd ./app; cargo check; popd
pushd ./web; svelte-check --tsconfig ./tsconfig.json; popd
