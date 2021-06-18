package main

import "sync"

/*
Design a data structure that follows the constraints of a Least Recently Used (LRU) cache.
Implement the LRUCache class:
    LRUCache(int capacity) Initialize the LRU cache with positive size capacity.
    int get(int key) Return the value of the key if the key exists, otherwise return -1.
		void put(int key, int value) Update the value of the key if the key exists. Otherwise,
		add the key-value pair to the cache.
		If the number of keys exceeds the capacity from this operation, evict the least recently used key.
The functions get and put must each run in O(1) average time complexity.
*/

type Node struct {
	key      int
	value    int
	previous *Node
	next     *Node
}

type LRUCache struct {
	// head is a sentinel that's never filled.
	// the next node after head is the most recently used.
	head *Node
	// tail is a sentinel that's never filled.
	// the node before tail is the least recently used.
	tail *Node
	// capacity is the max number of items the cache can have
	capacity int
	// items is the actual cache
	items map[int]*Node
	// multiple readers mutex
	mutex sync.RWMutex
}

// time O(1)
// space O(1)
func (cache *LRUCache) Get(key int) int {
	cache.mutex.RLock()
	defer cache.mutex.RUnlock()

	item, ok := cache.items[key]
	if !ok {
		return -1
	}

	return item.value
}

// time O(1)
// space O(1)
func (cache *LRUCache) remove(node *Node) {
	delete(cache.items, node.key)

	previous := node.previous
	next := node.next

	previous.next, next.previous = next, previous
}

// time O(1)
// space O(1)
func (cache *LRUCache) update(key, value int) {
	node := cache.items[key]

	cache.remove(node)
	cache.add(key, value)
}

func (cache *LRUCache) getLeastRecentlyUsedNode() *Node {
	return cache.tail.previous
}

func (cache *LRUCache) getMostRecentlyUsedNode() *Node {
	return cache.head.next
}

// time O(1)
// space O(1)
func (cache *LRUCache) Has(key int) bool {
	cache.mutex.RLock()
	defer cache.mutex.RUnlock()

	_, ok := cache.items[key]
	return ok
}

func (cache *LRUCache) add(key, value int) {
	node := &Node{
		key:   key,
		value: value,
	}

	cache.items[node.key] = node

	nodeAfterHead := cache.head.next

	// head points to the most recently used node
	cache.head.next, nodeAfterHead.previous = node, node

	// the previous most recenly used node becomes
	// the second most recenly used node
	node.next, node.previous = nodeAfterHead, cache.head
}

// time O(1)
// space O(1)
func (cache *LRUCache) Put(key, value int) {
	cache.mutex.Lock()
	defer cache.mutex.Unlock()

	if len(cache.items) == cache.capacity {
		cache.remove(cache.getLeastRecentlyUsedNode())
	}

	if _, ok := cache.items[key]; ok {
		cache.update(key, value)
		return
	}

	cache.add(key, value)
}

// time O(1)
// space O(1)
func (cache *LRUCache) Remove(key int) {
	cache.mutex.Lock()
	defer cache.mutex.Unlock()

	item, ok := cache.items[key]
	if !ok {
		return
	}

	cache.remove(item)
}

func NewLRUCache(capacity int) LRUCache {
	if capacity < 1 {
		panic("cache capacity must be a positive integer")
	}

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

	return LRUCache{
		head:     head,
		tail:     tail,
		capacity: capacity,
		items:    make(map[int]*Node),
		mutex:    sync.RWMutex{},
	}
}

func main() {
}
