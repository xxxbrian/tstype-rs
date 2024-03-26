use tstype_rs::parser::parse;

fn main() {
    let text = "User[] | Map<string,Map<number| string|Something, Array<string[][][] | Map<string|number, User[]>>>[][]>[] | number[] | string";
    println!("AST:\n{:?}", parse(text).unwrap());
}
