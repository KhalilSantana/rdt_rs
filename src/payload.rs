#[derive(Clone, Copy, Debug)]
pub struct Payload {
    content: [u8; 5],
    padding: u8,
}

impl Payload {
    pub fn new(content: [u8; 5]) -> Self {
        Payload {
            content,
            padding: 0,
        }
    }
}
impl std::fmt::Display for Payload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output: String = "".to_owned();
        for i in 0..self.padding as usize {
            output.push_str(&self.content[i].to_string());
        }
        write!(f, "{}", output)
    }
}

pub fn split_data(data: &[u8]) -> Vec<Payload> {
    // TODO: Don't hardcode 5 here
    let mut chunks = data.chunks_exact(5);
    let mut output: Vec<Payload> = vec![];
    for e in &mut chunks {
        output.push(Payload::new([e[0], e[1], e[2], e[3], e[4]]))
    }
    // Padding for the last payload
    let remainder = chunks.remainder();
    let mut last_payload = [0; 5];
    last_payload[..remainder.len()].clone_from_slice(remainder);
    output.push(Payload {
        content: last_payload,
        padding: (5 - remainder.len()) as u8,
    });
    output
}
