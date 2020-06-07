package linkedlist_test

import (
	"testing"

	linkedlist "."
)

func TestLength(t *testing.T) {
	var list = linkedlist.New()

	if list.Length() != 0 {
		t.Errorf("expected 0, got %d", list.Length())
	}

	list.Push(1)

	if list.Length() != 1 {
		t.Errorf("expected 0, got %d", list.Length())
	}
}

func TestIsEmpty(t *testing.T) {
	var list = linkedlist.New()

	if !list.IsEmpty() {
		t.Errorf("expected list to be empty")
	}

	list.Push(1)

	if list.IsEmpty() {
		t.Errorf("expected list not to be empty")
	}
}

func TestHead(t *testing.T) {
	var list = linkedlist.New()
	list.Push(1)
	list.Push(2)
	list.Push(3)

	var head = list.Head()

	if head.Value != 1 {
		t.Errorf("expected 1, got %d", head.Value)
	}
}

func TestTail(t *testing.T) {
	var list = linkedlist.New()
	list.Push(1)

	var tail = list.Tail()

	if tail.Value != 1 {
		t.Errorf("expected 1, got %d", tail.Value)
	}

	list.Push(2)
	list.Push(3)

	tail = list.Tail()

	if tail.Value != 3 {
		t.Errorf("expected 3, got %d", tail.Value)
	}
}

func TestPush(t *testing.T) {
	var list = linkedlist.New()
	list.Push(1)
	list.Push(2)
	list.Push(3)

	var head = list.Head()

	if head.Value != 1 {
		t.Errorf("expected 1, got %d", head.Value)
	}

	var tail = list.Tail()
	if tail.Value != 3 {
		t.Errorf("expected 3, got %d", head.Value)
	}

	if list.Length() != 3 {
		t.Errorf("expected length to be 3, got %d", list.Length())
	}
}

func TestIncludes(t *testing.T) {
	var list = linkedlist.New()

	list.Push(1)
	list.Push(2)
	list.Push(3)

	if !list.Includes(func(value interface{}) bool {
		return int(value.(int)) == 3
	}) {
		t.Errorf("expected list to include number 3")
	}

	if list.Includes(func(value interface{}) bool {
		return int(value.(int)) == -10
	}) {
		t.Errorf("expected list to not include number -10")
	}
}

func TestFillRange(t *testing.T) {
	var list = linkedlist.New()
	list.FillRange(0, 5)

	for i := 0; i < 5; i++ {
		if !list.Includes(func(value interface{}) bool {
			return int(value.(int)) == i
		}) {
			t.Errorf("expected list to include %d", i)
		}
	}
}

func TestAll(t *testing.T) {
	var list = linkedlist.New()
	list.FillRange(1, 10)

	if !list.All(func(value interface{}) bool {
		var number = int(value.(int))
		return number > 0 && number < 10
	}) {
		t.Errorf("expected list elements to be between 0 and 10")
	}
}

func TestAny(t *testing.T) {
	var list = linkedlist.New()
	list.FillRange(1, 10)

	if !list.Any(func(value interface{}) bool {
		var number = int(value.(int))
		return number == 9
	}) {
		t.Errorf("expected list to include number 9")
	}

	if list.Any(func(value interface{}) bool {
		var number = int(value.(int))
		return number == 10
	}) {
		t.Errorf("expected list to not include number 10")
	}
}

func TestNone(t *testing.T) {
	var list = linkedlist.New()
	list.FillRange(1, 10)

	if !list.None(func(value interface{}) bool {
		var number = int(value.(int))
		return number == 20
	}) {
		t.Errorf("expected none to return true")
	}

	if !list.None(func(value interface{}) bool {
		var number = int(value.(int))
		return number == 5
	}) {
		t.Errorf("expected none to return false")
	}
}

func TestRemoveAll(t *testing.T) {
	var list = linkedlist.New()
	list.FillRange(0, 5)

	list.RemoveAll(func(value interface{}) bool { return int(value.(int)) > 0 })

	if list.Length() != 1 {
		t.Errorf("expected list.Length() to be 1, got %d", list.Length())
	}

	if list.At(0) != 0 {
		t.Errorf("expected list.At(0) to be 0, got %d", list.At(0))
	}
}

func TestRemoveOne(t *testing.T) {
	var list = linkedlist.New()
	list.FillRange(0, 5)

	list.RemoveOne(func(value interface{}) bool { return int(value.(int)) == 0 })
	if list.At(0) != 1 {
		t.Errorf("expected list.At(0) to be 1, got %d", list.At(0))
	}

	list.RemoveOne(func(value interface{}) bool { return int(value.(int)) == 1 })
	if list.At(0) != 2 {
		t.Errorf("expected list.At(0) to be 2, got %d", list.At(0))
	}

	list.RemoveOne(func(value interface{}) bool { return int(value.(int)) == 2 })
	if list.At(0) != 3 {
		t.Errorf("expected list.At(0) to be 3, got %d", list.At(0))
	}

	if list.Length() != 2 {
		t.Errorf("expected list.Length() to be 2, got %d", list.Length())
	}
}

func TestAt(t *testing.T) {
	var list = linkedlist.New()
	list.FillRange(0, 5)

	for i := 0; i < 5; i++ {
		if list.At(i) != i {
			t.Errorf("expected %d to be at position %d", i, i)
		}
	}
}

func TestMap(t *testing.T) {
	var list = linkedlist.New()
	list.FillRange(0, 5)

	var squaredNumbers = list.Map(func(value interface{}) interface{} {
		var number = int(value.(int))
		return number * number
	})

	var square = func(number int) int { return number * number }

	for i := 0; i < 5; i++ {
		var target = int(squaredNumbers.At(i).(int))
		var mappedListValue = square(int(list.At(i).(int)))

		if mappedListValue != target {
			t.Errorf("expected %d to be  %d", mappedListValue, target)
		}
	}
}

func TestFilter(t *testing.T) {
	var list = linkedlist.New()
	list.FillRange(0, 5)

	var oddNumbers = list.Filter(func(value interface{}) bool {
		return int(value.(int))%2 == 0
	})

	if oddNumbers.Length() != 3 {
		t.Errorf("expected oddNumbers.Length() to be 3, got %d", oddNumbers.Length())
	}

	if oddNumbers.At(0) != 0 {
		t.Errorf("expected oddNumbers.At(0) to be 0, got %d", oddNumbers.At(0))
	}
	if oddNumbers.At(1) != 2 {
		t.Errorf("expected oddNumbers.At(2) to be 2, got %d", oddNumbers.At(1))
	}
	if oddNumbers.At(2) != 4 {
		t.Errorf("expected oddNumbers.At(4) to be 4, got %d", oddNumbers.At(2))
	}
}

func TestReduce(t *testing.T) {
	var list = linkedlist.New()
	list.FillRange(1, 10)

	var expected = 9 * (10) / 2
	var result = list.Reduce(0, func(accumulator interface{}, currentValue interface{}) interface{} {
		return int(accumulator.(int)) + int(currentValue.(int))
	})

	if result != expected {
		t.Errorf("expected %d, got %d", expected, result)
	}
}

func TestForEach(t *testing.T) {
	var list = linkedlist.New()
	list.FillRange(1, 10)

	var expected = 9 * (10) / 2
	var result = 0

	list.ForEach(func(value interface{}) {
		result = result + int(value.(int))
	})

	if result != expected {
		t.Errorf("expected %d, got %d", expected, result)
	}
}
