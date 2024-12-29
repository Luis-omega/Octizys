use octizys_common::equivalence::assert_equivalent;
use octizys_macros::Equivalence;
use octizys_pretty::highlight::{HighlightRenderer, TerminalRender24};

/*
#[derive(Equivalence)]
enum Empty{};
//This shouldn't matter much as we can't construct an inhabitant!
*/

#[derive(Equivalence)]
enum Complex {
    One,
    Two(u32, u32, u32),
    Three {
        x1: u32,
        x2: u32,
        x3: u32,
    },
    Four(
        u32,
        #[allow(dead_code)]
        #[equivalence(ignore)]
        u32,
        u32,
    ),
    Five {
        x1: u32,
        #[allow(dead_code)]
        #[equivalence(ignore)]
        x2: u32,
        x3: u32,
    },
    Six {
        #[equivalence(ignore)]
        #[allow(dead_code)]
        x1: u32,
        #[equivalence(ignore)]
        #[allow(dead_code)]
        x2: u32,
        #[equivalence(ignore)]
        #[allow(dead_code)]
        x3: u32,
    },
    #[equivalence(ignore)]
    Seven {
        x1: u32,
        x2: u32,
        x3: u32,
    },
}

#[test]
fn one() {
    let l = Complex::One;
    assert_equivalent(&l, &l, TerminalRender24::render_highlight);
}

#[test]
fn two_reflexive() {
    let l = Complex::Two(1, 2, 3);
    assert_equivalent(&l, &l, TerminalRender24::render_highlight);
}

#[test]
#[should_panic(
    expected = "Not equivalent!\n\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0mComplex::Two\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0m\n  \u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0m(\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0mu32\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0m 1\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0m\n  \u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0m)\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0m\n  \u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;117;0m(\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;117;0mu32\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;117;0m 2\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;117;0m\n  \u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;117;0m)\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0m\n  \u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;117;0;0m(\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;117;0;0mu32\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;117;0;0m 4\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;117;0;0m\n  \u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;117;0;0m)\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0m\n  \u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0m(\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0mu32\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0m 3\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0m\n  \u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0m)\u{1b}[0m\u{1b}[0m"
)]
fn two_differents() {
    let l = Complex::Two(1, 2, 3);
    let r = Complex::Two(1, 4, 3);
    assert_equivalent(&l, &r, TerminalRender24::render_highlight);
}

#[test]
fn three_reflexive() {
    let l = Complex::Three {
        x1: 1,
        x2: 2,
        x3: 3,
    };
    assert_equivalent(&l, &l, TerminalRender24::render_highlight);
}

#[test]
#[should_panic = "Not equivalent!\n\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0mComplex::Three\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0m\n  \u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0m(\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0mu32\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0m 1\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0m\n  \u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0m)\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0m\n  \u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;117;0m(\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;117;0mu32\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;117;0m 2\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;117;0m\n  \u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;117;0m)\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0m\n  \u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;117;0;0m(\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;117;0;0mu32\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;117;0;0m 3\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;117;0;0m\n  \u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;117;0;0m)\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0m\n  \u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0m(\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0mu32\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0m 3\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0m\n  \u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0m)\u{1b}[0m\u{1b}[0m"]
fn three_differents() {
    let l = Complex::Three {
        x1: 1,
        x2: 2,
        x3: 3,
    };
    let r = Complex::Three {
        x1: 1,
        x2: 3,
        x3: 3,
    };
    assert_equivalent(&l, &r, TerminalRender24::render_highlight);
}

#[test]
fn four_reflexive() {
    let l = Complex::Four(1, 2, 3);
    assert_equivalent(&l, &l, TerminalRender24::render_highlight);
    let r = Complex::Four(1, 4, 3);
    assert_equivalent(&l, &r, TerminalRender24::render_highlight);
}

#[test]
#[should_panic = "Not equivalent!\n\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0mComplex::Four\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0m\n  \u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;117;0m(\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;117;0mu32\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;117;0m 1\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;117;0m\n  \u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;117;0m)\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0m\n  \u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;117;0;0m(\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;117;0;0mu32\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;117;0;0m 0\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;117;0;0m\n  \u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;117;0;0m)\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0m\n  \u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0m(\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0mu32\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0m 3\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0m\n  \u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0m)\u{1b}[0m\u{1b}[0m"]
fn four_different() {
    let l = Complex::Four(1, 2, 3);
    let r = Complex::Four(0, 2, 3);
    assert_equivalent(&l, &r, TerminalRender24::render_highlight);
}

#[test]
fn five_reflexive() {
    let l = Complex::Five {
        x1: 1,
        x2: 2,
        x3: 3,
    };
    assert_equivalent(&l, &l, TerminalRender24::render_highlight);
    let r = Complex::Five {
        x1: 1,
        x2: 4,
        x3: 3,
    };
    assert_equivalent(&l, &r, TerminalRender24::render_highlight);
    assert_equivalent(&r, &l, TerminalRender24::render_highlight);
}

#[test]
#[should_panic = "Not equivalent!\n\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0mComplex::Five\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0m\n  \u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0m(\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0mu32\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0m 1\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0m\n  \u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0m)\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0m\n  \u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;117;0m(\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;117;0mu32\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;117;0m 3\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;117;0m\n  \u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;117;0m)\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0m\n  \u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;117;0;0m(\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;117;0;0mu32\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;117;0;0m 4\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;117;0;0m\n  \u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;117;0;0m)\u{1b}[0m\u{1b}[0m"]
fn five_differente() {
    let l = Complex::Five {
        x1: 1,
        x2: 2,
        x3: 3,
    };
    let r = Complex::Five {
        x1: 1,
        x2: 2,
        x3: 4,
    };
    assert_equivalent(&l, &r, TerminalRender24::render_highlight);
}

#[test]
fn six_reflexive() {
    let l = Complex::Six {
        x1: 1,
        x2: 2,
        x3: 3,
    };
    assert_equivalent(&l, &l, TerminalRender24::render_highlight);
    let r = Complex::Six {
        x1: 4,
        x2: 5,
        x3: 6,
    };
    assert_equivalent(&l, &r, TerminalRender24::render_highlight);
    assert_equivalent(&r, &l, TerminalRender24::render_highlight);
}

#[test]
#[should_panic(
    expected = "Not equivalent!\n\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0mComplex::Seven\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0m\n  \u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;117;0m(\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;117;0mu32\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;117;0m 1\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;117;0m\n  \u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;117;0m)\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0m\n  \u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;117;0;0m(\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;117;0;0mu32\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;117;0;0m 4\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;117;0;0m\n  \u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;117;0;0m)\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0m\n  \u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;117;0m(\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;117;0mu32\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;117;0m 2\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;117;0m\n  \u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;117;0m)\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0m\n  \u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;117;0;0m(\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;117;0;0mu32\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;117;0;0m 5\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;117;0;0m\n  \u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;117;0;0m)\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0m\n  \u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;117;0m(\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;117;0mu32\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;117;0m 3\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;117;0m\n  \u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;117;0m)\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0m\n  \u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;117;0;0m(\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;117;0;0mu32\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;117;0;0m 6\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;117;0;0m\n  \u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;117;0;0m)\u{1b}[0m\u{1b}[0m"
)]
/// We cannot allow to ignore a Branch! What would the behaviour be?
// TODO: Emit a error if users attempt this!
fn seven_reflexive() {
    let l = Complex::Seven {
        x1: 1,
        x2: 2,
        x3: 3,
    };
    assert_equivalent(&l, &l, TerminalRender24::render_highlight);
    let r = Complex::Seven {
        x1: 4,
        x2: 5,
        x3: 6,
    };
    assert_equivalent(&l, &r, TerminalRender24::render_highlight);
}

#[test]
#[should_panic(
    expected = "Not equivalent!\n\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;117;0m(\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;117;0mComplex::Three\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;117;0m\n    \u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;117;0mu32\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;117;0m 1\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;117;0m\n    \u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;117;0mu32\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;117;0m 2\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;117;0m\n    \u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;117;0mu32\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;117;0m 3\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;117;0m\n\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;117;0m)\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;0;0;0m\n\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;117;0;0m(\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;117;0;0mComplex::Five\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;117;0;0m\n    \u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;117;0;0mu32\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;117;0;0m 1\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;117;0;0m\n    \u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;117;0;0mu32\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;117;0;0m 3\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;117;0;0m\n\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[38;2;229;229;229m\u{1b}[48;2;117;0;0m)\u{1b}[0m\u{1b}[0m"
)]
fn differents_branches() {
    let l = Complex::Three {
        x1: 1,
        x2: 2,
        x3: 3,
    };
    let r = Complex::Five {
        x1: 1,
        x2: 2,
        x3: 3,
    };
    assert_equivalent(&l, &r, TerminalRender24::render_highlight);
}
