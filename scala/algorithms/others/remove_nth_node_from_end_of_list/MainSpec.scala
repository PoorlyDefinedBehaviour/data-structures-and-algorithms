import collection.mutable.Stack
import org.scalatest._
import flatspec._
import matchers._

class MainSpec extends AnyFlatSpec with should.Matchers {
  "List.removeNthFromEnd" should "remove nth element starting from the end of the list" in {
    List(1, 2, 3, 4, 5).removeNthFromEnd(2) should be(List(1, 2, 3, 5))

    List(1).removeNthFromEnd(1) should be(List())

    List(1).removeNthFromEnd(5) should be(List(1))

    List().removeNthFromEnd(1) should be(List())
  }
}
