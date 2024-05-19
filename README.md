# An experimental JS runtime.
Usage: put JS code into `index.js`. Does not have many APIs, just fs is implemented.
Example for fs:
```js
const contents = await rt.fs.readFile("file.txt");
console.log("content:", contents)
```