import { readFileStr } from 'https://deno.land/std/fs/mod.ts'
import { assertEquals } from 'https://deno.land/std/testing/asserts.ts'

const input = await readFileStr('input/2019/day3.txt')

interface Point {
  x: number
  y: number
}

interface HorizontalSegment {
  axis: 'horizontal'
  xLow: number
  xHigh: number
  y: number
}

interface VerticalSegment {
  axis: 'vertical'
  x: number
  yLow: number
  yHigh: number
}

type Segment = HorizontalSegment | VerticalSegment

function parseSegments (source: string): Segment[] {
  let x = 0
  let y = 0

  return source.split(',').map((source) => {
    const direction = source[0]
    const distance = Number(source.slice(1))

    switch (direction) {
      case 'R':
        x += distance
        return { axis: 'horizontal', xLow: x - distance, xHigh: x, y }
      case 'L':
        x -= distance
        return { axis: 'horizontal', xLow: x, xHigh: x + distance, y }
      case 'U':
        y += distance
        return { axis: 'vertical', x, yLow: y - distance, yHigh: y }
      case 'D':
        y -= distance
        return { axis: 'vertical', x, yLow: y, yHigh: y + distance }
    }
  })
}

function parseProgram (source: string): [Segment[], Segment[]] {
  const [first, second] = source.split('\n')
  return [parseSegments(first), parseSegments(second)]
}

function segmentIntersection (lhs: Segment, rhs: Segment): Point | null {
  if (lhs.axis === rhs.axis) return null

  const { x, yLow, yHigh } = (lhs.axis === 'vertical' ? lhs : rhs) as VerticalSegment
  const { xLow, xHigh, y } = (lhs.axis === 'horizontal' ? lhs : rhs) as HorizontalSegment

  if ((xLow <= x && x <= xHigh) && (yLow <= y && y <= yHigh)) {
    return { x, y }
  } else {
    return null
  }
}

function part1 (input: string): number {
  const [firstSegments, secondSegments] = parseProgram(input)

  let minimumDistance = Number.MAX_SAFE_INTEGER

  for (const segment of firstSegments) {
    for (const other of secondSegments) {
      const intersection = segmentIntersection(segment, other)

      if (intersection) {
        const distance = Math.abs(intersection.x) + Math.abs(intersection.y)

        if (distance > 0 && distance < minimumDistance) {
          minimumDistance = distance
        }
      }
    }
  }

  return minimumDistance
}

assertEquals(part1('R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83'), 159)
assertEquals(part1('R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7'), 135)

console.log(part1(input))
