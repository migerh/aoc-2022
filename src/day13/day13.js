const fs = require('fs');

const input = fs.readFileSync('./example', 'utf-8');

const signalPairs = input.split('\n\n');
const signals = signalPairs.map(p => {
    let signal = p.split('\n').map(s => JSON.parse(s));
    return {
        left: signal[0],
        right: signal[1],
    };
});

function compare(left, right) {
    if (!Array.isArray(left) && !Array.isArray(right)) {
        return Math.sign(right - left);
    } else if (!Array.isArray(left) && Array.isArray(right)) {
        return compare([left], right);
    } else if (Array.isArray(left) && !Array.isArray(right)) {
        return compare(left, [right]);
    } else if (Array.isArray(left) && Array.isArray(right)) {
        const min = Math.min(left.length, right.length);
        for (let i = 0; i < min; i++) {
            const c = compare(left[i], right[i]);
            if (c !== 0) {
                return c;
            }
        }

        if (left.length == right.length) {
            return 0;
        } else if (left.length <= min) {
            return 1;
        } else {
            return -1;
        }
    }

    return -1;
}

let valid = [];
let index = 1;
for (let signal of signals) {
    if (compare(signal.left, signal.right) === 1) {
        valid.push(index);
    }
    index += 1;
}

console.log('Part 1:', valid.reduce((sum, i) => sum + i, 0));

let sortedSignals = [];
for (let signal of signals) {
    sortedSignals.push({
        isDivider: false,
        signal: signal.left,
    });
    sortedSignals.push({
        isDivider: false,
        signal: signal.right,
    });
}

sortedSignals.push({
    isDivider: true,
    signal: [[2]],
});

sortedSignals.push({
    isDivider: true,
    signal: [[6]],
})

sortedSignals.sort((a, b) => compare(a.signal, b.signal));
sortedSignals.reverse();

let i = 1;
let sum = 1;
for (let signal of sortedSignals) {
    console.log(JSON.stringify(signal.signal));
    if (signal.isDivider) {
        sum *= i;
    }
    i += 1;
}

console.log('Part 2:', sum);

