use Renderable;

use tree::{ RcNode };
pub struct SceneGraph {
    pub root: RcNode<Box<Renderable>>,
}
