
use std::cmp;
use std::collections::HashMap;

pub type BDD = isize;

type VarId = isize;

#[derive(Hash,Eq,PartialEq)]
struct Node {
    var: VarId,
    t: BDD,
    f: BDD,
}

pub struct Context<'a> {
    vars: HashMap<&'a str,VarId>,
    computed: HashMap<&'a (BDD,BDD,BDD), BDD>,
    unique: HashMap<&'a (VarId,BDD,BDD), BDD>,
    nodes: Vec<Node>,
    next: BDD,
}

impl <'a> Context<'a> {

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

    fn var_id(&self, var: &str) -> VarId {
        *self.vars.get(var).expect("Variable is not present in BDD")
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

    fn add_unique(&mut self, v: VarId, t: BDD, f: BDD) -> BDD {
        let key : &'a (VarId,BDD,BDD) = &(v, t, f);
        match self.unique.get(key) {
            Some(n) => *n,
            None => {
                let node = Node { var: v, t: t, f: f };
                let i = self.next;
                self.next = self.next + 1;
                self.unique.insert(key, i);
                self.nodes.push(node);
                i
            }
        }
    }

    fn add_computed(&mut self, r: BDD, f: BDD, g: BDD, h: BDD) {
        let key = (f, g, h);
        self.computed.insert(&key, r);
        ()
    }

    // Return the top variable from the set of bdds.
    fn top_var(&self, f: BDD, g: BDD, h: BDD) -> VarId {
        let nf = self.nodes.get(f.abs() as usize).unwrap();
        let ng = self.nodes.get(g.abs() as usize).unwrap();
        let nh = self.nodes.get(h.abs() as usize).unwrap();
        cmp::min(nf.var, cmp::min(ng.var, nh.var))
    }

    fn fix(&self, v: VarId, val: bool, n: BDD) -> BDD {
        let node = self.nodes.get(n.abs() as usize).unwrap();
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

    pub fn True(&self) -> BDD {
        1
    }

    pub fn False(&self) -> BDD {
        0
    }

    pub fn var(&self, var: &str) -> BDD {
        *self.vars.get(var).expect("Missing variable")
    }
}
