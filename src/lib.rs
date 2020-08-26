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
        match input.get(0..expected.len()){
            Some(next) if next == expected =>{
                Ok((&input[expected.len()..], ()))
            }
            _ => Err(input),
        }
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
}
