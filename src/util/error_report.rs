
pub fn report( error_message : &str, input : &str, start : usize, end : usize ) -> String {
    let len = input.len();
    assert!( start < len, "Encountered start index longer than input" );
    assert!( end < len, "Encountered end index longer than input" );
    assert!( end >= start, "Encountered end smaller than the start" );

    let lines = input.split(|c| c == '\n' || c == '\r').collect::<Vec<&str>>();
   
    let mut i = 0;
    let mut before = None;
    let mut current = None;
    let mut after = None; 
    let mut pointer = None;
    for l in lines {
        if !matches!(current, None) {
            after = Some(l);
            break;
        }
        let dash_len = start - i;
        let arrow_len = 1 + end - start;
        i += l.len() + 1; 
        if i > end {
            current = Some(l); 
            pointer = Some(format!( "{}{}", "-".repeat(dash_len), "^".repeat(arrow_len)));
        }
        else {
            before = Some(l);
        }
    }

    let p = pointer.expect("pointer was not assiged in error reporter");

    match (before, current, after) {
        (None, Some(c), None) => format!( "{}\n{}\n", c, p ),
        (None, Some(c), Some(a)) => format!( "{}\n{}\n{}\n", c, p, a ),
        (Some(b) , Some(c), None) => format!( "{}\n{}\n{}\n", b, c, p ),
        (Some(b), Some(c), Some(a)) => format!( "{}\n{}\n{}\n{}\n", b, c, p, a ),
        _ => panic!("Enountered start and end outside of input range in error reporter: {} {}", start, end),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_display_single_character_error() {
        let input = r#"
line zero
line one
line two
line three
line four
"#;

        let output = report( "some error", input, 21, 21 );

        assert_eq!( output, r#"line one
line two
-^
line three
"# );
    }
}
