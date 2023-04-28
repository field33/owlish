const fs = require('fs') // replace with import fs from 'fs'; if you need
const packageFileContent = fs.readFileSync('./pkg/package.json', 'utf-8')
const packageJSON = JSON.parse(packageFileContent)
packageJSON.type = 'module'
packageJSON.main = packageJSON.module
packageJSON.exports = { 
    '.': './' + packageJSON.module,
    [packageJSON.files[0]]: './' + packageJSON.files[0]
}
packageJSON.dependencies = { typescript: '^4.8.3' }
packageJSON.scripts = { tsc: 'tsc owlish.d.ts' }
fs.writeFileSync(
    './pkg/package.json',
    JSON.stringify(packageJSON, null, 2),
    'utf-8'
)
