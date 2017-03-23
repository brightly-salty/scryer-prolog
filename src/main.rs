extern crate termion;
mod prolog;

use prolog::io::*;
use prolog::machine::*;

#[cfg(test)]
mod tests {
    use super::*;
    use prolog::ast::*;
    
    fn submit(wam: &mut Machine, buffer: &str) -> EvalResult {
        let result = eval(wam, buffer);
        wam.reset();
        result
    }
    
    #[test]
    fn test_queries_on_facts() {
        let mut wam = Machine::new();

        submit(&mut wam, "p(Z, Z).");
        submit(&mut wam, "clouds(are, nice).");

        // submit returns true on failure, false on success.
        assert_eq!(submit(&mut wam, "?- p(Z, Z).").failed_query(), false);
        assert_eq!(submit(&mut wam, "?- p(Z, z).").failed_query(), false);
        assert_eq!(submit(&mut wam, "?- p(Z, w).").failed_query(), false);
        assert_eq!(submit(&mut wam, "?- p(z, w).").failed_query(), true);
        assert_eq!(submit(&mut wam, "?- p(w, w).").failed_query(), false);
        assert_eq!(submit(&mut wam, "?- clouds(Z, Z).").failed_query(), true);
        assert_eq!(submit(&mut wam, "?- clouds(are, Z).").failed_query(), false);
        assert_eq!(submit(&mut wam, "?- clouds(Z, nice).").failed_query(), false);

        assert_eq!(submit(&mut wam, "?- p(Z, h(Z, W), f(W)).").failed_query(), true);

        submit(&mut wam, "p(Z, h(Z, W), f(W)).");

        assert_eq!(submit(&mut wam, "?- p(z, h(z, z), f(w)).").failed_query(), true);
        assert_eq!(submit(&mut wam, "?- p(z, h(z, w), f(w)).").failed_query(), false);
        assert_eq!(submit(&mut wam, "?- p(z, h(z, W), f(w)).").failed_query(), false);
        assert_eq!(submit(&mut wam, "?- p(Z, h(Z, w), f(Z)).").failed_query(), false);
        assert_eq!(submit(&mut wam, "?- p(z, h(Z, w), f(Z)).").failed_query(), true);

        submit(&mut wam, "p(f(X), h(Y, f(a)), Y).");

        assert_eq!(submit(&mut wam, "?- p(Z, h(Z, W), f(W)).").failed_query(), false);
    }

    #[test]
    fn test_queries_on_rules() {
        let mut wam = Machine::new();

        submit(&mut wam, "p(X, Y) :- q(X, Z), r(Z, Y).");
        submit(&mut wam, "q(q, s).");
        submit(&mut wam, "r(s, t).");

        assert_eq!(submit(&mut wam, "?- p(X, Y).").failed_query(), false);
        assert_eq!(submit(&mut wam, "?- p(q, t).").failed_query(), false);
        assert_eq!(submit(&mut wam, "?- p(t, q).").failed_query(), true);
        assert_eq!(submit(&mut wam, "?- p(q, T).").failed_query(), false);
        assert_eq!(submit(&mut wam, "?- p(Q, t).").failed_query(), false);
        assert_eq!(submit(&mut wam, "?- p(t, t).").failed_query(), true);

        submit(&mut wam, "p(X, Y) :- q(f(f(X)), R), r(S, T).");
        submit(&mut wam, "q(f(f(X)), r).");

        assert_eq!(submit(&mut wam, "?- p(X, Y).").failed_query(), false);

        submit(&mut wam, "q(f(f(x)), r).");

        assert_eq!(submit(&mut wam, "?- p(X, Y).").failed_query(), false);

        submit(&mut wam, "p(X, Y) :- q(X, Y), r(X, Y).");
        submit(&mut wam, "q(s, t).");
        submit(&mut wam, "r(X, Y) :- r(a).");
        submit(&mut wam, "r(a).");

        assert_eq!(submit(&mut wam, "?- p(X, Y).").failed_query(), false);
        assert_eq!(submit(&mut wam, "?- p(t, S).").failed_query(), true);
        assert_eq!(submit(&mut wam, "?- p(t, s).").failed_query(), true);
        assert_eq!(submit(&mut wam, "?- p(s, T).").failed_query(), false);
        assert_eq!(submit(&mut wam, "?- p(S, t).").failed_query(), false);

        submit(&mut wam, "p(f(f(a), g(b), X), g(b), h) :- q(X, Y).");
        submit(&mut wam, "q(X, Y).");

        assert_eq!(submit(&mut wam, "?- p(f(X, Y, Z), g(b), h).").failed_query(), false);
        assert_eq!(submit(&mut wam, "?- p(f(X, g(Y), Z), g(Z), X).").failed_query(), true);
        assert_eq!(submit(&mut wam, "?- p(f(X, g(Y), Z), g(Z), h).").failed_query(), false);
        assert_eq!(submit(&mut wam, "?- p(Z, Y, X).").failed_query(), false);
        assert_eq!(submit(&mut wam, "?- p(f(X, Y, Z), Y, h).").failed_query(), false);
    }

    #[test]
    fn test_queries_on_predicates() {
        let mut wam = Machine::new();

        submit(&mut wam, "p(X, a). p(b, X).");

        assert_eq!(submit(&mut wam, "?- p(x, Y).").failed_query(), false);
        assert_eq!(submit(&mut wam, "?- p(X, a).").failed_query(), false);
        assert_eq!(submit(&mut wam, "?- p(b, X).").failed_query(), false);
        assert_eq!(submit(&mut wam, "?- p(X, X).").failed_query(), false);
        assert_eq!(submit(&mut wam, "?- p(b, a).").failed_query(), false);
        assert_eq!(submit(&mut wam, "?- p(a, b).").failed_query(), true);

        submit(&mut wam, "p(X, Y, a). p(X, a, Y). p(X, Y, a).");

        assert_eq!(submit(&mut wam, "?- p(c, d, X).").failed_query(), false);
        assert_eq!(submit(&mut wam, "?- p(a, a, a).").failed_query(), false);
        assert_eq!(submit(&mut wam, "?- p(b, c, d).").failed_query(), true);
        
        submit(&mut wam, "p(X, a). p(X, Y) :- q(Z), p(X, X).");

        assert_eq!(submit(&mut wam, "?- p(X, Y).").failed_query(), false);
        assert_eq!(submit(&mut wam, "?- p(x, a).").failed_query(), false);
        assert_eq!(submit(&mut wam, "?- p(X, a).").failed_query(), false);
        assert_eq!(submit(&mut wam, "?- p(X, b).").failed_query(), true);

        submit(&mut wam, "q(z).");

        assert_eq!(submit(&mut wam, "?- p(X, b).").failed_query(), false);
        assert_eq!(submit(&mut wam, "?- p(x, a).").failed_query(), false);
        assert_eq!(submit(&mut wam, "?- p(X, Y).").failed_query(), false);

        submit(&mut wam, "p(X, a). p(X, Y) :- q(Y), p(X, X).");

        assert_eq!(submit(&mut wam, "?- p(X, Y).").failed_query(), false);
        assert_eq!(submit(&mut wam, "?- p(X, b).").failed_query(), true);

        submit(&mut wam, "p(a, z). p(X, Y) :- q(Y), p(X, Y).");

        assert_eq!(submit(&mut wam, "?- p(X, Y).").failed_query(), false);
        assert_eq!(submit(&mut wam, "?- p(X, z).").failed_query(), false);
        assert_eq!(submit(&mut wam, "?- p(a, z).").failed_query(), false);
        assert_eq!(submit(&mut wam, "?- p(a, X).").failed_query(), false);
        assert_eq!(submit(&mut wam, "?- p(b, a).").failed_query(), true);

        submit(&mut wam, "p(X, Y, Z) :- q(X), r(Y), s(Z). 
                        p(a, b, Z) :- q(Z).");

        submit(&mut wam, "q(x).");
        submit(&mut wam, "r(y).");
        submit(&mut wam, "s(z).");

        assert_eq!(submit(&mut wam, "?- p(X, Y, Z).").failed_query(), false);
        assert_eq!(submit(&mut wam, "?- p(a, b, c).").failed_query(), true);
        assert_eq!(submit(&mut wam, "?- p(a, b, C).").failed_query(), false);

        submit(&mut wam, "p(X) :- q(X). p(X) :- r(X).");
        submit(&mut wam, "q(X) :- a.");
        submit(&mut wam, "r(X) :- s(X, t). r(X) :- t(X, u).");

        submit(&mut wam, "s(x, t).");
        submit(&mut wam, "t(y, u).");
        
        assert_eq!(submit(&mut wam, "?- p(X).").failed_query(), false);
        assert_eq!(submit(&mut wam, "?- p(x).").failed_query(), false);
        assert_eq!(submit(&mut wam, "?- p(y).").failed_query(), false);
        assert_eq!(submit(&mut wam, "?- p(z).").failed_query(), true);

        submit(&mut wam, "p(f(f(X)), h(W), Y) :- g(W), h(W), f(X).
                          p(X, Y, Z) :- h(Y), g(W), z(Z).");
        submit(&mut wam, "g(f(X)) :- z(X). g(X) :- h(X).");
        submit(&mut wam, "h(w). h(x). h(z).");
        submit(&mut wam, "f(s).");
        submit(&mut wam, "z(Z).");

        assert_eq!(submit(&mut wam, "?- p(X, Y, Z).").failed_query(), false);
        assert_eq!(submit(&mut wam, "?- p(X, X, Z).").failed_query(), false);
        assert_eq!(submit(&mut wam, "?- p(f(f(Z)), Y, Z).").failed_query(), false);
        assert_eq!(submit(&mut wam, "?- p(X, X, X).").failed_query(), false);
        assert_eq!(submit(&mut wam, "?- p(X, Y, X).").failed_query(), false);
        assert_eq!(submit(&mut wam, "?- p(f(f(X)), h(f(X)), Y).").failed_query(), true);
    }

    #[test]
    fn test_queries_on_lists() {
        let mut wam = Machine::new();

        submit(&mut wam, "p([Z, W]).");

        assert_eq!(submit(&mut wam, "?- p([Z, Z]).").failed_query(), false);
        assert_eq!(submit(&mut wam, "?- p([Z, W, Y]).").failed_query(), true);
        assert_eq!(submit(&mut wam, "?- p([Z | W]).").failed_query(), false);
        assert_eq!(submit(&mut wam, "?- p([Z | [Z]]).").failed_query(), false);
        assert_eq!(submit(&mut wam, "?- p([Z | [W]]).").failed_query(), false);
        assert_eq!(submit(&mut wam, "?- p([Z | []]).").failed_query(), true);

        submit(&mut wam, "p([Z, Z]).");

        assert_eq!(submit(&mut wam, "?- p([Z, Z]).").failed_query(), false);
        assert_eq!(submit(&mut wam, "?- p([Z, W, Y]).").failed_query(), true);
        assert_eq!(submit(&mut wam, "?- p([Z | W]).").failed_query(), false);
        assert_eq!(submit(&mut wam, "?- p([Z | [Z]]).").failed_query(), false);
        assert_eq!(submit(&mut wam, "?- p([Z | [W]]).").failed_query(), false);
        assert_eq!(submit(&mut wam, "?- p([Z | []]).").failed_query(), true);

        submit(&mut wam, "p([Z]).");

        assert_eq!(submit(&mut wam, "?- p([Z, Z]).").failed_query(), true);
        assert_eq!(submit(&mut wam, "?- p([Z, W, Y]).").failed_query(), true);
        assert_eq!(submit(&mut wam, "?- p([Z | W]).").failed_query(), false);
        assert_eq!(submit(&mut wam, "?- p([Z | [Z]]).").failed_query(), true);
        assert_eq!(submit(&mut wam, "?- p([Z | [W]]).").failed_query(), true);
        assert_eq!(submit(&mut wam, "?- p([Z | []]).").failed_query(), false);

        submit(&mut wam, "member(X, [X|Xs]).
                          member(X, [Y|Xs]) :- member(X, Xs).");

        assert_eq!(submit(&mut wam, "?- member(a, [c, [X, Y]]).").failed_query(), true);
        assert_eq!(submit(&mut wam, "?- member(c, [a, [X, Y]]).").failed_query(), true);
        assert_eq!(submit(&mut wam, "?- member(a, [a, [X, Y]]).").failed_query(), false);
        assert_eq!(submit(&mut wam, "?- member(a, [X, Y, Z]).").failed_query(), false);
        assert_eq!(submit(&mut wam, "?- member([X, X], [a, [X, Y]]).").failed_query(), false);
        assert_eq!(submit(&mut wam, "?- member([X, X], [a, [b, c], [b, b], [Z, x], [d, f]]).").failed_query(), false);
        assert_eq!(submit(&mut wam, "?- member([X, X], [a, [b, c], [b, d], [foo, x], [d, f]]).").failed_query(), true);
        assert_eq!(submit(&mut wam, "?- member([X, Y], [a, [b, c], [b, b], [Z, x], [d, f]]).").failed_query(), false);
        assert_eq!(submit(&mut wam, "?- member([X, Y, Y], [a, [b, c], [b, b], [Z, x], [d, f]]).").failed_query(), true);
        assert_eq!(submit(&mut wam, "?- member([X, Y, Z], [a, [b, c], [b, b], [Z, x], [d, f]]).").failed_query(), true);
    }
}

fn prolog_repl() {
    let mut wam = Machine::new();
        
    loop {
        print!("prolog> ");

        let buffer = read();

        if buffer == "quit\n" {
            break;
        } else if buffer == "clear\n" {
            wam = Machine::new();
            continue;
        }

        let result = eval(&mut wam, buffer.trim());
        print(&mut wam, result);
        
        wam.reset();
    }
}

fn main() {
    prolog_repl();
}