object Memes {
  implicit class Pipe[A](a: A) {
    def |>[B](f: A => B): B = f(a)
  }
}

object Main extends App {
  import Memes._

  def double(x: Int) = x + x

  2 |> double |> double |> println
}
