mod word_combinations;

use word_combinations::Word;

fn main()
{
	let password = "M@tth1asMonden";

	let filepath = "../../wallet/bip32/src/mnemonic/words/english.txt";

	// let c : char = 'a';
	// println!("{}", c as u32 - 97);

	let mut W = Word::new(filepath.to_string());

	// let words : std::vec::Vec<String> = W.getWords();
	//println!("{}", words);

	let result_vec : std::vec::Vec<String> = W.makeList(12);

	let mut counter : usize = 0;
	let mut row : usize = 0;

	for num in result_vec
	{
		if row % 12 as usize == 0
		{
			counter = counter + 1;
			print!("\n{}: {}", counter, num);
		}
		else
		{
			print!(" {}", num);
		}

		row = row + 1;
	}
}