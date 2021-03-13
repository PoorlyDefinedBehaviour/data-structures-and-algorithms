import scala.annotation.tailrec

object Graph {
  type AdjencyList = List[List[Int]]

  def isConnected(graph: AdjencyList): Boolean = {
    var reachableNodes = graph.flatten.toSet

    (0 until graph.length).forall(reachableNodes.contains(_))
  }
}

object Main extends App {
  /*
          1 - 3 - 5
        /        /
      0        /
        \    /
          2 - 4 - 6
   */
  val connectedGraph = List(
    List(1, 2),
    List(0, 3),
    List(0, 4),
    List(1, 5),
    List(2, 6),
    List(3, 4),
    List(6, 4)
  )

  /*
      1 - 3 - 5
    /         /
  0        /
   \     /
    2 - 4   6
   */
  val unconnectedGraph =
    List(
      List(1, 2),
      List(0, 3),
      List(0, 4),
      List(1, 5),
      List(2),
      List(3, 4),
      List()
    )

  println(Graph.isConnected(connectedGraph))
  println(Graph.isConnected(unconnectedGraph))
}
