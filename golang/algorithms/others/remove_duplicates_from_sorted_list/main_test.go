package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func listsEqual(a, b *ListNode) bool {
	currentA := a
	currentB := b

	for {
		if currentA == nil {
			return currentB == nil
		}

		if currentA.Val != currentB.Val {
			return false
		}

		currentA = currentA.Next
		currentB = currentB.Next
	}
}

func Test_deleteDuplicates(t *testing.T) {
	t.Run("when list is empty", func(t *testing.T) {
		t.Run("returns the empty list", func(t *testing.T) {
			var list *ListNode = nil
			assert.Nil(t, deleteDuplicates(list))
		})
	})

	t.Run("when list contains duplicates", func(t *testing.T) {
		t.Run("removes duplicates", func(t *testing.T) {
			list := &ListNode{
				Val: 1,
				Next: &ListNode{
					Val: 2,
					Next: &ListNode{
						Val:  2,
						Next: nil,
					},
				},
			}

			expected := &ListNode{
				Val: 1,
				Next: &ListNode{
					Val:  2,
					Next: nil,
				},
			}

			actual := deleteDuplicates(list)

			assert.True(t, listsEqual(expected, actual))
		})
	})

	t.Run("when list contains no duplicates", func(t *testing.T) {
		t.Run("returns the list unmodified", func(t *testing.T) {
			list := &ListNode{
				Val: 1,
				Next: &ListNode{
					Val: 2,
					Next: &ListNode{
						Val:  3,
						Next: nil,
					},
				},
			}

			expected := &ListNode{
				Val: 1,
				Next: &ListNode{
					Val: 2,
					Next: &ListNode{
						Val:  3,
						Next: nil,
					},
				},
			}

			actual := deleteDuplicates(list)

			assert.True(t, listsEqual(expected, actual))
		})
	})
}
