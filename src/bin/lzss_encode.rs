use anyhow::anyhow;
use anyhow::Result;
use std::io::stdout;
use std::io::BufWriter;
use std::io::Write;

fn main() -> Result<()> {
    let input_text = std::env::args()
        .nth(1)
        .ok_or_else(|| anyhow!("need input text"))?;
    let out = stdout();
    let mut out = BufWriter::new(out.lock());

    encode(input_text.as_str(), &mut out)
}

fn encode(text: &str, out: &mut impl Write) -> Result<()> {
    let mut search_buff: Vec<char> = Vec::new();
    let mut check_characters: Vec<char> = Vec::new();
    for (i, c) in text.char_indices() {
        check_characters.push(c);
        if elements_in_array(check_characters.as_slice(), search_buff.as_slice()).is_none()
            || i == (text.len() - 1)
        {
            let len = check_characters.len();
            if len > 1 {
                let maybe_index = check_characters
                    .as_slice()
                    .split_last()
                    .and_then(|(_, cc)| elements_in_array(cc, search_buff.as_slice()));
                if let Some(index) = maybe_index {
                    let offset = i + 1 - index - len;
                    let token = format!("<{},{}>", offset, len);
                    if token.len() > len {
                        write!(out, "{}", check_characters.iter().collect::<String>())?;
                    } else {
                        write!(out, "{}", token)?;
                    }
                }
            } else {
                write!(out, "{}", c)?;
            }
            check_characters.clear();
        }
        search_buff.push(c);
    }

    writeln!(out)?;

    Ok(())
}

fn elements_in_array(check_elements: &[char], elements: &[char]) -> Option<usize> {
    elements
        .windows(check_elements.len())
        .enumerate()
        .find_map(|(pos, x)| {
            (check_elements == x).then(|| pos)
        })
}
