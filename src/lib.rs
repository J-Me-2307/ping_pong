fn ping_pong(input: (Vec<&str>, bool)) -> Vec<&str>{

}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use super::*;

     #[rstest]
     #[case((vec!["Ping!"], true), vec!["Ping!", "Pong!"])]
     #[case((vec!["Ping!", "Ping!"], false), vec!["Ping!", "Pong!", "Ping!"])]
     #[case((vec!["Ping!", "Ping!", "Ping!"], true), vec!["Ping!", "Pong!", "Ping!", "Pong!", "Ping!", "Pong!"])]
    fn returns_correct(
        #[case] input: (Vec<&str>, bool), 
        #[case] expected: Vec<&str>
    ){
        let actual: Vec<&str> = ping_pong(input);

        assert_eq!(expected, actual);
    }
}