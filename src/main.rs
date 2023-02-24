use std::{default::Default, rc::Rc};

#[derive(Clone)]
enum ContextType {
    Left,
    Right,
}

#[derive(Clone)]
struct ContextBox<T> {
    ty: ContextType,
    data: Rc<T>,
    context: Context<T>,
    tree: Tree<T>,
}

struct Context<T>(Option<Rc<ContextBox<T>>>);

impl<T: std::fmt::Debug> std::fmt::Display for Context<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self(None) => write!(f, "*"),
            Self(Some(context)) => match context.ty {
                ContextType::Left => write!(f, "{} ^ [{:?} {} .]", context.context, context.data, context.tree),
                ContextType::Right => write!(f, "{} ^ [{:?} . {}]", context.context, context.data, context.tree),
            }
        }
    }
}

impl<T> Clone for Context<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Default for Context<T> {
    fn default() -> Self {
        Self(None)
    }
}

impl<T> Context<T> {
    fn new(ty: ContextType, data: Rc<T>, context: Self, tree: Tree<T>) -> Self {
        Self(Some(Rc::new(ContextBox {
            ty,
            data,
            context,
            tree,
        })))
    }

    fn left(data: Rc<T>, context: Self, tree: Tree<T>) -> Self {
        Self::new(ContextType::Left, data, context, tree)
    }

    fn right(data: Rc<T>, context: Self, tree: Tree<T>) -> Self {
        Self::new(ContextType::Right, data, context, tree)
    }
}

#[derive(Clone)]
struct TreeBox<T> {
    data: Rc<T>,
    left: Tree<T>,
    right: Tree<T>,
}

struct Tree<T>(Option<Rc<TreeBox<T>>>);

impl<T: std::fmt::Debug> std::fmt::Display for Tree<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            None => write!(f, "[]"),
            Some(tree) => if tree.left.0.is_some() || tree.right.0.is_some() {
                write!(f, "[{:?} {} {}]", tree.data, tree.left, tree.right)
            } else {
                write!(f, "{:?}", tree.data)
            }
        }
    }
}

impl<T> Clone for Tree<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Tree<T> {
    fn new(data: Rc<T>, left: Tree<T>, right: Tree<T>) -> Self {
        Self(Some(Rc::new(TreeBox { data, left, right })))
    }
}

impl<T> Default for Tree<T> {
    fn default() -> Self {
        Self(None)
    }
}

#[derive(Clone, Default)]
struct Loc<T> {
    context: Context<T>,
    tree: Tree<T>,
}

impl<T: std::fmt::Debug> std::fmt::Display for Loc<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ^ {}", self.context, self.tree)
    }
}

impl<T> Loc<T> {
    fn new(context: Context<T>, tree: Tree<T>) -> Self {
        Self { context, tree }
    }

    fn set(&self, data: T) -> Self {
        let tree = match &self.tree {
            Tree(None) => Tree::new(Rc::new(data), Tree::default(), Tree::default()),
            Tree(Some(tree)) => Tree::new(Rc::new(data), tree.left.clone(), tree.right.clone()),
        };
        Self::new(self.context.clone(), tree)
    }

    fn up(&self) -> Option<Self> {
        match &self.context {
            Context(None) => None,
            Context(Some(context)) => {
                let (left, right) = match context.ty {
                    ContextType::Left => (context.tree.clone(), self.tree.clone()),
                    ContextType::Right => (self.tree.clone(), context.tree.clone()),
                };
                let tree = Tree::new(context.data.clone(), left, right);
                let context = context.context.clone();
                Some(Self::new(context, tree))
            }
        }
    }

    fn left(&self) -> Option<Self> {
        self.tree.0.as_ref().map(|tree| {
            let context =
                Context::right(tree.data.clone(), self.context.clone(), tree.right.clone());
            let tree = tree.left.clone();
            Self::new(context, tree)
        })
    }

    fn right(&self) -> Option<Self> {
        self.tree.0.as_ref().map(|tree| {
            let context = Context::left(tree.data.clone(), self.context.clone(), tree.left.clone());
            let tree = tree.right.clone();
            Self::new(context, tree)
        })
    }
}

fn main() {
    fn input() -> String {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s
    }

    let mut tree: Loc<String> = Loc::default();
    loop {
        println!("{}", tree);
        let s = input();
        let words: Vec<&str> = s.split_whitespace().collect();
        match words.as_slice() {
            &["set", value] => {
                tree = tree.set(value.into());
            }
            &["l" | "left"] => {
                tree = tree.left().unwrap_or(tree);
            }
            &["r" | "right"] => {
                tree = tree.right().unwrap_or(tree);
            }
            &["u" | "up"] => {
                tree = tree.up().unwrap_or(tree);
            }
            _ => println!("Invalid command: `{s}`"),
        }
    }
}
