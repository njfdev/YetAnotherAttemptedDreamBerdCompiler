pub fn preprocess(input: &str) -> String {
    // replace all the parentheses with spaces
    let output = input.replace("(", " ").replace(")", " ");

    // TODO: add AEMI, ABI, AQMI, and AI

    output
}