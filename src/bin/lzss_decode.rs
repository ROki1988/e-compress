use anyhow::anyhow;
use anyhow::Result;

fn main() -> Result<()> {
    let input_text = std::env::args()
        .nth(1)
        .ok_or_else(|| anyhow!("need input text"))?;
    let plain = decode(input_text.as_str())?;
    println!("{}", plain);

    Ok(())
}

fn decode(text: &str) -> Result<String> {
    let mut out = String::new();
    let mut token_length = String::new();
    let mut token_offset = String::new();
    let mut inside_token = false;
    let mut scanning_offset = false;
    for c in text.chars() {
        match c {
            '<' => {
                inside_token = true;
                scanning_offset = true;
            }
            ',' => {
                scanning_offset = false;
            }
            '>' => {
                inside_token = false;

                let len = out.len();
                let start: usize = len - token_offset.parse::<usize>()?;
                let end: usize = token_length.parse::<usize>()? + start;

                token_offset.clear();
                token_length.clear();

                let referenced_text = out.as_str()[start..end].to_string();

                out.push_str(referenced_text.as_str());
            }
            _ if c.is_numeric() && inside_token => {
                if scanning_offset {
                    token_offset.push(c);
                } else {
                    token_length.push(c);
                }
            }
            _ if !c.is_numeric() && inside_token => return Err(anyhow!("invalid format")),
            _ => {
                out.push(c);
            }
        }
    }

    Ok(out)
}
