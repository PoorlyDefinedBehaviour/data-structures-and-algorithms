object Main {
  def factorial(x: Int): Int = {
    def factorialImpl(y: Int, accumulator: Int): Int = {
      if (y <= 1) {
        accumulator
      } else {
        factorialImpl(y - 1, y * accumulator)
      }
    }

    factorialImpl(x, 1)
  }
  
  def main(args: Array[String]): Unit = {
    println(factorial(5))
  }
}
