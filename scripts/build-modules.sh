printf "<<< building core >>> \n"
cd modules/core && cargo build
cd ../..

printf "<<< building js-bridge >>> \n"
cd modules/js-bridge && npm run build
cd ../..