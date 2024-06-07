const fs = require('fs');
const path = require('path');
const os = require('os');

const platform = os.platform();

const binMap = {
    win32: 'react_svg_modifier-windows.exe',
    darwin: 'react_svg_modifier-macos',
    linux: 'react_svg_modifier-linux'
};

const src = path.join(__dirname, 'bin', binMap[platform]);
const dest = path.join(__dirname, 'bin', 'react_svg_modifier');

fs.copyFileSync(src, dest);
fs.chmodSync(dest, 0o755);
