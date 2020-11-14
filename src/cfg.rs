use clang::{Entity, EntityKind};
use petgraph::prelude::*;
use std::convert::TryFrom;
use std::fmt::Debug;

#[derive(Debug)]
pub enum Node {
    Start,
    Return,
    ImplicitReturn,
    Statement(String),

    /// Node that is connected to two other nodes, depending on a condition
    ConditionalGoto,
}

impl<'tu> From<Entity<'tu>> for Node {
    fn from(entity: Entity<'tu>) -> Self {
        Self::Statement(format!("{:?}", entity.get_kind()))
    }
}

#[derive(Default)]
/// Inner value only matters if this edge comes out of a condition node
/// In the above case, it is `true` if the edge points to the block where the flow should go when
/// the condition is true, `false` else
struct Edge(bool);

impl std::fmt::Display for Edge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(if self.0 { "if true" } else { "" })
    }
}

impl<'tu> std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

/// Control-Flow Graph of a function's body
pub struct CFG {
    graph: DiGraph<Node, Edge>,
}

impl<'tu> TryFrom<clang::Entity<'tu>> for CFG {
    type Error = ();

    fn try_from(function: Entity<'tu>) -> Result<Self, Self::Error> {
        if function.get_kind() != EntityKind::FunctionDecl {
            return Err(());
        }

        let args = function.get_arguments().unwrap(); // TODO don't unwrap
        let root_stmt = function
            .get_children()
            .iter()
            .filter(|c| c.get_kind() == EntityKind::CompoundStmt)
            .copied()
            .next()
            .unwrap()
            .get_children()
            .into_iter();

        let mut graph: DiGraph<_, _> = Default::default();

        /// Builds a (sub-)graph from a list of supposed-to-be statements. Returns the entry node of
        /// the (sub-)graph.
        ///
        /// Normal statements are simply chained together. Control flow operators may create other
        /// sub-graphs and will introduce [Node::ConditionalGoto](enum.Node.html#variant.ConditionalGoto)s.
        ///
        ///   * `next` - The node where the (sub-)graph should connect its final node(s)
        ///   * `break_to` - The node where a `break` statement should bring the control flow to. By
        ///     default, the function's implicit return. Gets reassigned when entering loops.
        fn build(
            graph: &mut DiGraph<Node, Edge>,
            next: NodeIndex,
            break_to: NodeIndex,
            mut statements: Vec<Entity>,
        ) -> NodeIndex {
            println!(
                "{}",
                statements
                    .iter()
                    .map(|c| format!("{:?}", c.get_kind()))
                    .collect::<Vec<_>>()
                    .join(", ")
            );

            statements
                .into_iter()
                .rfold(next, |next, entity| match entity.get_kind() {
                    EntityKind::ReturnStmt => graph.add_node(Node::Return),
                    EntityKind::BreakStmt => break_to,
                    EntityKind::IfStmt => {
                        let node = graph.add_node(Node::ConditionalGoto);

                        // FIXME handle else stmt (and else if, in case they're different)
                        let block = entity.get_child(1).unwrap(); // TODO don't unwrap
                        let block_start = build(graph, next, break_to, block.get_children());

                        graph.add_edge(node, block_start, Edge(true));
                        graph.add_edge(node, next, Edge(false));

                        node
                    }
                    EntityKind::ForStmt => {
                        // FIXME might by a compound statement ?
                        let stmt_init = graph.add_node(Node::from(entity.get_child(0).unwrap()));
                        // FIXME might by a compound statement ?
                        let stmt_cond = entity.get_child(1).unwrap();
                        // FIXME might by a compound statement ?
                        let stmt_after = graph.add_node(Node::from(entity.get_child(2).unwrap()));
                        let stmt_block = entity.get_child(3).unwrap();

                        let condition = graph.add_node(Node::ConditionalGoto);
                        let block_start = build(graph, stmt_after, next, stmt_block.get_children());

                        graph.add_edge(stmt_after, condition, Edge::default());
                        graph.add_edge(stmt_init, condition, Edge::default());
                        graph.add_edge(condition, block_start, Edge(true));
                        graph.add_edge(condition, next, Edge(false));

                        stmt_init
                    }
                    _ => {
                        let node = graph.add_node(Node::from(entity));
                        graph.add_edge(node, next, Edge::default());

                        node
                    }
                })
        }

        let prev = graph.add_node(Node::Start);
        let next = graph.add_node(Node::ImplicitReturn);

        let entry = build(&mut graph, next, next, root_stmt.collect());

        graph.add_edge(prev, entry, Edge::default());

        Ok(CFG { graph })
    }
}

#[cfg(feature = "dot")]
impl CFG {
    pub fn write_dot(&self, mut write: impl std::io::Write) -> std::io::Result<()> {
        let dot = petgraph::dot::Dot::new(&self.graph);
        write!(&mut write, "{}", dot)
    }
}
