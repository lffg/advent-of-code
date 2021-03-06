import { readFileSync } from 'fs';
import { print } from '../shared/out';

function walk(map: string[], right: number, down: number): number {
  let treesEncountered = 0;
  let movedRight = 0;
  const lineLength = map[0].length;

  for (let i = 0; i < map.length; i += down) {
    const currentLine = map[i];
    const currentBlock = currentLine[movedRight % lineLength];

    if (currentBlock === '#') {
      treesEncountered++;
    }

    movedRight += right;
  }

  return treesEncountered;
}

export function part1(map: string[]) {
  // Right 3, down 1.
  return walk(map, 3, 1);
}

export function part2(map: string[]) {
  return [
    [1, 1], // Right 1, down 1.
    [3, 1], // Right 3, down 1. (This is the slope you already checked.)
    [5, 1], // Right 5, down 1.
    [7, 1], // Right 7, down 1.
    [1, 2] // Right 1, down 2.
  ].reduce((a, [right, down]) => a * walk(map, right, down), 1);
}

const input = readFileSync(__dirname + '/input.txt', 'utf8')
  .trim()
  .split('\n');

print('Part 1', () => part1(input));
print('Part 2', () => part2(input));
