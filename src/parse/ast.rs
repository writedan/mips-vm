#[derive(Debug)]
pub struct ASTree<T> {
	root: T,
	children: Vec<ASTNode<T>>
}

#[derive(Debug)]
pub enum ASTNode<T> {
	Tree(ASTree<T>),
	Node(T)
}

impl<T> ASTree<T> {
	pub fn new(root: T) -> ASTree<T> {
		ASTree {
			root,
			children: Vec::new()
		}
	}

	pub fn root(self) -> T {
		self.root
	}

	pub fn add_child(mut self, child: T) {
		self.children.push(ASTNode::Node(child));
	}

	pub fn add_node(&mut self, child: ASTNode<T>) {
		self.children.push(child);
	}

	pub fn add_subtree(mut self, child: ASTree<T>) {
		self.children.push(ASTNode::Tree(child));
	}

	pub fn get_children(self) -> Vec<ASTNode<T>> {
		self.children
	}
}