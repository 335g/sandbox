
let rows = "ABCDEFGHI"
let cols = "123456789"

struct SquareMap[T] {
  contents: Array[T]
}

enum List[T] {
  Nil
  Cons(T, List[T])
}

fn assign(values : SquareMap[List[Char]], key : String, val : Char) -> Bool {
  ...
}