if [ "$1" == "-r" ]; then
    printf "<<< running node-demo REFRESHED >>> \n"
    cd apps/node-demo && npm run refresh && npm start
else
    printf "<<< running node-demo >>> \n"
    cd apps/node-demo && npm start
fi
