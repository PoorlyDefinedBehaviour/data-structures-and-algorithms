object Church {
  type Boolean[A] = (A, A) => A

  def True[A](Then: A, Else: A): A = Then
  def False[A](Then: A, Else: A): A = Else
  def If[A](condition: Boolean[A], Then: A, Else: A): A =
    condition(Then, Else)
  def Not[A](bool: Boolean[Boolean[A]]): Boolean[A] =
    bool(False, True)
  def And[A](a: Boolean[Boolean[A]], b: Boolean[A]): Boolean[A] =
    a(b, False)
  def Or[A](a: Boolean[Boolean[A]], b: Boolean[A]): Boolean[A] =
    a(True, b)
}

object Main {
  def main(args: Array[String]): Unit = {
    println(Church.True(1, 2)) // 1
    println(Church.False(1, 2)) // 2
    println(Church.If[Int](Church.True, 1, 2)) // 1
    println(Church.If[Int](Church.Not(Church.False), 1, 2)) // 1
    println(Church.If[Int](Church.And(Church.True, Church.True), 1, 2)) // 1
    println(Church.If[Int](Church.Or(Church.True, Church.True), 2, 1)) // 2
    println(Church.If[Int](Church.And(Church.True, Church.False), 1, 2)) // 2
  }
}
