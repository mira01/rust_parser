type ParseResult<'a, Output> = Result<(&'a str, Output), &'a str>;

trait Parser<'a, Output>{
    fn parse(&self, input: &'a str) -> ParseResult<'a, Output>;
}

impl<'a, F, Output> Parser<'a, Output> for F
    where
        F: Fn(&'a str) -> ParseResult<Output>,
{
    fn parse(&self, input: &'a str) -> ParseResult<'a, Output>{
        self(input)
    }
}

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

fn match_literal<'a>(expected: &'static str) -> impl Parser<'a, ()>{
    move |input: &'a str| {
        if let Some(output) = input.strip_prefix(expected){
            Ok((output, ()))
        } else {
            Err(input)
        }
    }
}

fn identifier(input: &str) -> ParseResult<String>{
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

fn pair<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Parser<'a, (R1, R2)>
where
    P1: Parser<'a, R1>,
    P2: Parser<'a, R2>,
{
    move |input|{
        parser1.parse(input).and_then(|(next_input, result1)|{
            parser2.parse(next_input)
                .map(|(last_input, result2)|{
                   (last_input, (result1, result2))
                })
                .map_err(|_err| (input))
        })
    }
}

fn map<'a, P, F, A, B>(parser: P, map_fn: F) -> impl Parser<'a, B>
    where
        P: Parser<'a, A>,
        F: Fn(A) -> B,
{
    move |input|
        parser.parse(input)
            .map(|(next_input, result)| (next_input, map_fn(result)))
}

fn left<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Parser<'a, R1>
where
    P1: Parser<'a, R1>,
    P2: Parser<'a, R2>,
{
    map(pair(parser1, parser2), |(left, _right)| left)
}

fn right<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Parser<'a, R2>
where
    P1: Parser<'a, R1>,
    P2: Parser<'a, R2>,
{
    map(pair(parser1, parser2), |(_left, right)| right)
}

fn one_or_more<'a, P, A>(parser: P) -> impl Parser<'a, Vec<A>>
where
    P: Parser<'a, A>,
{
    move |mut input|{
        let mut result = Vec::new();

        if let Ok((next_input, first_item)) = parser.parse(input){
            input = next_input;
            result.push(first_item);
        } else {
            return Err(input);
        }

        while let Ok((next_input, next_item)) = parser.parse(input){
            input = next_input;
            result.push(next_item);
        }

        Ok((input, result))
    }
}

fn zero_or_more<'a, P, A>(parser: P) -> impl Parser<'a, Vec<A>>
where
    P: Parser<'a, A>,
{
    move |mut input|{
        let mut result = Vec::new();

        while let Ok((next_input, next_item)) = parser.parse(input){
            input = next_input;
            result.push(next_item);
        }

        Ok((input, result))
    }
}


#[cfg(test)]
mod tests {
    use super::Parser;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn parse_a(){
        let parse_ahoj = super::match_literal("ahoj");
        assert_eq!(parse_ahoj.parse("ahoj"), Ok(("", ())));
        assert_eq!(parse_ahoj.parse("ahoj bobe"), Ok((" bobe", ())));

        assert_eq!(parse_ahoj.parse("čau"), Err("čau"));
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
        assert_eq!(tag_opener.parse("<my-first-element/>"),
             Ok(("/>", ((), "my-first-element".to_string())))
        );
        assert_eq!(tag_opener.parse("oops"), Err("oops"));
        assert_eq!(tag_opener.parse("<!oops"), Err("<!oops"));

    }

    #[test]
    fn right_combinator(){
        let tag_opener = super::right(super::match_literal("<"), super::identifier);
        assert_eq!(tag_opener.parse("<my-first-element/>"), Ok(("/>", "my-first-element".to_string())));
        assert_eq!(tag_opener.parse("oops"), Err("oops"));
        assert_eq!(tag_opener.parse("<!oops"), Err("<!oops"));
    }

    #[test]
    fn one_or_more_combinator(){
        let parser = super::one_or_more(super::match_literal("ha"));
        assert_eq!(parser.parse("hahaha"), Ok(("", vec![(), (), ()])));
        assert_eq!(parser.parse("hahax"), Ok(("x", vec![(), ()])));
        assert_eq!(parser.parse("ahah"), Err("ahah"));
        assert_eq!(parser.parse(""), Err(""));
    }

    #[test]
    fn zero_or_more_combinator(){
        let parser = super::zero_or_more(super::match_literal("ha"));
        assert_eq!(parser.parse("hahaha"), Ok(("", vec![(), (), ()])));
        assert_eq!(parser.parse("ahah"), Ok(("ahah", vec![])));
        assert_eq!(parser.parse(""), Ok(("", vec![])));
    }
}
