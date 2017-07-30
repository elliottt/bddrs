
use super::bddrs::Context;

#[test]
fn empty_size() {
    let ctx = Context::new(vec![]);
    assert!(ctx.size() == 2)
}

#[test]
fn simple() {
    let mut ctx = Context::new(vec!["foo"]);
    let t = ctx.tru();
    let f = ctx.fls();
    let res = ctx.ite(t, t, f);
    assert!(res == t)
}

#[test]
fn var_test() {
    let mut ctx = Context::new(vec!["foo"]);
    let t = ctx.tru();
    let f = ctx.fls();
    let foo = ctx.var("foo");
    let res = ctx.ite(foo,t,f);
    assert!(res == foo)
}

#[test]
fn and_test() {
    let mut ctx = Context::new(vec!["foo", "bar"]);
    let foo  = ctx.var("foo");
    let bar  = ctx.var("bar");
    let nfoo = ctx.not(foo);
    let or   = ctx.or(foo, nfoo);
    let res  = ctx.and(or, bar);
    assert!(res == bar)
}
