import scala.annotation.tailrec

sealed trait List[+A] { self =>
  def foldRight[B](accum: B)(f: (A, B) => B): B =
    self match {
      case Nil => accum
      case Cons(head, tail) =>
        f(head, tail.foldRight(accum)(f))
    }

  def removeNthFromEnd(n: Int): List[A] = {
    val (_, list) = self
      .foldRight((1, Nil: List[A]))((x, args) => {
        val (currentIndex, xs) = args

        (currentIndex + 1, if (currentIndex == n) xs else Cons(x, xs))

      })

    list
  }
}

case object Nil extends List[Nothing]
case class Cons[+A](head: A, tail: List[A]) extends List[A]

object List {
  def apply[A](xs: A*): List[A] = {
    if (xs.isEmpty) {
      Nil
    } else {
      Cons(xs.head, apply(xs.tail: _*))
    }
  }
}

object Main extends App {

  /*
  Given the head of a linked list, remove the nth node from the end of the list and return its head.

  Follow up: Could you do this in one pass?

  Example 1:

  Input: head = [1,2,3,4,5], n = 2
  Output: [1,2,3,5]
   */
  println(List(1, 2, 3, 4, 5).removeNthFromEnd(2))
}
