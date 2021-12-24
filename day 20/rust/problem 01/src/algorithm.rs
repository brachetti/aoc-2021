use arrayvec::ArrayString;

extern crate arrayvec;

#[derive(Copy, Clone, Debug)]
pub(crate) struct IEAlgorithm {
    description: ArrayString<512>,
}

impl IEAlgorithm {
    pub(crate) fn new(input: String) -> Self {
        let description = ArrayString::from(&input).unwrap();
        Self { description }
    }
}
