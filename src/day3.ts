import { readFileStr } from 'https://deno.land/std/fs/mod.ts'
import { assertEquals } from 'https://deno.land/std/testing/asserts.ts'

const input = await readFileStr('input/2019/day3.txt')

interface Point {
  x: number
  y: number
}

interface HorizontalSegment {
  axis: 'horizontal'
  direction: 'right' | 'left'
  distance: number
  xLow: number
  xHigh: number
  y: number
}

interface VerticalSegment {
  axis: 'vertical'
  direction: 'up' | 'down'
  distance: number
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
        return { axis: 'horizontal', direction: 'right', distance, xLow: x - distance, xHigh: x, y }
      case 'L':
        x -= distance
        return { axis: 'horizontal', direction: 'left', distance, xLow: x, xHigh: x + distance, y }
      case 'U':
        y += distance
        return { axis: 'vertical', direction: 'up', distance, x, yLow: y - distance, yHigh: y }
      case 'D':
        y -= distance
        return { axis: 'vertical', direction: 'down', distance, x, yLow: y, yHigh: y + distance }
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

function segmentOffsetToPoint (segment: Segment, point: Point): number {
  switch (segment.direction) {
    case 'right': return point.x - segment.xLow
    case 'left': return segment.xHigh - point.x
    case 'up': return point.y - segment.yLow
    case 'down': return segment.yHigh - point.y
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

function part2 (input: string): number {
  const [firstSegments, secondSegments] = parseProgram(input)

  let firstDistance = 0
  let minimumDistance = Number.MAX_SAFE_INTEGER

  for (const segment of firstSegments) {
    let secondDistance = 0

    for (const other of secondSegments) {
      const intersection = segmentIntersection(segment, other)

      if (intersection) {
        const firstOffset = segmentOffsetToPoint(segment, intersection)
        const secondOffset = segmentOffsetToPoint(other, intersection)

        const distance = (firstDistance + firstOffset) + (secondDistance + secondOffset)

        if (distance > 0 && distance < minimumDistance) {
          minimumDistance = distance
        }
      }

      secondDistance += other.distance
    }

    firstDistance += segment.distance
  }

  return minimumDistance
}

assertEquals(part1('R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83'), 159)
assertEquals(part1('R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7'), 135)

console.log(part1(input))

assertEquals(part2('R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83'), 610)
assertEquals(part2('R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7'), 410)

console.log(part2(input))
