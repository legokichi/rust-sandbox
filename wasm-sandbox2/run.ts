import pkg = require("./pkg");

async function main(){
    for(const i of [...Array(3).keys()]){
        const a = await pkg.handler({
            now: i
        });
        console.log(a);
    }
}

main()
    .then(console.info)
    .catch(console.error);

