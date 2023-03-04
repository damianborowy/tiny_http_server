#!/bin/bash

run() {
  chmod +x ./build/server_manager
  ./build/server_manager
}

run

cd frontend
yarn build

cd ..
cd backend
cargo build --release

cd ..
mkdir -p build
mv backend/target/release/* build
mv frontend/dist/index.html build/index.html
rm -R backend/target
rm -R frontend/dist

echo ""

run