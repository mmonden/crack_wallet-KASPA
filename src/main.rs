mod word_combinations;

use word_combinations::Word;

fn main()
{
	let filename = "";
	let password = "M@tth1asMonden";

	let filepath = "/home/matthiasmonden/projects/rusty-kaspa/wallet/bip32/src/mnemonic/words/english.txt";

	println!("{:p}", &filepath);

	let mut c : char = 'a';
	println!("{}", c as u32 - 97);

	let mut W = Word::new(filepath.to_string());

	let words = W.getWords();
	//println!("{}", words);

	let mut wordV = W.makeList(12, 3);
	println!("{:?}", wordV);
}