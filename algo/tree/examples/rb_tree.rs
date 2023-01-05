//! rb-Tree

// https://github.com/TheAlgorithms/Rust/blob/master/src/data_structures/rb_tree.rs

use std::ptr::null_mut;

#[derive(Debug, Copy, Clone)]
enum Color {
    Red,
    Black,
}

#[derive(Debug)]
struct Tree {
    root: *mut Node, // addr
}

#[derive(Debug)]
struct Node {
    c: Color,
    k: usize,     // key
    v: usize,     // value
    p: *mut Node, // addr
    l: *mut Node, // addr
    r: *mut Node, // addr
}

impl Node {
    pub fn new(k: usize, v: usize) -> Self {
        Self {
            k,
            v,
            c: Color::Red,
            p: null_mut(),
            l: null_mut(),
            r: null_mut(),
        }
    }
}

impl Tree {
    fn new() -> Self {
        Self { root: null_mut() }
    }

    fn insert_slice(&mut self, slice: &[usize]) {
        for (k, v) in slice.iter().enumerate() {
            self.insert(k, *val);
        }
    }

    fn insert(&mut self, k: usize, v: usize) {
        unsafe {
            let mut p = null_mut();
            let mut node = self.root;

            while !node.is_null() {
                p = node;
                node = if k < (*node).k {
                    (*node).l
                } else if k > (*node).k {
                    (*node).r
                } else {
                    // update value by key
                    (*node).v = v;

                    return;
                }
            }

            // TIP: node is the newest value
            node = Box::into_raw(Box::new(Node::new(k, 0)));

            if !p.is_null() {
                // insert
                if (*node).k < (*p).k {
                    (*p).l = node;
                } else {
                    (*p).r = node;
                }
            } else {
                // if HAVE NOT root
                self.root = node;
            }

            (*node).p = p;

            //insert_fixup(self, node);
        }
    }

    pub fn pre_ord(&self) {
        unsafe {
            if !self.root.is_null() {
                (*((*self).root)).pre_ord();
                println!("");
            }
        }
    }

    pub fn in_ord(&self) {
        unsafe {
            if !self.root.is_null() {
                (*((*self).root)).in_ord();
                println!("");
            }
        }
    }

    pub fn post_ord(&self) {
        unsafe {
            if !self.root.is_null() {
                (*((*self).root)).post_ord();
                println!("");
            }
        }
    }

    pub fn level_ord(&self) {}

    pub fn pre_ord_vec(&self, vec: &mut Vec<usize>) {
        unsafe {
            if !self.root.is_null() {
                (*((*self).root)).pre_ord_vec(vec);
            }
        }
    }

    pub fn search() {}

    pub fn remove() {}
}

impl Node {
    pub fn pre_ord(&self) {
        unsafe {
            print!("{} ", (*self).k);

            if !(*self).l.is_null() {
                let node = (*self).l;
                (*node).pre_ord();
            }
            if !(*self).r.is_null() {
                let node = (*self).r;
                (*node).pre_ord();
            }
        }
    }

    pub fn in_ord(&self) {
        unsafe {
            if !(*self).l.is_null() {
                let node = (*self).l;
                (*node).in_ord();
            }
            print!("{} ", (*self).k);

            if !(*self).r.is_null() {
                let node = (*self).r;
                (*node).in_ord();
            }
        }
    }

    pub fn post_ord(&self) {
        unsafe {
            if !(*self).l.is_null() {
                let node = (*self).l;
                (*node).post_ord();
            }
            if !(*self).r.is_null() {
                let node = (*self).r;
                (*node).post_ord();
            }
            print!("{} ", (*self).k);
        }
    }

    pub fn pre_ord_vec(&self, vec: &mut Vec<usize>) {
        unsafe {
            vec.push((*self).v);

            if !(*self).l.is_null() {
                let node = (*self).l;
                (*node).pre_ord_vec(vec);
            }
            if !(*self).r.is_null() {
                let node = (*self).r;
                (*node).pre_ord_vec(vec);
            }
        }
    }
}

#[inline]
fn insert_fixup(tree: &mut Tree, mut node: *mut Node) {
    unsafe {
        let mut p: *mut Node = (*node).parent;
        let mut gp: *mut Node;
        let mut tmp: *mut Node;

        loop {
            if p.is_null() {
                // #0.0
                // node IS Root
                // node.color == Red
                // over
                //
                //    N           n
                //   / \   -->   / \
                //  *   *       *   *
                (*node).c = Color::Black;

                break;
            }

            if ((*p).c == Color::Black) {
                // #0.1
                // node.parent.color == Black
                // over
                // (N could also be the left child of p)
                //
                //   p
                //  / \
                // *   N
                break;
            }

            // node IS NOT Root
            // AND
            // node.parent.color == Red
            gp = (*p).p;
            tmp = (*gp).r;

            if p != tmp {
                // p = (*gp).left
                if !tmp.is_null() && ((*tmp).c == Color::Red) {
                    // #1
                    // (N,P,U).color == Red
                    // g.color == Black
                    //
                    // do: g.color -> Red
                    // (N,P).color -> Black
                    // after all: G.color == Red
                    // 如果 G 的父亲也是红, 性质又被破坏了,
                    // HACK: 可以将 GPUN 看成一个新的红色 N 节点, 如此递归调整下去; 特俗的, 如果碰巧将根节点染成了红色, 可以在算法的最后强制 root->黑.
                    // Case 1 - color flips and recurse at g
                    //      G               g
                    //     / \             / \
                    //    p   u    -->    P   U
                    //   /               /
                    //  n               n

                    // p   == gp.l
                    // tmp == gp.r
                    // gp has left AND right
                    (*p).c = Color::Black;
                    (*tmp).c = Color::Black;
                    (*gp).c = Color::Red;

                    // come back
                    node = gp;
                    p = (*node).p;

                    continue;
                }

                tmp = (*p).r;

                if node == tmp {
                    // 第 2 种情况, P 为红, N 为 P 右孩子, N 为红, U 为黑或缺少. 策略: 旋转变换, 从而进入下一种情况
                    // left rotate
                    // !None Left!
                    // Root -> Left
                    // Right -> Root
                    /* node = (*parent).right */
                    /*
                     * Case 2(Option) - left rotate at p (then Case 3)
                     *
                     *    G               G
                     *   / \             / \
                     *  p   U    -->    n   U
                     *   \             /
                     *    n           p
                     */

                    lr(tree, p);
                    p = node;
                }

                // 第 3 种情况, 可能由第二种变化而来, 但不是一定: P 为红, N 为 P 左孩子, N 为红. 策略: 旋转, 交换 P,G 颜色, 调整后, 因为 P 为黑色, 所以不怕 P 的父节点是红色的情况. over
                /*
                 * Case 3 - right rotate at g
                 *
                 *      G               P
                 *     / \             / \
                 *    p   U    -->    n   g
                 *   / \ / \         / \ / \
                 *  n  * *  *       *  * *  U
                 */

                (*p).color = Color::Black;
                (*gp).color = Color::Red;

                rotate_r(tree, gp);
            } else {
                /* parent = (*gparent).right */
                tmp = (*gparent).left;
                if !tmp.is_null() && matches!((*tmp).color, Color::Red) {
                    /*
                     * Case 1 - color flips and recurse at g
                     *    G               g
                     *   / \             / \
                     *  u   p    -->    U   P
                     *       \               \
                     *        n               n
                     */

                    (*parent).color = Color::Black;
                    (*tmp).color = Color::Black;
                    (*gparent).color = Color::Red;
                    node = gparent;
                    parent = (*node).parent;
                    continue;
                }
                tmp = (*parent).left;
                if node == tmp {
                    /*
                    * Case 2 - right rotate at p (then Case 3)
                    *
                    *       G             G
                    *      / \           / \
                    *     U   p   -->   U   n
                    *        /               \
                    *       n                 p

                    */

                    rotate_r(tree, parent);
                    parent = node;
                }
                /*
                 * Case 3 - left rotate at g
                 *
                 *       G             P
                 *      / \           / \
                 *     U   p   -->   g   n
                 *          \       /
                 *           n     U
                 */

                (*parent).color = Color::Black;
                (*gparent).color = Color::Red;
                rotate_l(tree, gparent);
            }
            break;
        }
    }
}

#[inline]
fn rotate_l(tree: &mut Tree, x: *mut Node) {
    unsafe {
        // Left rotate at x
        // (x could also be the left child of p)
        //
        //   p               p
        //  / \             / \
        // *   x    -->    *   y
        //    / \             / \
        //   *   y           x   *
        //      / \         / \
        //     c   *       *   c
        //
        //
        //   1               1
        //  / \             / \
        // *   2    -->    *   2
        //    / \             / \
        //   *   4           3   *
        //      / \         / \
        //     3   *       *   4
        //

        // tmp value
        let p = (*x).p;
        let y = (*x).r;
        let c = (*y).l;

        (*y).l = x;
        (*x).p = y;
        (*x).r = c;

        if !c.is_null() {
            (*c).p = x;
        }

        // ???
        if p.is_null() {
            tree.root = y;
        } else if (*p).l == x {
            (*p).l = y;
        } else {
            (*p).r = y;
        }

        (*y).p = p;
    }
}

#[inline]
fn rotate_r(tree: &mut Tree, x: *mut Node) {
    unsafe {
        // Right rotate at x
        // (x could also be the left child of p)
        //
        //   p             p
        //  / \           / \
        // *   x    -->  *   y
        //    / \           / \
        //   y             x   *
        //  / \           / \
        // *   c         c   *
        //
        //   1             1
        //  / \           / \
        // *   4    -->  *   2
        //    / \           / \
        //   2   *         4   *
        //  / \           / \
        // *   3         3   *
        //

        let p = (*x).p;
        let y = (*x).l;
        let c = (*y).r;

        (*y).r = x;
        (*x).p = y;
        (*x).l = c;

        if !c.is_null() {
            (*c).p = x;
        }

        if p.is_null() {
            // p IS Root
            tree.root = y;
        } else if (*p).l == x {
            (*p).l = y;
        } else {
            (*p).r = y;
        }

        (*y).p = p;
    }
}

fn main() {
    let mut tree = Tree::new();

    tree.insert_slice(&((0..256).collect::<Vec<usize>>())); // 2^256

    tree.pre_ord();
    tree.in_ord();
    tree.post_ord();

    let mut vec = vec![];
    tree.pre_ord_vec(&mut vec);
    println!("{:?}", vec);
}
