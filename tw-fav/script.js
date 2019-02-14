const fs = require("fs");
const path = require("path");
const {promisify} = require("util");


async function main (){
    const pathname = "/home/legokichi/Dropbox/tw";
    const reg = /(\d+-\d+-\d+_\d+-\d+-\d+)_(\S+)-(\d+)-(\d+)\.\S+$/;
    const files = await promisify(fs.readdir)(pathname);
    const len = files.length;
    for(let i = 0; i < len; i++){
        const filename = files[i];
        console.log(i, "/", len, filename);
        const stat = await promisify(fs.lstat)(path.join(pathname, filename));
        const filetype = stat.isFile() ? "file"
                        : stat.isDirectory() ? "dir"
                        : stat.isBlockDevice() ? "blcdev"
                        : stat.isCharacterDevice() ? "chardev"
                        : stat.isSymbolicLink() ? "symlink"
                        : stat.isFIFO() ? "fifo"
                        : stat.isSocket() ? "socket"
                        : "unkown";
        if(filetype !== "file"){ continue; }
        if(!reg.test(filename)){ continue; }
        const [_, date, screen_name, twid, index] = reg.exec(filename);
        if(!await promisify(fs.exists)(path.join(pathname, screen_name))){
            await promisify(fs.mkdir)(path.join(pathname, screen_name));
        }
        await promisify(fs.rename)(path.join(pathname, filename), path.join(pathname, screen_name, filename));
    }
}
main().catch(console.error)
