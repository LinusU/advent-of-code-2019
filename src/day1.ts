import { readFileStr } from 'https://deno.land/std/fs/mod.ts'
import { assertEquals } from 'https://deno.land/std/testing/asserts.ts'

const input = await readFileStr('input/2019/day1.txt')

function part1 (input: string): number {
  const masses = input.trim().split('\n').map(s => Number.parseInt(s))
  const fuel = masses.reduce((mem, mass) => mem + Math.floor(mass / 3) - 2, 0)

  return fuel
}

assertEquals(part1('12\n'), 2)
assertEquals(part1('14\n'), 2)
assertEquals(part1('1969\n'), 654)
assertEquals(part1('100756\n'), 33583)
assertEquals(part1('12\n14\n1969\n100756\n'), 2 + 2 + 654 + 33583)

console.log(part1(input))
