use tstype_rs::builder::build;
use tstype_rs::parser::parse;

fn main() {
    let text = "User[] | Record<string,Record<number| string|Something, Array<string[][][] | Record<string|number, User[]>>>[][]>[] | number[] | string";
    println!("AST:\n{:?}", parse(text).unwrap());
    println!("TypeScript type:\n{}", build(parse(text).unwrap()));
}
