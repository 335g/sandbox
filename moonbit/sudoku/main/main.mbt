
pub fn ij_assign(values : SquareMap[List[Char]], i : Int, j : Int, val : Int) -> Bool {
  let key = rows[i].to_string() + cols[j].to_string()
  let val = val.to_string()[0]

  assign(values, key, val)
}

pub fn aaa() -> String {
  println("Pushed")
  "Hello!"
}

fn main {
  println("a")
}
