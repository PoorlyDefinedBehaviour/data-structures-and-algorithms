object quicksort {
  private def sort[A](
      list: scala.collection.mutable.Seq[A],
      start: Int,
      end: Int
  )(implicit ordering: Ordering[A]): Unit = {
    if (start >= end) {
      return
    }

    var pivotIndex = start
    val pivotValue = list(end)

    for (index <- start to end) {
      val value = list(index)
      if (ordering.lt(value, pivotValue)) {
        val temp = list(pivotIndex)
        list.update(pivotIndex, value)
        list.update(index, temp)
        pivotIndex += 1
      }
    }

    val temp = list(pivotIndex)
    list.update(pivotIndex, list(end))
    list.update(end, temp)

    sort(list, start, pivotIndex - 1)
    sort(list, pivotIndex + 1, end)
  }

  def apply[A](list: Seq[A])(implicit ordering: Ordering[A]): Seq[A] = {
    val newList = scala.collection.mutable.Seq(list: _*)

    sort(newList, 0, list.length - 1)

    newList.toSeq
  }
}

object Main {
  def main(args: Array[String]): Unit = {
    println(quicksort(Seq(5, 4, 3, 2, 1)))
    println(quicksort(Seq(3, 2, 1)))
    println(quicksort(Seq[Int]()))

  }
}
