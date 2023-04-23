mod word_combinations;

use word_combinations::Word;

fn main()
{
	let password = "M@tth1asMonden";

	let filepath = "/home/matthiasmonden/projects/rusty-kaspa/wallet/bip32/src/mnemonic/words/english.txt";

	// let c : char = 'a';
	// println!("{}", c as u32 - 97);

	let mut W = Word::new(filepath.to_string());

	// let words : std::vec::Vec<String> = W.getWords();
	//println!("{}", words);

	let wordV : std::vec::Vec<String> = W.makeList(12);
	println!("{:?}", wordV);
}