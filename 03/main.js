import fs from 'fs/promises';

const getPriority = (char) => {
	const start = char.toUpperCase() == char ? 65 : 97;
	const offset = char.toUpperCase() == char ? 26 : 0;
	return char.charCodeAt(0) - start + 1 + offset;
};

const main = async () => {
	const content = await fs.readFile('./input.txt', 'utf-8');
	const lines = content.trim().split('\n');

	const part1 = lines
		.map((line) => {
			const len = line.length / 2;
			const compartment1 = line.slice(0, len);
			const compartment2 = line.slice(len);

			for (const char of compartment1) {
				if (compartment2.includes(char)) return getPriority(char);
			}

			return 0;
		})
		.reduce((acc, val) => acc + val, 0);

	console.log({ part1 });

	const NUM_OF_GROUPS = 3;

	const part2 = Array.from({ length: lines.length / NUM_OF_GROUPS })
		.map((_, i) => {
			const start = NUM_OF_GROUPS * i;
			const f = lines[start + 0];
			const s = lines[start + 1];
			const t = lines[start + 2];

			for (const char of f) {
				if (s.includes(char) && t.includes(char)) return getPriority(char);
			}

			return 0;
		})
		.reduce((acc, val) => acc + val, 0);

	console.log({ part2 });
};

main();
