use anyhow::Result;

pub fn process_input(input: &[u8]) -> Result<u32> {
    let out: u32 = input
        .split(|&c| c == b',')
        .map(|seq| hash(std::str::from_utf8(seq).expect("invalid utf8")))
        .sum();
    Ok(out)
}

fn hash(input: &str) -> u32 {
    input.chars().filter(|&c| c != '\n').fold(0, |acc, c| {
        let code = c as u32;
        let mut cur_value = acc + code;
        cur_value *= 17;
        cur_value % 256
    })
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_hash() {
        assert_eq!(hash("HASH"), 52);
        assert_eq!(hash("rn=1"), 30);
        assert_eq!(hash("cm-"), 253);
    }

    #[test]
    fn test_input() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7".as_bytes();
        assert_eq!(process_input(input).unwrap(), 1320);
    }
}
