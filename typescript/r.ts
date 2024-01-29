import * as fs from 'fs';


function binToJS(fname,bin_path,target_path){
    const _Buffer = fs.readFileSync(bin_path)
    const _BufferObj = Buffer.from(_Buffer);
    const _IntArray = JSON.parse(JSON.stringify(_BufferObj)).data;
    fs.writeFileSync(target_path+"hex.js", `export const WMIC_DATA = "${wmicBufferObj.toString('hex')}`);
    fs.writeFileSync(target_path+"bin.js", `export const WMIC_DATA = [${wmicIntArray};]`);

}
const wmicBuffer = fs.readFileSync(path)
const wmicBufferObj = Buffer.from(wmicBuffer);
const wmicIntArray = JSON.parse(JSON.stringify(wmicBufferObj)).data;
fs.writeFileSync('./src/app/WMIC.HEX.js', `export const WMIC_DATA = "${wmicBufferObj.toString('hex')}`);
fs.writeFileSync('./src/app/WMIC.INT.js', `export const WMIC_DATA = [${wmicIntArray};]`);


