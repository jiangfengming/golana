package main

import "fmt2"

type Node struct {
	prev *Node
	next *Node
	key  interface{}
}

type List struct {
	head *Node
	tail *Node
	t1   *Node
	t2   *Node
}

var glist = List{}

func (L *List) Insert(key interface{}) {
	list := &Node{
		next: L.head,
		key:  key,
	}
	if L.head != nil {
		L.head.prev = list
	}
	L.head = list

	l := L.head
	for l.next != nil {
		l = l.next
	}
	L.tail = l
}

func (l *List) Display() {
	list := l.head
	for list != nil {
		fmt2.Println("%+v ->", list.key)
		list = list.next
	}
}

func Display(list *Node) {
	for list != nil {
		fmt2.Println("%v ->", list.key)
		list = list.next
	}
}

func ShowBackwards(list *Node) {
	for list != nil {
		fmt2.Println("%v <-", list.key)
		list = list.prev
	}
}

func (l *List) Reverse() {
	curr := l.head
	var prev *Node
	l.tail = l.head

	for curr != nil {
		next := curr.next
		curr.next = prev
		prev = curr
		curr = next
	}
	l.head = prev
	Display(l.head)
}

func main() {

	link := List{}
	link.Insert(1)
	link.Insert(3)
	link.Insert(5)
	link.Insert(7)
	link.Insert(9)

	//fmt2.Println("\n==============================\n")
	//fmt2.Println("Head: %v\n", link.head.key)
	//fmt2.Println("Tail: %v\n", link.tail.key)
	//fmt2.Println("ttt: %v\n", link.ttt.key)
	//link.Display()
	//fmt2.Println("\n==============================\n")
	//fmt2.Println("head: %v\n", link.head.key)
	//fmt2.Println("tail: %v\n", link.tail.key)
	link.Reverse()
	//fmt2.Println("\n==============================\n")

}
