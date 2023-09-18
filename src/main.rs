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
}
