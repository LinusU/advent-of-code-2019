import { readFileStr } from 'https://deno.land/std/fs/mod.ts'
import { assertEquals } from 'https://deno.land/std/testing/asserts.ts'

const input = await readFileStr('input/2019/day1.txt')

function part1 (input: string): number {
  const masses = input.trim().split('\n').map(s => Number.parseInt(s))
  const fuel = masses.reduce((mem, mass) => mem + Math.floor(mass / 3) - 2, 0)

  return fuel
}

function part2 (input: string): number {
  function getFuel (mass: number) {
    if (mass < 9) return 0
    const fuel = Math.floor(mass / 3) - 2
    return fuel + getFuel(fuel)
  }

  const masses = input.trim().split('\n').map(s => Number.parseInt(s))
  const fuel = masses.reduce((mem, mass) => mem + getFuel(mass), 0)

  return fuel
}

assertEquals(part1('12\n'), 2)
assertEquals(part1('14\n'), 2)
assertEquals(part1('1969\n'), 654)
assertEquals(part1('100756\n'), 33583)
assertEquals(part1('12\n14\n1969\n100756\n'), 2 + 2 + 654 + 33583)

console.log(part1(input))

assertEquals(part2('14\n'), 2)
assertEquals(part2('1969\n'), 966)
assertEquals(part2('100756\n'), 50346)
assertEquals(part2('14\n1969\n100756\n'), 2 + 966 + 50346)

console.log(part2(input))
