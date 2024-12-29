use octizys_common::equivalence::assert_equivalent;
use octizys_macros::Equivalence;
use octizys_pretty::highlight::{HighlightRenderer, TerminalRender24};

#[derive(Equivalence)]
struct Empty;

#[derive(Equivalence)]
struct One {
    one: u32,
}

#[derive(Equivalence)]
struct OneAllIgnored {
    #[allow(dead_code)]
    #[equivalence(ignore)]
    one: u32,
}

#[derive(Equivalence)]
struct Two {
    one: u32,
    two: u32,
}

#[derive(Equivalence)]
struct TwoAllIgnored {
    #[allow(dead_code)]
    #[equivalence(ignore)]
    one: u32,
    #[allow(dead_code)]
    #[equivalence(ignore)]
    two: u32,
}

#[derive(Equivalence)]
struct Complex {
    x1: u32,
    x2: u32,
    #[allow(dead_code)]
    #[equivalence(ignore)]
    x3: u32,
    x4: u32,
    x5: u32,
}

#[test]
fn empty() {
    let l = Empty;
    let r = Empty;
    assert_equivalent(&l, &r, TerminalRender24::render_highlight)
}

#[test]
fn one_reflexive() {
    let l = One { one: 1 };
    assert_equivalent(&l, &l, TerminalRender24::render_highlight);
}

#[test]
#[should_panic(
    expected = "Not equivalent!\n\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0mOne\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0m\n  \u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;117;0m(\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;117;0mu32\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;117;0m 1\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;117;0m\n  \u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;117;0m)\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0m\n  \u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;117;0;0m(\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;117;0;0mu32\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;117;0;0m 2\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;117;0;0m\n  \u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;117;0;0m)\u{1b}[0m\u{1b}[0m"
)]
fn one_differents() {
    let l = One { one: 1 };
    let r = One { one: 2 };
    assert_equivalent(&l, &r, TerminalRender24::render_highlight)
}

#[test]
fn one_all_ignored() {
    let l = OneAllIgnored { one: 1 };
    let r = OneAllIgnored { one: 2 };
    assert_equivalent(&l, &l, TerminalRender24::render_highlight);
    assert_equivalent(&l, &r, TerminalRender24::render_highlight)
}

#[test]
fn two_reflexive() {
    let l = Two { one: 1, two: 2 };
    assert_equivalent(&l, &l, TerminalRender24::render_highlight);
}

#[test]
#[should_panic(
    expected = "Not equivalent!\n\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0mTwo\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0m\n  \u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;117;0m(\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;117;0mu32\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;117;0m 1\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;117;0m\n  \u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;117;0m)\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0m\n  \u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;117;0;0m(\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;117;0;0mu32\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;117;0;0m 3\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;117;0;0m\n  \u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;117;0;0m)\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0m\n  \u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0m(\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0mu32\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0m 2\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0m\n  \u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0m)\u{1b}[0m\u{1b}[0m"
)]
fn two_differents() {
    let l = Two { one: 1, two: 2 };
    let r = Two { one: 3, two: 2 };
    assert_equivalent(&l, &r, TerminalRender24::render_highlight)
}

#[test]
fn two_all_ignored() {
    let l = TwoAllIgnored { one: 1, two: 2 };
    let r = TwoAllIgnored { one: 3, two: 4 };
    assert_equivalent(&l, &r, TerminalRender24::render_highlight);
    assert_equivalent(&r, &l, TerminalRender24::render_highlight)
}

#[test]
fn complex_reflexive() {
    let l = Complex {
        x1: 1,
        x2: 2,
        x3: 3,
        x4: 4,
        x5: 5,
    };
    assert_equivalent(&l, &l, TerminalRender24::render_highlight);
}

#[test]
#[should_panic(
    expected = "Not equivalent!\n\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0mComplex\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0m\n  \u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0m(\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0mu32\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0m 1\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0m\n  \u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0m)\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0m\n  \u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;117;0m(\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;117;0mu32\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;117;0m 2\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;117;0m\n  \u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;117;0m)\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0m\n  \u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;117;0;0m(\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;117;0;0mu32\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;117;0;0m 6\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;117;0;0m\n  \u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;117;0;0m)\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0m\n  \u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;117;0m(\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;117;0mu32\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;117;0m 4\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;117;0m\n  \u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;117;0m)\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0m\n  \u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;117;0;0m(\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;117;0;0mu32\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;117;0;0m 8\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;117;0;0m\n  \u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;117;0;0m)\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0m\n  \u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0m(\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0mu32\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0m 5\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0m\n  \u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0m)\u{1b}[0m\u{1b}[0m"
)]
fn complex_differents() {
    let l = Complex {
        x1: 1,
        x2: 2,
        x3: 3,
        x4: 4,
        x5: 5,
    };
    let r = Complex {
        x1: 1,
        x2: 6,
        x3: 3,
        x4: 8,
        x5: 5,
    };
    assert_equivalent(&l, &r, TerminalRender24::render_highlight)
}

#[test]
fn complex_ignored() {
    let l = Complex {
        x1: 1,
        x2: 2,
        x3: 32,
        x4: 4,
        x5: 5,
    };
    let r = Complex {
        x1: 1,
        x2: 2,
        x3: 16,
        x4: 4,
        x5: 5,
    };
    assert_equivalent(&l, &r, TerminalRender24::render_highlight);
    assert_equivalent(&r, &l, TerminalRender24::render_highlight)
}
