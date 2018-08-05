if [ "$1" == "-r" ]; then
    printf "<<< running desktop REFRESHED >>> \n"
    cd apps/desktop && npm run refresh && npm start
else
    printf "<<< running desktop >>> \n"
    cd apps/desktop && npm start
fi
