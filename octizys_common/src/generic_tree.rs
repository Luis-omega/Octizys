use core::fmt;
use std::fmt::{Debug, Display};

use octizys_pretty::{
    combinators::external_text,
    document::Document,
    highlight::{EmptyRender, HighlightRenderer},
    store::{NonLineBreakStr, Store},
};

/// A tree structure that can represent any node.
/// Useful to test different parts of the compiler.

/// A Tree with and arbitrary number of items
/// Since we are going to use this for testing, the representation
/// can be suboptimal.
/// For optimization the `recurse`crate may be useful.
///
/// The Display implementation transfoms the tree into a String
/// containing a indented Sexpression.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Tree<T> {
    Node { value: T, children: Vec<Tree<T>> },
}

pub enum TreeCompareError<'items, T> {
    DifferentsBranches {
        left: &'items Tree<T>,
        right: &'items Tree<T>,
    },
    MissingLeftBranch(&'items Tree<T>),
    MissingRightBranch(&'items Tree<T>),
}

impl<'items, T> TreeCompareError<'items, T> {
    pub fn as_string_with(&self, f: fn(&T) -> Document) -> String {
        match self {
            TreeCompareError::DifferentsBranches { left, right } => {
                format!(
                    "The three are different!:\n left:\n{0}\nright:\n{1}",
                    left.to_string_with(f),
                    right.to_string_with(f)
                )
            }
            TreeCompareError::MissingLeftBranch(t) => format!(
                "The three are different!:\n left:empty_tree \nright:{0}",
                t.to_string_with(f)
            ),
            TreeCompareError::MissingRightBranch(t) => format!(
                "The three are different!:\n left:{0}\nright:empty_tree",
                t.to_string_with(f)
            ),
        }
    }
}

impl<'items, T> Display for TreeCompareError<'items, T>
where
    T: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{0}",
            self.as_string_with(|x| external_text(&x.to_string()))
        )
    }
}

pub fn node<T>(value: T, children: Vec<Tree<T>>) -> Tree<T> {
    Tree::Node { value, children }
}

pub fn leaf<T>(value: T) -> Tree<T> {
    Tree::Node {
        value,
        children: vec![],
    }
}

/// zip together two arrays of tree completely or fail!
fn zip_tree_arrays<'items, T>(
    left: &'items Vec<Tree<T>>,
    right: &'items Vec<Tree<T>>,
) -> Result<Vec<(&'items Tree<T>, &'items Tree<T>)>, TreeCompareError<'items, T>>
{
    if left.len() < right.len() {
        return Err(TreeCompareError::MissingLeftBranch(&right[left.len()]));
    }
    if right.len() < left.len() {
        return Err(TreeCompareError::MissingRightBranch(&left[right.len()]));
    }
    return Ok(std::iter::zip(left, right).collect());
}

impl<T> Tree<T> {
    /// Compare every branch iteratively until we find one with a difference
    /// then we return the two different branches.
    // TODO: maybe allow find all the branches for more errors after the first?
    // TODO: maybe return a path to the branches to help with diff?
    pub fn compare<'items>(
        &'items self,
        rhs: &'items Self,
    ) -> Result<(), TreeCompareError<'items, T>>
    where
        T: Eq,
    {
        let mut stack = vec![(self, rhs)];
        while stack.len() > 0 {
            let (left, right) = stack.pop().unwrap();
            if left.value() != right.value() {
                return Err(TreeCompareError::DifferentsBranches {
                    left,
                    right,
                });
            }
            match zip_tree_arrays(left.children(), right.children()) {
                Ok(mut ziped) => {
                    ziped.reverse();
                    stack.extend(ziped)
                }
                Err(e) => return Err(e),
            }
        }
        return Ok(());
    }

    /// Returns a reference to the value of a node.
    #[inline]
    pub fn value(&self) -> &T {
        match self {
            Tree::Node { value, .. } => value,
        }
    }

    /// Returns a reference to the children of a node.
    #[inline]
    pub fn children(&self) -> &Vec<Tree<T>> {
        match self {
            Tree::Node { children, .. } => children,
        }
    }

    pub fn to_document_with(
        &self,
        to_document: fn(&T) -> Document,
    ) -> Document {
        use octizys_pretty::combinators::*;

        let children = concat_iter(
            self.children()
                .iter()
                .map(|x| hard_break() + x.to_document_with(to_document)),
        );

        concat(vec![
            Document::static_str(NonLineBreakStr::new("(")),
            to_document(self.value()),
            nest(2, children),
            if self.children().len() > 0 {
                hard_break()
            } else {
                empty()
            },
            Document::static_str(NonLineBreakStr::new(")")),
        ])
    }

    pub fn to_string_with(&self, to_document: fn(&T) -> Document) -> String {
        let store = Store::default();
        let doc = self.to_document_with(to_document);
        doc.render_to_string(2, EmptyRender::render_highlight, &store)
    }
}

impl<T> Display for Tree<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let s = self.to_string_with(|x| external_text(&x.to_string()));
        write!(f, "{s}")
    }
}

pub fn assert_tree_eq_with<T>(
    left: Tree<T>,
    right: Tree<T>,
    f: fn(&T) -> Document,
) -> ()
where
    T: Eq,
{
    match left.compare(&right) {
        Ok(_) => (),
        Err(e) => panic!("{0}", e.as_string_with(f)),
    }
}

pub fn assert_tree_eq<T>(left: Tree<T>, right: Tree<T>) -> ()
where
    T: Eq + Display,
{
    assert_tree_eq_with(left, right, |x| external_text(&x.to_string()))
}

#[cfg(test)]
mod generic_tree_test {
    use octizys_pretty::combinators::external_text;

    use super::leaf;

    #[test]
    #[should_panic(
        expected = "The three are different!:\n left:\n(5\n  (6)\n  (7)\n)\nright:\n(3\n  (6)\n  (7)\n)"
    )]
    fn find_failure() {
        let tree1 = super::node(
            "1",
            vec![
                super::node("2", vec![leaf("3"), leaf("4")]),
                super::node("5", vec![leaf("6"), leaf("7")]),
            ],
        );
        let tree2 = super::node(
            "1",
            vec![
                super::node("2", vec![leaf("3"), leaf("4")]),
                super::node("3", vec![leaf("6"), leaf("7")]),
            ],
        );
        super::assert_tree_eq(tree1, tree2)
    }

    #[test]
    fn three_elements() {
        let tree = super::node("1", vec![leaf("2"), leaf("3")]);
        let result = tree.to_string_with(|x| external_text(x));
        let expected = "(1\n  (2)\n  (3)\n)";
        assert_eq!(result, expected);
    }

    #[test]
    fn seven_elements() {
        let tree = super::node(
            "1",
            vec![
                super::node("2", vec![leaf("3"), leaf("4")]),
                super::node("5", vec![leaf("6"), leaf("7")]),
            ],
        );
        let result = tree.to_string_with(|x| external_text(x));

        let expected =
            "(1\n  (2\n    (3)\n    (4)\n  )\n  (5\n    (6)\n    (7)\n  )\n)";

        assert_eq!(result, expected);
    }
}
