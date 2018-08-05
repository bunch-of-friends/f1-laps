printf "<<< removing js-bridge node_modules >>> \n"
cd modules/js-bridge && rm -rf node_modules lib target
cd ../..

printf "<<< removing core target >>> \n"
cd modules/core && rm -rf target
cd ../..

printf "<<< removing desktop node_modules >>> \n"
cd apps/desktop && rm -rf node_modules
cd ../..

printf "<<< removing node-demo node_modules >>> \n"
cd apps/node-demo && rm -rf node_modules
cd ../..

printf "<<< removing rust-demo target >>> \n"
cd apps/rust-demo && rm -rf target
cd ../..