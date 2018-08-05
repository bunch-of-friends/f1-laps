printf "<<< removing js-bridge artifacts >>> \n"
cd modules/js-bridge && rm -rf node_modules lib native/target native/index.node
cd ../..

printf "<<< removing core artifacts >>> \n"
cd modules/core && rm -rf target
cd ../..

printf "<<< removing desktop artifacts >>> \n"
cd apps/desktop && rm -rf node_modules package-lock.json #file dependency on on js-bridge causing troubles, therefore lock file removed
cd ../..

printf "<<< removing node-demo artifacts >>> \n"
cd apps/node-demo && rm -rf node_modules package-lock.json #file dependency on on js-bridge causing troubles, therefore lock file removed
cd ../..

printf "<<< removing rust-demo artifacts >>> \n"
cd apps/rust-demo && rm -rf target
cd ../..