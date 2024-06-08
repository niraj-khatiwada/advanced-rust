fn main() {}

#[cfg(test)]
mod test {

    #[derive(PartialEq, Eq, Debug)]
    enum Access {
        Allowed,
        Denied,
    }

    #[test]
    fn test() {
        assert_eq!(Access::Allowed, Access::Denied)
    }
}
