const file = await Deno.readFile('input.txt');
const decoder = new TextDecoder('utf-8');
const inp = decoder.decode(file);

const cals = inp.split('\n').reduce((acc, val) => {
	const num = parseInt(val);

	if (isNaN(num)) {
		acc.push(0);
	} else {
		acc[acc.length - 1] += num;
	}

	return acc;
}, [] as number[]);

console.log('Part 1');
console.log(cals.sort((a, b) => b - a)[0]);

console.log('Part 2');
console.log(
	cals
		.sort((a, b) => b - a)
		.slice(0, 3)
		.reduce((acc, val) => acc + val, 0)
);
