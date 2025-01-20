// package main

// type BST struct {
// 	Value int
// 	Left  *BST
// 	Right *BST
// }


// func (tree *BST) FindClosestValue(target int) int {
// 	return tree.findClosestValue(target, tree.Value)
// }
// // Avarage: O(log(n)) time | O(log(n)) space
// // Worst: O(n) time | O(n) space
// func (tree *BST) findClosestValue(target int, closest int) int {
// 	if abcdif(target, closest) > abcdif(target, tree.Value) {
// 		closest = tree.Value
// 	}

// 	if target<tree.Value && tree.Left != nil {
// 		return tree.Left.findClosestValue(target, closest)
// 	}  else if target>tree.Value && tree.Left != nil { 
// 		return tree.Right.findClosestValue(target, closest)
// 	}
// 	return closest
// }

// func abcdif(a, b int) int {
// 	if a>b {
// 		return a-b
// 	}

// 	return b-a
// }







// func main() {
// 	println("dsdsd")
// }


package main

type BST struct {
	Value int
	Left  *BST
	Right *BST
}

// FindClosestValue iteratively with O(n) time and O(1) space
func (tree *BST) FindClosestValue(target int) int {
	closest := tree.Value
	currentNode := tree

	for currentNode != nil {
		if abs(target-closest) > abs(target-currentNode.Value) {
			closest = currentNode.Value
		}

		if target < currentNode.Value {
			currentNode = currentNode.Left
		} else if target > currentNode.Value {
			currentNode = currentNode.Right
		} else {
			break // Exact match found
		}
	}
	return closest
}

func abs(a int) int {
	if a < 0 {
		return -a
	}
	return a
}

func main() {
	// Example: Create BST
	bst := &BST{
		Value: 10,
		Left: &BST{
			Value: 5,
			Left: &BST{
				Value: 2,
			},
			Right: &BST{
				Value: 5,
			},
		},
		Right: &BST{
			Value: 15,
			Right: &BST{
				Value: 22,
			},
		},
	}

	// Find closest value
	target := 12
	println("The closest value to", target, "is", bst.FindClosestValue(target))
}
