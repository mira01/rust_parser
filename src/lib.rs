#[derive(Clone, Debug, PartialEq, Eq)]
struct Element{
    name: String,
    attributes: Vec<(String, String)>,
    children: Vec<Element>,
}

//Fn(&str) -> Result<(&str, Element), Error>;

fn the_letter_a(input: &str) -> Result<(&str, ()), &str>{
    match input.chars().next(){
        Some('a') => Ok((&input['a'.len_utf8()..], ())),
        _ => Err(input),
    }
}

fn match_literal(expected: &'static str) -> impl Fn(&str) -> Result<(&str, ()), &str>{
    move |input| {
        if let Some(output) = input.strip_prefix(expected){
            Ok((output, ()))
        } else {
            Err(input)
        }
    }
}

fn identifier(input: &str) -> Result<(&str, String), &str>{
    let mut matched = String::new();
    let mut chars = input.chars();

    match chars.next() {
        Some(next) if next.is_alphabetic() => matched.push(next),
        _ => return Err(input),
    }

    while let Some(next) = chars.next(){
        if next.is_alphanumeric() || next == '-' {
            matched.push(next);
        } else {
            break;
        }
    }

    Ok((&input.strip_prefix(&matched).unwrap(), matched))
}

fn pair<P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Fn(&str) -> Result<(&str, (R1, R2)), &str>
where
P1: Fn(&str) -> Result<(&str, R1), &str>,
P2: Fn(&str) -> Result<(&str, R2), &str>,{
    move |input| match parser1(input){
        Ok((next_input, result1)) =>
            match parser2(next_input) {
                Ok((final_input, result2)) => Ok((final_input, (result1, result2))),
                Err(err) => Err(input)
        },
        Err(err) => Err(err),
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn parse_a(){
        let parse_ahoj = super::match_literal("ahoj");
        assert_eq!(parse_ahoj("ahoj"), Ok(("", ())));
        assert_eq!(parse_ahoj("ahoj bobe"), Ok((" bobe", ())));

        assert_eq!(parse_ahoj("čau"), Err("čau"));
    }

    #[test]
    fn parse_identifier(){
        assert_eq!(super::identifier("id-enti-ffier"), Ok(("", "id-enti-ffier".into())) );
        assert_eq!(super::identifier("r2-d2 and co"), Ok((" and co", "r2-d2".into())) );
        assert_eq!(super::identifier("-not at all"), Err("-not at all") );
    }

    #[test]
    fn pair_combinator(){
        let tag_opener = super::pair(super::match_literal("<"), super::identifier);
        assert_eq!(tag_opener("<my-first-element/>"),
             Ok(("/>", ((), "my-first-element".to_string())))
        );
        assert_eq!(tag_opener("oops"), Err("oops"));
        assert_eq!(tag_opener("<!oops"), Err("<!oops"));

    }
}
