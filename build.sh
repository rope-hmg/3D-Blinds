wasm-pack build ./app --target web --release

pushd ./web
npm install ../app/pkg --save
npx vite build
popd
