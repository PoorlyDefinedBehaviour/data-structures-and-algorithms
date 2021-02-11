object quicksort {
  def apply[A](list: Seq[A])(implicit ordering: Ordering[A]): Seq[A] =
    list match {
      case Nil => Nil
      case (pivot :: tail) => {
        val (smaller, greater): (Seq[A], Seq[A]) =
          tail.partition(x => ordering.lt(x, pivot))

        quicksort(smaller) ++ Seq(pivot) ++ quicksort(greater)
      }
    }
}

object Main {
  def main(args: Array[String]): Unit = {
    println(quicksort(Seq(5, 4, 3, 2, 1)))
    println(quicksort(Seq(3, 2, 1)))
    println(quicksort(Seq[Int]()))
  }
}
