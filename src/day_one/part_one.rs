use crate::prelude::*;

pub(crate) fn run(buf: Buf) -> Result<()> {
    let mut total = 0;

    for line in buf.lines() {
        let line = line?;
        let first = line
            .chars()
            .find_map(|c| c.to_digit(10))
            .ok_or_else(|| anyhow!("no first char"))?;

        let last = line
            .chars()
            .rev()
            .find_map(|c| c.to_digit(10))
            .unwrap_or(first);

        total += first * 10 + last;
    }

    println!("{total}");
    Ok(())
}
