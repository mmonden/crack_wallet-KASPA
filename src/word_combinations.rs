use std;

struct WordCount
{
	amount_of_each : [i32; 26],
	amount_of_each_letters : [char; 4],
	at_end : [bool; 26],
	letters : [i32; 26],
	line : [usize; 12]
}

pub struct Word
{
	filepath: String,
	data: String,

	// usedWords : std::collections::HashSet<String>,

	word_count : WordCount
}

impl Word
{
	pub fn new(path : String) -> Word
	{
		return Word
		{
			filepath : path.clone(),
			data : std::fs::read_to_string(path).expect("No file given!"),

			// usedWords : std::collections::HashSet::new(),

			word_count : WordCount 
			{ 
				amount_of_each : [4, 3, 3, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
				amount_of_each_letters : ['a', 'b', 'c', 'd'],
				at_end : [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false],
				letters : [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
				line : [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
			}
		};
	}

	pub fn getWords(&self) -> std::vec::Vec<String>
	{
		return self.data.lines().map(|l : &str| l.to_string()).collect();
	}

	fn getCurrentLetter(&self, word : &str) -> char
	{
		return word.chars().next().unwrap();
	}

	fn selectWords(&mut self, length_words : usize) -> std::vec::Vec<String>	
	{
		let mut word_vector : std::vec::Vec<String> = std::vec::Vec::new();

		let mut counter : usize = 0;
		let mut pivot : usize = 0;
		let mut word_num : usize = 0;

		let words : std::vec::Vec<String> = self.getWords();
		let length : usize = words.len();

		let mut current_letter_number: usize = 0;

		for index in 0 .. 25
		{
			counter = 0;
			// word_num = 0; ??

			while counter < length
			{
				counter += 1;
				current_letter_number = self.getCurrentLetter(&words[counter - 1]) as usize;
	
				/*
					As long as there is room for another word, i.e. we have less than 12 words, you can add one.
				*/
				if word_vector.len() < length_words
				{
					/*
						If the words are not at the end, the normal procedure can be followed for taken a word.
						At end means that, i.e. max 4 words starting with 'a' the 4th word is the last word in the list before 'b' words, then we set at end true.
					*/
					if !self.word_count.at_end[current_letter_number - 97]
					{
						if self.word_count.letters[current_letter_number - 97] < self.word_count.amount_of_each[current_letter_number - 97]
						{
							word_vector.push(words[counter - 1].to_string());
	
							self.word_count.letters[current_letter_number - 97] += 1;
							self.word_count.line[word_num] = counter - 1;
	
							word_num += 1;
	
							/*
								If the last word is selected, at end is true for this letter.
							*/
							if self.getCurrentLetter(&words[counter]) as usize == current_letter_number + 1
							{
								self.word_count.at_end[current_letter_number - 97] = true;
								println!("{:?}", self.word_count.at_end);
							}
						}
					}
					// else 
					// {
					// 	break;
					// }
				}
				else 
				{
					break;	
				}
			}
			
			/*
				If there is no space left, we do the algorithm.
				1 2 3 4 . .
				1 2 3 . 4 .
				1 2 3 . . 4

				1 2 . 3 4 .
				1 2 . 3 . 4
				1 2 . . 3 4
				
				1 . 2 3 4 .
				1 . 2 3 . 4
				1 . 2 . 3 4
				1 . . 2 3 4

				. 1 2 3 4 .
				etc.
			*/

			pivot = length_words - index;

			while !self.word_count.at_end[22] && !self.word_count.at_end[23] && !self.word_count.at_end[24] && !self.word_count.at_end[25]
			{	
				pivot -= 1;

				for j in pivot .. length_words
				{
					while self.word_count.line[j] < length - 1
					{
						for i in 0 .. pivot + 1
						{
							word_vector.push(words[self.word_count.line[i]].to_string());
						}

						self.word_count.line[j] += 1;	// Should be checked if okay to do so;

						if j == 11
						{	
							word_vector.push(words[self.word_count.line[j]].to_string());
						}
						else
						{
							break;
						}
					}
				}

				break;
			}

			break;
		}

		return word_vector;
	}

	fn makeAllCombinations(&mut self, word_vec : std::vec::Vec<String>)
	{

	}

	pub fn makeList(&mut self, max_words : usize) -> std::vec::Vec<String>
	{
		let mut vec_array : std::vec::Vec<String> = std::vec::Vec::new();

		vec_array = self.selectWords(max_words);
		println!("{:?}", self.word_count.line);

		return vec_array;//.iter().map(|s| s.to_string()).collect();
	}
}