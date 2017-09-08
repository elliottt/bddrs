
use std::cmp;
use std::collections::HashMap;
use std::rc::Rc;

pub type BDD = isize;

type VarId = isize;

#[derive(Hash,Eq,PartialEq,Clone)]
struct Node {
    var: VarId,
    t: BDD,
    f: BDD,
}

pub struct Context<'a> {
    vars: HashMap<&'a str,VarId>,
    computed: HashMap<(BDD,BDD,BDD), BDD>,
    unique: HashMap<Rc<Node>, BDD>,
    nodes: Vec<Rc<Node>>,
    next: BDD,
}

impl <'a> Context<'a> {

    fn get_node(&self, n: BDD) -> &Node {
        let ix = (n.abs() - 2) as usize;
        self.nodes.get(ix).expect("Missing node")
    }

    pub fn new(ps: Vec<&'a str>) -> Context<'a> {

        // setup the variable ordering map
        let mut vars = HashMap::new();
        let mut ix = 0 as isize;
        for var in ps.iter() {
            vars.insert(*var, ix);
            ix = ix + 1
        }
        Context {
            vars: vars,
            computed: HashMap::new(),
            unique: HashMap::new(),
            nodes: Vec::new(),
            next: 2,
        }
    }

    pub fn size(&self) -> isize {
        self.next
    }

    fn terminal_case(&self, p: BDD, t: BDD, f: BDD) -> Option<BDD> {
        if p == 1 {
            Some(t)
        } else if p == 0 {
            Some(f)
        } else if t == 1 && f == 0 {
            Some(p)
        } else {
            None
        }
    }

    fn lookup_computed(&self, p: BDD, t: BDD, f: BDD) -> Option<BDD> {
        let key = (p,t,f);
        self.computed.get(&key).map(|res| *res)
    }

    fn get_unique(&self, v: VarId, t: BDD, f: BDD) -> Option<BDD> {
        // does HashMap::get_equiv still exist?
        let node = Rc::new(Node { var: v, t: t, f: f });
        self.unique.get(&node).map(|r| r.abs())
    }

    fn add_unique(&mut self, v: VarId, t: BDD, f: BDD) -> BDD {
        match self.get_unique(v,t,f) {
            Some(n) => n,
            None => {
                let node = Rc::new(Node { var: v, t: t, f: f });
                let i = self.next;
                self.next = self.next + 1;
                self.unique.insert(node.clone(), i);
                self.nodes.push(node);
                i
            }
        }
    }

    fn add_computed(&mut self, r: BDD, f: BDD, g: BDD, h: BDD) {
        self.computed.insert((f,g,h), r);
        ()
    }

    fn get_var(&self, i: BDD) -> Option<VarId> {
        if i == 0 || i == 1 {
            None
        } else {
            Some(self.get_node(i).var)
        }
    }

    fn opt_min(&self, a: Option<isize>, b: Option<isize>) -> Option<isize> {
        match a {
            Some(i) => b.map_or(a, |n| Some(cmp::min(i,n))),
            None    => b
        }
    }

    // Return the top variable from the set of bdds.
    fn top_var(&self, f: BDD, g: BDD, h: BDD) -> VarId {
        let nf = self.get_var(f);
        let ng = self.get_var(g);
        let nh = self.get_var(h);
        let x  = self.opt_min(nf, ng);
        self.opt_min(x,nh).unwrap()
    }

    fn fix(&self, v: VarId, val: bool, n: BDD) -> BDD {
        if n == 0 || n == 1 {
            return n
        }

        let node = self.get_node(n);
        if node.var == v {
            if val {
                node.t
            } else {
                node.f
            }
        } else {
            n
        }
    }

    fn ite_true(&mut self, v: VarId, f: BDD, g: BDD, h: BDD) -> BDD {
        let f1 = self.fix(v, true, f);
        let g1 = self.fix(v, true, g);
        let h1 = self.fix(v, true, h);
        self.ite(f1,g1,h1)
    }

    fn ite_false(&mut self, v: VarId, f: BDD, g: BDD, h: BDD) -> BDD {
        let f1 = self.fix(v, false, f);
        let g1 = self.fix(v, false, g);
        let h1 = self.fix(v, false, h);
        self.ite(f1,g1,h1)
    }

    pub fn ite(&mut self, f: BDD, g: BDD, h: BDD) -> BDD {
        match self.terminal_case(f,g,h) {
            Some(res) => return res,
            None => ()
        }

        match self.lookup_computed(f,g,h) {
            Some(res) => return res,
            None => ()
        }

        // fix the value of `v` in the true/false cases
        // t_branch: ( f v -> g v)
        // f_branch: (!f!v -> h!v)
        let v = self.top_var(f,g,h);
        let t_branch = self.ite_true(v, f, g, h);
        let f_branch = self.ite_false(v, f, g, h);

        if t_branch == f_branch {
            t_branch
        } else {
            let r = self.add_unique(v, t_branch, f_branch);
            self.add_computed(r, f, g, h);
            r
        }
    }

    pub fn tru(&self) -> BDD {
        1
    }

    pub fn fls(&self) -> BDD {
        0
    }

    pub fn var(&mut self, var: &str) -> BDD {
        let vid = *self.vars.get(var).expect("Missing variable");
        self.add_unique(vid, 1, 0)
    }

    pub fn and(&mut self, f: BDD, g: BDD) -> BDD {
        let h = self.fls();
        self.ite(f, g, h)
    }

    pub fn or(&mut self, f: BDD, h: BDD) -> BDD {
        let g = self.tru();
        self.ite(f, g, h)
    }

    pub fn not(&mut self, f: BDD) -> BDD {
        self.ite(f, 0, 1)
    }
}
