use std;

// pub trait Words
// {
// 	fn new(path: String) -> Word;
// 	// fn getWords() -> String;
// }

struct WordCount
{
	letters : [i32 ; 26]
}

pub struct Word
{
	filepath: String,
	data: String,

	usedWords : std::collections::HashSet<String>,

	wordCount : WordCount
}

impl Word
{
	pub fn new(path : String) -> Word
	{
		return Word
		{
			filepath : path.clone(),
			data : std::fs::read_to_string(path).expect("No file given!"),

			usedWords : std::collections::HashSet::new(),

			wordCount : WordCount { letters : [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] }
		};
	}

	pub fn getWords(&self) -> String
	{
		return self.data.to_string();
	}

	fn getCurrentLetter(&self, word : String) -> char
	{
		return word.chars().next().unwrap();
	}

	fn selectWords(&mut self, length : usize, maxAmount : i32) -> std::vec::Vec<String>	
	{
		let mut wordVector : std::vec::Vec<String> = std::vec::Vec::new();
		
		let mut currentLetter : char = 'a';

		for word in self.getWords().lines()
		{
			let currentLetter_number = self.getCurrentLetter(word.to_string().clone()) as usize;

			if wordVector.len() < length
			{
				if self.wordCount.letters[currentLetter_number - 97] < maxAmount
				{
					if !(self.usedWords.contains(word))
					{
						wordVector.push(word.to_string());
						self.usedWords.insert(word.to_string().clone());
						self.wordCount.letters[currentLetter_number - 97] += 1;
					}
				}
			}
			else
			{
				break;
			}
		}

		return wordVector;
	}

	fn makeAllCombinations(&mut self, wordVec : std::vec::Vec<String>)
	{

	}

	fn vecToArray(&self, vec : std::vec::Vec<String>)
	{

	}

	fn arrayToVec(&self, array : [String; 12])
	{
		
	}

	pub fn makeList(&mut self, maxWords : usize, amountOfEach : i32) -> std::vec::Vec<String> //std::vec::Vec<[String; 12]>
	{
		let mut vecArray : std::vec::Vec<[String; 12]> = std::vec::Vec::new();
		let mut wordVec : std::vec::Vec<String> = std::vec::Vec::new();

		wordVec = self.selectWords(maxWords, amountOfEach);

		return wordVec; //vecArray;
	}
}