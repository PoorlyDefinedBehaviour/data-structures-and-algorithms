package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestLRUCache_Put(t *testing.T) {
	t.Parallel()

	t.Run("when key is not in cache, should add it", func(t *testing.T) {
		t.Parallel()

		cache := NewLRUCache(2)

		cache.Put(1, 2)

		mru := cache.getMostRecentlyUsedNode()

		assert.Equal(t, 1, mru.key)
		assert.Equal(t, 2, mru.value)

		lru := cache.getLeastRecentlyUsedNode()

		assert.Equal(t, 1, lru.key)
		assert.Equal(t, 2, lru.value)

		cache.Put(3, 4)

		mru = cache.getMostRecentlyUsedNode()

		assert.Equal(t, 3, mru.key)
		assert.Equal(t, 4, mru.value)

		lru = cache.getLeastRecentlyUsedNode()

		assert.Equal(t, 1, lru.key)
		assert.Equal(t, 2, lru.value)
	})

	t.Run("when key is already in the cache, should update it and make it the most recently used", func(t *testing.T) {
		t.Parallel()

		cache := NewLRUCache(5)

		cache.Put(1, 2)

		mru := cache.getMostRecentlyUsedNode()

		assert.Equal(t, 1, mru.key)
		assert.Equal(t, 2, mru.value)

		lru := cache.getLeastRecentlyUsedNode()

		assert.Equal(t, 1, lru.key)
		assert.Equal(t, 2, lru.value)

		cache.Put(3, 4)

		mru = cache.getMostRecentlyUsedNode()

		assert.Equal(t, 3, mru.key)
		assert.Equal(t, 4, mru.value)

		lru = cache.getLeastRecentlyUsedNode()

		assert.Equal(t, 1, lru.key)
		assert.Equal(t, 2, lru.value)

		cache.Put(1, 2)

		mru = cache.getMostRecentlyUsedNode()

		assert.Equal(t, 1, mru.key)
		assert.Equal(t, 2, mru.value)

		lru = cache.getLeastRecentlyUsedNode()

		assert.Equal(t, 3, lru.key)
		assert.Equal(t, 4, lru.value)
	})

	t.Run("when cache is full, removes the least recently used key", func(t *testing.T) {
		t.Parallel()

		cache := NewLRUCache(3)

		cache.Put(10, 2)
		cache.Put(20, 2)
		cache.Put(30, 2)

		lru := cache.getLeastRecentlyUsedNode()

		assert.Equal(t, 10, lru.key)
		assert.Equal(t, 2, lru.value)

		cache.Put(40, 2)

		mru := cache.getMostRecentlyUsedNode()

		assert.Equal(t, 40, mru.key)
		assert.Equal(t, 2, mru.value)

		lru = cache.getLeastRecentlyUsedNode()

		assert.Equal(t, 20, lru.key)
		assert.Equal(t, 2, lru.value)
	})
}

func TestNewLRUCache(t *testing.T) {
	t.Parallel()

	cache := NewLRUCache(10)

	assert.Equal(t, 10, cache.capacity)

	head := &Node{
		key:      -1,
		value:    -1,
		previous: nil,
		next:     nil,
	}

	tail := &Node{
		key:      -1,
		value:    -1,
		previous: nil,
		next:     nil,
	}

	head.next, tail.previous = tail, head

	assert.Equal(t, head, cache.head)
	assert.Equal(t, tail, cache.tail)
	assert.Equal(t, 0, len(cache.items))
}

func TestLRUCache_Has(t *testing.T) {
	t.Parallel()

	cache := NewLRUCache(10)

	assert.Equal(t, false, cache.Has(10))

	cache.Put(10, 1)

	assert.Equal(t, true, cache.Has(10))

	cache.Remove(10)

	assert.Equal(t, false, cache.Has(10))
}

func TestLRUCache_Get(t *testing.T) {
	t.Parallel()

	cache := NewLRUCache(10)

	assert.Equal(t, -1, cache.Get(10))

	cache.Put(10, 32)

	assert.Equal(t, 32, cache.Get(10))

	cache.Remove(10)

	assert.Equal(t, -1, cache.Get(10))
}

func TestLRUCache_Remove(t *testing.T) {
	t.Parallel()

	cache := NewLRUCache(10)

	cache.Put(10, 20)

	assert.Equal(t, 1, len(cache.items))

	cache.Remove(10)

	assert.Equal(t, 0, len(cache.items))

	assert.Equal(t, cache.head, cache.getLeastRecentlyUsedNode())
	assert.Equal(t, cache.tail, cache.getMostRecentlyUsedNode())
}
