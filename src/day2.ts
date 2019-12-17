import { readFileStr } from 'https://deno.land/std/fs/mod.ts'
import { assertEquals } from 'https://deno.land/std/testing/asserts.ts'

const input = await readFileStr('input/2019/day2.txt')

function part1 (input: string): number {
  const memory = input.split(',').map(s => Number.parseInt(s))
  let eip = 0

  memory[1] = 12
  memory[2] = 2

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

console.log(part1(input))
