use protocol::{Protocol, Set};
use term::Term;

pub mod protocol;
pub mod term;

fn main() {
  let term = term::Term::Tuple(
    term::Term::Tuple(
      term::Term::Bool(false).into(),
      term::Term::Bool(true).into(),
    )
    .into(),
    term::Term::String("Oi".to_string()).into(),
  );

  let encoded = term::encode(&term);

  let decoded = term::decode(&encoded);

  println!("\n{decoded:?}\n");

  let prot = Protocol::Set(Set {
    name: "asdf".to_string(),
    payload: Term::Tuple(Term::Bool(false).into(), Term::Bool(true).into()),
  });

  let encoded = protocol::encode(prot);

  let decoded = protocol::decode(&encoded);

  println!("{decoded:?}");
}
