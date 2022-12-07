const fs = require('fs');

const input = fs.readFileSync('./input', 'utf-8');

const lines = input.split('\n');

const root = {
    type: 'folder',
    name: "/",
    files: [],
    parent: null,
};


let current_node = root;
let path = [root];
let count = lines.length;
let i = 0;
let entries = [root];

while (i < count) {
    let l = lines[i];
    if (l.trim().length === 0) {
        continue;
    }

    ++i;

    if (l === '$ cd /') {
        current_node = root;
        continue;
    }

    if (l === "$ cd ..") {
        path.pop();
        current_node = path[path.length - 1];
        continue;
    }

    if (l.startsWith("$ cd")) {
        let name = l.slice(5);
        current_node = current_node.files.find((f) => f.name === name);
        path.push(current_node);
        continue;
    }

    if (l === "$ ls") {
        if (current_node.files.length > 0) {
            continue;
        }

        while (i < count && !lines[i].startsWith("$")) {
            let ll = lines[i];
            ++i;

            if (ll.startsWith("dir")) {
                let v = {
                    type: 'folder',
                    name: ll.slice(4),
                    files: [],
                    // parent: current_node,
                };
                entries.push(v);
                current_node.files.push(v);
            } else {
                let file = ll.split(' ');

                let v = {
                    type: 'file',
                    name: file[1],
                    size: parseInt(file[0], 10),
                    // parent: current_node,
                };
                entries.push(v);
                current_node.files.push(v);
            }
        }
    }
}

const collect = [];
function descent(node) {
    let sum = 0;
    for (let file of node.files) {
        if (file.type === 'folder') {
            sum += descent(file);
        } else {
            sum += file.size;
        }
    }

    collect.push([node, sum]);
    return sum;
}

descent(root);
console.log('part 1', collect
    .map(([_, size]) => size)
    .filter(v => v < 100000)
    .reduce((sum, v) => sum + v, 0));

collect.sort((a, b) => a[1] - b[1]);

const total = 70000000;
const needed = 30000000;
const free = total - collect[collect.length - 1][1];

for (let e of collect) {
    if (free + e[1] > needed) {
        console.log('part 2', e[1]);
        break;
    }
}
