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

#[derive(Debug)]
pub struct BaseASTree<T> {
	 children: Vec<ASTNode<T>>
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
}

impl<T> Tree<T> for ASTree<T> {
	fn add_child(&mut self, child: T) {
		self.children.push(ASTNode::Node(child));
	}

	fn add_node(&mut self, child: ASTNode<T>) {
		self.children.push(child);
	}

	fn add_subtree(&mut self, child: ASTree<T>) {
		self.children.push(ASTNode::Tree(child));
	}

	fn get_children(self) -> Vec<ASTNode<T>> {
		self.children
	}
}

impl<T> Tree<T> for BaseASTree<T> {
	fn add_child(&mut self, child: T) {
		self.children.push(ASTNode::Node(child));
	}

	fn add_node(&mut self, child: ASTNode<T>) {
		self.children.push(child);
	}

	fn add_subtree(&mut self, child: ASTree<T>) {
		self.children.push(ASTNode::Tree(child));
	}

	fn get_children(self) -> Vec<ASTNode<T>> {
		self.children
	}
}

impl<T> BaseASTree<T> {
	pub fn new() -> BaseASTree<T> {
		BaseASTree {
			children: Vec::new()
		}
	}
}


pub trait Tree<T> {
	fn add_child(&mut self, child: T);
	fn add_node(&mut self, child: ASTNode<T>);
	fn add_subtree(&mut self, child: ASTree<T>);
	fn get_children(self) -> Vec<ASTNode<T>>;
}