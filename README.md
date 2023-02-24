# zipper

This is a demo crate for the zipper immutable data structure. It supports immutable update and traversal of a binary tree in constant time. The API is very simple:

```{rust}
impl<T> Loc<T> {
    /// Traversal
    fn up(&self) -> Option<Self>;
    fn left(&self) -> Option<Self>;
    fn right(&self) -> Option<Self>;

    /// Insert a value at the current node
    fn set(&self, data: T) -> Self;

    /// Delete the part of the tree beneath the current node
    fn delete(&self) -> Self;
}
```

`cargo run` starts an interactive interpreter where you can manipulate an initially empty tree with the following commands:

- `u`, `up`: Go up the tree
- `l`, `left`: Go down the left branch
- `r`, `right`: Go down the right branch
- `set blah`: Sets the value of the current node to a string
- `d`, `del`, `delete`: Delete the tree underneath the current node

We support nice pretty-printing of the tree:

`* ^ ["x" . "y"] ^ ["z" "w" []]`

Here, the right-most list is the current tree that we are looking at. `^` separates the parent context on the left from the child on the right. The child replaces `.` in the parent. `*` is the empty parent, or "top context".
