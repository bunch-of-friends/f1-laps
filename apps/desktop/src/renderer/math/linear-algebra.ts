export interface Point {
  x: number;
  y: number;
}

function subtract(a: Point, b: Point) {
  return {
    x: a.x - b.x,
    y: a.y - b.y
  };
}

function determinant(u: Point, v: Point) {
  return u.x * v.y - u.y * v.x;
}

function magnitude(u: Point) {
  return Math.sqrt(u.x ** 2 + u.y ** 2);
}

export function isColinear(a: Point, b: Point, c: Point) {
  const ab = subtract(b, a);
  const ac = subtract(c, a);

  return Math.abs(determinant(ab, ac)) / magnitude(ac) < 0.001;
}
