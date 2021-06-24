package minstack

/*
https://leetcode.com/problems/min-stack/

Design a stack that supports push, pop, top, and retrieving the minimum element in constant time.

Implement the MinStack class:

    MinStack() initializes the stack object.
    void push(val) pushes the element val onto the stack.
    void pop() removes the element on the top of the stack.
    int top() gets the top element of the stack.
    int getMin() retrieves the minimum element in the stack.

Example 1:

Input
["MinStack","push","push","push","getMin","pop","top","getMin"]
[[],[-2],[0],[-3],[],[],[],[]]

Output
[null,null,null,null,-3,null,0,-2]

Explanation
MinStack minStack = new MinStack();
minStack.push(-2);
minStack.push(0);
minStack.push(-3);
minStack.getMin(); // return -3
minStack.pop();
minStack.top();    // return 0
minStack.getMin(); // return -2

Constraints:

    -231 <= val <= 231 - 1
    Methods pop, top and getMin operations will always be called on non-empty stacks.
    At most 3 * 104 calls will be made to push, pop, top, and getMin.
*/
type entry struct {
	value                           int
	minimumValueWhenEntryWasCreated int
}

// time O(1)
// space O(n)
type MinStack struct {
	stack        []entry
	minimumValue int
}

func New() MinStack {
	return MinStack{stack: make([]entry, 0)}
}

// time O(1)
// space O(1)
func (minStack *MinStack) Push(value int) {
	if len(minStack.stack) == 0 {
		minStack.stack = append(minStack.stack, entry{
			value:                           value,
			minimumValueWhenEntryWasCreated: value,
		})

		minStack.minimumValue = value
	} else {
		minStack.stack = append(minStack.stack, entry{
			value:                           value,
			minimumValueWhenEntryWasCreated: minStack.minimumValue,
		})

		if value < minStack.minimumValue {
			minStack.minimumValue = value
		}
	}
}

// time O(1)
// space O(1)
func (minStack *MinStack) Pop() int {
	entry := minStack.stack[len(minStack.stack)-1]

	minStack.stack = minStack.stack[:len(minStack.stack)-1]

	minStack.minimumValue = entry.minimumValueWhenEntryWasCreated

	return entry.value
}

// time O(1)
// space O(1)
func (minStack *MinStack) Top() int {
	return minStack.stack[len(minStack.stack)-1].value
}

// time O(1)
// space O(1)
func (minStack *MinStack) Min() int {
	return minStack.minimumValue
}
