printf "<<< installing js-bridge dependencies >>> \n"
cd modules/js-bridge && npm install
cd ../..

printf "<<< installing desktop dependencies >>> \n"
cd apps/desktop && npm install
cd ../..

printf "<<< installing node-demo dependencies >>> \n"
cd apps/node-demo && npm install
cd ../..