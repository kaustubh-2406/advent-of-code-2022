const td = new TextDecoder('utf-8');

const sampleT = Deno.readFileSync('./sample.txt');
const sample = td.decode(sampleT);

const inputT = Deno.readFileSync('./input.txt');
const input = td.decode(inputT);

type Stack = Record<number, string[]>;
type State = {
	stackState: Stack;
	procState: number[][];
};

function parseInput(str: string): State {
	const lines = str.trimEnd().split('\n');
	const indx = lines.findIndex((l) => l == '');
	const temp = lines.slice(0, indx).reduce((acc, v) => {
		const splitOnThreeChars = (str: string): string[] => {
			const state = [];
			for (let i = 0; i < Math.ceil(str.length / 4); i += 1) {
				state.push(str.slice(4 * i, 4 * i + 3)[1]);
			}
			return state;
		};

		return [...acc, splitOnThreeChars(v)];
	}, [] as string[][]);

	const stackState: Record<number, string[]> = {};
	for (const i of temp.at(-1)!) {
		const num = +i!;
		stackState[num] = temp
			.slice(0, -1)
			.map((arr) => arr.at(num - 1)!)
			.reverse()
			.filter((x) => x && x != ' ');
	}

	// move 1 from 2 to 1
	const procState = lines.slice(indx + 1).map((line) =>
		line
			.match(/move (\d+) from (\d+) to (\d+)/i)!
			.slice(1)
			.map((val) => +val)
	);

	return { stackState, procState };
}

function solve({ stackState, procState }: State) {
	const val = procState.reduce((acc, [n, source, destn]) => {
		const vals: string[] = acc[source].slice(-n);
		const newsource = acc[source].slice(0, -n);
		const newdestn = [...acc[destn], ...vals.reverse()];

		return {
			...acc,
			[source]: newsource,
			[destn]: newdestn,
		};
	}, stackState);

	const ans = Object.values(val)
		.map((v) => v.pop())
		.join('');

	console.log('==================== PART 1 ====================');
	console.log('answer: ', ans);
}

[sample, input].map(parseInput).map(solve);
