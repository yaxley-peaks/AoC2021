const fs = require('fs');


const inp = fs.readFileSync('../input.txt', 'utf8')
    .split('\n')
    .map(Number);


let sum = 0;
for (let i = 1; i < inp.length; i++) {
    if (inp[i] > inp[i - 1]) {
        sum++;
    }
}
console.log(sum);

sum = 0;
for (let i = 3; i < inp.length; i++) {
    if (inp[i] > inp[i - 3]) {
        sum++;
    }
}
console.log(sum);