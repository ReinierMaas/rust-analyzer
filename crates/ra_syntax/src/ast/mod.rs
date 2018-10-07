mod generated;

use std::marker::PhantomData;

use itertools::Itertools;

use crate::{
    SmolStr, SyntaxNodeRef, SyntaxKind::*,
    yellow::{RefRoot, SyntaxNodeChildren},
};
pub use self::generated::*;

pub trait AstNode<'a>: Clone + Copy + 'a {
    fn cast(syntax: SyntaxNodeRef<'a>) -> Option<Self>
        where Self: Sized;
    fn syntax(self) -> SyntaxNodeRef<'a>;
}

pub trait NameOwner<'a>: AstNode<'a> {
    fn name(self) -> Option<Name<'a>> {
        child_opt(self)
    }
}

pub trait LoopBodyOwner<'a>: AstNode<'a> {
    fn loop_body(self) -> Option<Block<'a>> {
        child_opt(self)
    }
}

pub trait ArgListOwner<'a>: AstNode<'a> {
    fn arg_list(self) -> Option<ArgList<'a>> {
        child_opt(self)
    }
}

pub trait FnDefOwner<'a>: AstNode<'a> {
    fn functions(self) -> AstChildren<'a, FnDef<'a>> {
        children(self)
    }
}

pub trait ModuleItemOwner<'a>: AstNode<'a> {
    fn items(self) -> AstChildren<'a, ModuleItem<'a>> {
        children(self)
    }
}

pub trait TypeParamsOwner<'a>: AstNode<'a> {
    fn type_param_list(self) -> Option<TypeParamList<'a>> {
        child_opt(self)
    }

    fn where_clause(self) -> Option<WhereClause<'a>> {
        child_opt(self)
    }
}

pub trait AttrsOwner<'a>: AstNode<'a> {
    fn attrs(self) -> AstChildren<'a, Attr<'a>> {
        children(self)
    }
}

impl<'a> FnDef<'a> {
    pub fn has_atom_attr(&self, atom: &str) -> bool {
        self.attrs()
            .filter_map(|x| x.as_atom())
            .any(|x| x == atom)
    }
}

impl<'a> Attr<'a> {
    pub fn as_atom(&self) -> Option<SmolStr> {
        let tt = self.value()?;
        let (_bra, attr, _ket) = tt.syntax().children().collect_tuple()?;
        if attr.kind() == IDENT {
            Some(attr.leaf_text().unwrap().clone())
        } else {
            None
        }
    }

    pub fn as_call(&self) -> Option<(SmolStr, TokenTree<'a>)> {
        let tt = self.value()?;
        let (_bra, attr, args, _ket) = tt.syntax().children().collect_tuple()?;
        let args = TokenTree::cast(args)?;
        if attr.kind() == IDENT {
            Some((attr.leaf_text().unwrap().clone(), args))
        } else {
            None
        }
    }
}

impl<'a> Lifetime<'a> {
    pub fn text(&self) -> SmolStr {
        self.syntax().leaf_text().unwrap().clone()
    }
}

impl<'a> Name<'a> {
    pub fn text(&self) -> SmolStr {
        let ident = self.syntax().first_child()
            .unwrap();
        ident.leaf_text().unwrap().clone()
    }
}

impl<'a> NameRef<'a> {
    pub fn text(&self) -> SmolStr {
        let ident = self.syntax().first_child()
            .unwrap();
        ident.leaf_text().unwrap().clone()
    }
}

impl<'a> ImplItem<'a> {
    pub fn target_type(self) -> Option<TypeRef<'a>> {
        match self.target() {
            (Some(t), None) | (_, Some(t)) => Some(t),
            _ => None,
        }
    }

    pub fn target_trait(self) -> Option<TypeRef<'a>> {
        match self.target() {
            (Some(t), Some(_)) => Some(t),
            _ => None,
        }
    }

    fn target(self) -> (Option<TypeRef<'a>>, Option<TypeRef<'a>>) {
        let mut types = children(self);
        let first = types.next();
        let second = types.next();
        (first, second)
    }
}

impl<'a> Module<'a> {
    pub fn has_semi(self) -> bool {
        match self.syntax().last_child() {
            None => false,
            Some(node) => node.kind() == SEMI,
        }
    }
}

impl<'a> LetStmt<'a> {
    pub fn has_semi(self) -> bool {
        match self.syntax().last_child() {
            None => false,
            Some(node) => node.kind() == SEMI,
        }
    }
}

impl<'a> IfExpr<'a> {
    pub fn then_branch(self) -> Option<Block<'a>> {
        self.blocks().nth(0)
    }
    pub fn else_branch(self) -> Option<Block<'a>> {
        self.blocks().nth(1)
    }
    fn blocks(self) -> AstChildren<'a, Block<'a>> {
        children(self)
    }
}

fn child_opt<'a, P: AstNode<'a>, C: AstNode<'a>>(parent: P) -> Option<C> {
    children(parent).next()
}

fn children<'a, P: AstNode<'a>, C: AstNode<'a>>(parent: P) -> AstChildren<'a, C> {
    AstChildren::new(parent.syntax())
}


#[derive(Debug)]
pub struct AstChildren<'a, N> {
    inner: SyntaxNodeChildren<RefRoot<'a>>,
    ph: PhantomData<N>,
}

impl<'a, N> AstChildren<'a, N> {
    fn new(parent: SyntaxNodeRef<'a>) -> Self {
        AstChildren {
            inner: parent.children(),
            ph: PhantomData,
        }
    }
}

impl<'a, N: AstNode<'a>> Iterator for AstChildren<'a, N> {
    type Item = N;
    fn next(&mut self) -> Option<N> {
        loop {
            match N::cast(self.inner.next()?) {
                Some(n) => return Some(n),
                None => (),
            }
        }
    }
}
