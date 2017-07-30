
use super::bddrs::Context;

#[test]
fn empty_size() {
    let cxt = Context::new(vec![]);
    assert!(cxt.size() == 2)
}

#[test]
fn simple() {
    let mut cxt = Context::new(vec!["foo"]);
    let t = cxt.True();
    let f = cxt.False();
    let res = cxt.ite(t, t, f);
    assert!(res == t)
}

#[test]
fn var_test() {
    let mut cxt = Context::new(vec!["foo"]);
    let t = cxt.True();
    let f = cxt.False();
    let foo = cxt.var("foo");
    let res = cxt.ite(foo,t,f);
    assert!(res == foo)
}
