# CodeWars--Shortest-Word-7-kyu---Passed-
Simple, given a string of words, return the length of the shortest word(s).

String will never be empty and you do not need to account for different data types.


TEST CASES:
#[cfg(test)]
mod tests {
    use rand::{thread_rng, Rng, seq::SliceRandom};
    use super::find_short;
        
    fn dotest(s: &str, expected: u32) {
        let actual = find_short(s);
        assert!(actual == expected, 
            "With s = \"{s}\"\nExpected {expected} but got {actual}")
    }

    #[test]
    fn fixed_tests() {
        dotest("bitcoin take over the world maybe who knows perhaps", 3);
        dotest("turns out random test cases are easier than writing out basic ones", 3);
        dotest("lets talk about javascript the best language", 3);
        dotest("i want to travel the world writing code one day", 1);
        dotest("Lets all go on holiday somewhere very cold", 2);
        dotest("Let's travel abroad shall we", 2);
        dotest("Steem Lisk Classic Dash Ethereum Dogecoin MadeSafeCoin Ethereum Monero Monero LiteCoin ProofOfWork", 4);
        dotest("21inc Dash LiteCoin Ripple MadeSafeCoin Bitcoin Bitcoin Bitcoin Monero Ripple Bitcoin", 4);
        dotest("Ethereum Factom Monero 21inc Lisk Dogecoin DarkCoin Monero Ethereum Bitcoin ProofOfWork LiteCoin", 4);
        dotest("Lisk BTC DarkCoin Monero", 3);
        dotest("LiteCoin", 8);
        dotest("Waves 21inc LiteCoin Mine Dogecoin Monero LiteCoin", 4);
        dotest("Waves MadeSafeCoin 21inc BTC Steem Lisk Dogecoin Factom ProofOfStake Classic Classic BTC Ethereum Waves Factom Ripple ProofOfStake Classic Steem", 3);
        dotest("Ripple ProofOfWork Mine Lisk Mine Waves Ethereum", 4);
        dotest("DarkCoin", 8);
        dotest("Dogecoin Lisk ProofOfStake Bitcoin LiteCoin Steem DarkCoin Factom Ethereum MadeSafeCoin", 4);
        dotest("Monero Classic Dash Monero Dash Bitcoin Lisk Monero Steem ProofOfWork ProofOfStake LiteCoin Bitcoin Dash", 4);
        dotest("Steem LiteCoin Bitcoin BTC Waves Bitcoin Classic Ethereum", 3);
        dotest("Ripple ProofOfWork MadeSafeCoin Monero Ripple 21inc Waves Monero Dash", 4);
        dotest("DarkCoin BTC Bitcoin Ripple ProofOfStake 21inc MadeSafeCoin Mine Lisk Classic LiteCoin 21inc Waves BTC Steem 21inc Factom Dogecoin Classic", 3);
        dotest("Mine Dash Monero", 4);
        dotest("Lisk Steem Monero Waves Lisk Factom Classic Waves Dogecoin Ripple Steem", 4);
        dotest("Lisk Classic Waves MadeSafeCoin LiteCoin DarkCoin 21inc Ethereum LiteCoin BTC", 3);
        dotest("LiteCoin MadeSafeCoin LiteCoin Dogecoin", 8);
        dotest("Monero Dogecoin ProofOfWork ProofOfStake BTC 21inc Lisk Bitcoin LiteCoin BTC BTC Monero Monero Dogecoin Steem Steem", 3);
        dotest("DarkCoin Factom Steem Steem BTC Classic LiteCoin Lisk Ethereum ProofOfWork Monero Waves Bitcoin Steem MadeSafeCoin", 3);
        dotest("Classic Lisk Bitcoin Mine LiteCoin Factom Ripple BTC 21inc Ethereum LiteCoin Lisk", 3);
        dotest("Monero ProofOfWork Dash LiteCoin Dash Dash ProofOfWork Ethereum Bitcoin DarkCoin", 4);
        dotest("Classic 21inc LiteCoin BTC Lisk DarkCoin 21inc BTC", 3);
        dotest("LiteCoin Dash Dogecoin Ethereum Ethereum Steem Waves Steem 21inc Steem Bitcoin Ethereum MadeSafeCoin Monero Waves 21inc ProofOfWork Dash Steem Lisk", 4);
        dotest("ProofOfWork Waves Mine Ripple DarkCoin Waves Ethereum 21inc Ripple ProofOfStake Waves MadeSafeCoin Lisk ProofOfWork DarkCoin Bitcoin Waves ProofOfStake", 4);
        dotest("Ripple ProofOfStake Dogecoin Mine 21inc BTC LiteCoin Dogecoin MadeSafeCoin Dogecoin Waves Waves MadeSafeCoin DarkCoin Ripple ProofOfWork Lisk ProofOfWork Bitcoin", 3);
        dotest("LiteCoin 21inc Classic Monero Ethereum Factom Dash MadeSafeCoin Ripple ProofOfWork Bitcoin", 4);
        dotest("Monero MadeSafeCoin DarkCoin MadeSafeCoin Lisk Waves BTC DarkCoin", 3);
        dotest("Bitcoin LiteCoin BTC LiteCoin Waves ProofOfWork 21inc", 3);
        dotest("21inc Monero Dash BTC Lisk Ripple Lisk Factom", 3);
        dotest("21inc", 5);
        dotest("Mine ProofOfStake Monero Lisk BTC Steem Steem Monero Classic ProofOfStake Steem", 3);
        dotest("Bitcoin LiteCoin Monero MadeSafeCoin Lisk LiteCoin MadeSafeCoin ProofOfStake Factom Steem DarkCoin", 4);
        dotest("Factom Lisk Classic Ripple Factom DarkCoin LiteCoin", 4);
        dotest("DarkCoin Waves MadeSafeCoin ProofOfWork Dash Mine ProofOfWork", 4);
        dotest("Ripple MadeSafeCoin Dogecoin Ripple Waves Ethereum ProofOfStake 21inc", 5);
        dotest("ProofOfWork Dogecoin", 8);
        dotest("Classic Waves Factom Waves MadeSafeCoin Mine Steem Monero Dogecoin Bitcoin Classic Dogecoin Bitcoin MadeSafeCoin Ripple Mine", 4);
        dotest("Ripple Mine", 4);
        dotest("BTC ProofOfWork MadeSafeCoin Bitcoin Bitcoin Mine Mine Steem Steem", 3);
    }
    
    #[test]
    fn random_tests() {
        let mut rng = thread_rng();
        let all_chars = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!#$%&'*+,-./:;<=>?@^_`~".chars().collect::<Vec<_>>();
        for _ in 0..100 {
            let mut words = Vec::with_capacity(rng.gen_range(1..20));
            let mut expected = 1000;
            for _ in 0..words.capacity() {
                let l = rng.gen_range(1..40);
                expected = expected.min(l);
                words.push((0..l).map(|_| all_chars.choose(&mut rng).unwrap()).collect::<String>());
            }
            dotest(&words.join(" "), expected);
        }
    }
}
