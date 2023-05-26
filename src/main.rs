use sunscreen::{
    fhe_program,
    types::{bfv::Signed, Cipher},
    Compiler, Error, Runtime,
};

#[fhe_program(scheme = "bfv")]
fn simple_multiply(a: Cipher<Signed>, b: Cipher<Signed>) -> Cipher<Signed> {
    a * b
}

fn main() -> Result<(), Error> {
    let app = Compiler::new().fhe_program(simple_multiply).compile()?;

    let runtime = Runtime::new(app.params())?;

    let (public_key, private_key) = runtime.generate_keys()?;

    let a = runtime.encrypt(Signed::from(15), &public_key)?;
    let b = runtime.encrypt(Signed::from(5), &public_key)?;

    let results = runtime.run(
        app.get_program(simple_multiply).unwrap(),
        vec![a, b],
        &public_key,
    )?;

    let c: Signed = runtime.decrypt(&results[0], &private_key)?;
    assert_eq!(c, 75.into());

    Ok(())
}
