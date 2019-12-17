import { readFileStr } from 'https://deno.land/std/fs/mod.ts'
import { assertEquals } from 'https://deno.land/std/testing/asserts.ts'

const input = await readFileStr('input/2019/day2.txt')

function runProgram (source: string, noun: number, verb: number): number {
  const memory = input.split(',').map(s => Number.parseInt(s))
  let eip = 0

  memory[1] = noun
  memory[2] = verb

  while (true) {
    switch (memory[eip]) {
      case 1: {
        const lhsPtr = memory[eip + 1]
        const rhsPtr = memory[eip + 2]
        const outPtr = memory[eip + 3]

        memory[outPtr] = memory[lhsPtr] + memory[rhsPtr]
        eip += 4

        break
      }
      case 2: {
        const lhsPtr = memory[eip + 1]
        const rhsPtr = memory[eip + 2]
        const outPtr = memory[eip + 3]

        memory[outPtr] = memory[lhsPtr] * memory[rhsPtr]
        eip += 4

        break
      }
      case 99: {
        return memory[0]
      }
    }
  }
}

function part1 (input: string): number {
  return runProgram(input, 12, 2)
}

function part2 (input: string): number {
  for (let noun = 0; noun <= 99; noun++) {
    for (let verb = 0; verb <= 99; verb++) {
      if (runProgram(input, noun, verb) === 19690720) {
        return 100 * noun + verb
      }
    }
  }
}

console.log(part1(input))
console.log(part2(input))
