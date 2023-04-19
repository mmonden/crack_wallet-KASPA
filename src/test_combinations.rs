fn main()
{
	let init_array : std::vec::Vec<&str> = vec!["abandon", "ability", "able", "baby", "bachelor", "bacon", "cabbage", "cabin", "cable", "dad"];//, "damage"];//, "damp"];
	let ELEMENTS : i64 = init_array.len() as i64;

	let mut array_4 : std::vec::Vec<&str> = std::vec::Vec::new();
	let mut array_rest : std::vec::Vec<&str> = std::vec::Vec::new();

	array_4.extend_from_slice(&init_array[0..4]);
	array_rest.extend_from_slice(&init_array[4..init_array.len()]);

	let mut result_vec : std::vec::Vec<&str> = mixing(array_4);
	let mut final_vec : std::vec::Vec<&str> = std::vec::Vec::new();
	
	let last_word : &str = init_array[init_array.len() - 1];

	let mut index : usize = 0;
	let mut glob_index : usize = 0;
	let mut position : usize = 0;
	let mut length : usize = 4;

	let mut lock : bool = false;

	for word in array_rest
	{
		position = 0;

		while position <= length
		{
			index = 0;

			while index < result_vec.len()
			{
				if glob_index % (length + 1) == position
				{
					final_vec.push(word);
				}
				else
				{
					final_vec.push(result_vec[index]);
					index = index + 1;
				}

				glob_index = glob_index + 1;

			}

			position = position + 1;
			glob_index = 0;
		}

		final_vec.push(word);
		result_vec = final_vec.clone();
		final_vec = std::vec::Vec::new();

		length = length + 1;
	}

// //	for printing	
// 	let mut counter : usize = 0;
// 	let mut row : usize = 0;

// 	for num in result_vec
// 	{
// 		if row % ELEMENTS as usize == 0
// 		{
// 			counter = counter + 1;
// 			print!("\n{}: {}", counter, num);
// 		}
// 		else
// 		{
// 			print!(" {}", num);
// 		}

// 		row = row + 1;
// 	}
}

fn mixing(init_array : std::vec::Vec<&str>) -> std::vec::Vec<&str>
{
	let max_iterations : i32 = 4*3*2*1;
	let mut iteration : i32 = 1;
	let mut pivot : [usize; 2] = [1, 2];
	let mut counter : usize = 0;

	let ELEMENTS : usize = init_array.len();

	let mut temp_array : std::vec::Vec<&str> = std::vec::Vec::new();

	let mut combinations : std::collections::HashSet<std::vec::Vec<&str>> = std::collections::HashSet::new();
	combinations.insert(init_array.clone());

	let mut result_vec : std::vec::Vec<&str> = std::vec::Vec::new();
	result_vec.extend_from_slice(&init_array);

	while iteration < max_iterations
	{
		if iteration < ELEMENTS as i32
		{
			temp_array = result_vec[result_vec.len() - ELEMENTS .. result_vec.len()].to_vec();
		}
		else
		{
			temp_array = std::vec::Vec::new();

			if iteration % ELEMENTS as i32 == 0
			{
				temp_array = shuffle(result_vec[result_vec.len() - ELEMENTS .. result_vec.len()].to_vec(), pivot, ELEMENTS);

				counter = 0;

				while combinations.contains(&temp_array)
				{
					pivot = changePivot(pivot, ELEMENTS);

					temp_array = shuffle(result_vec[result_vec.len() - ELEMENTS .. result_vec.len()].to_vec(), pivot, ELEMENTS);

					counter = counter + 1;
				}

				result_vec.extend_from_slice(&temp_array);
				
				combinations.insert(temp_array.clone());
				
				iteration = iteration + 1;
			}
			else
			{
				temp_array = result_vec[result_vec.len() - ELEMENTS .. result_vec.len()].to_vec();
			}
		}
		
		result_vec.push(temp_array[ELEMENTS-1]);
		result_vec.extend_from_slice(&temp_array[0 .. ELEMENTS-1]);
		
		combinations.insert(result_vec[result_vec.len() - ELEMENTS .. result_vec.len()].to_vec());

		iteration = iteration + 1;
	}

	return result_vec;
}

fn shuffle(vector : std::vec::Vec<&str>, pivot : [usize; 2], ELEMENTS : usize) -> std::vec::Vec<&str>
{
	let mut result : std::vec::Vec<&str> = std::vec::Vec::new();

	if pivot[0] == 1
	{
		result.push(vector[1]); 
		result.push(vector[0]);
		result.extend_from_slice(&vector[2 .. ELEMENTS]);
	}
	else if pivot[0] == ELEMENTS
	{
		result.push(vector[ELEMENTS - 1]); 
		result.extend_from_slice(&vector[1 .. ELEMENTS - 1]); 
		result.push(vector[0]);
	}
	else
	{
		result.extend_from_slice(&vector[0 .. pivot[0] - 1]);
		result.push(vector[pivot[0]]);
		result.push(vector[pivot[0] - 1]);
		result.extend_from_slice(&vector[pivot[0] + 1 .. ELEMENTS]);
	}

	return result;
}

fn changePivot(pivot : [usize; 2], ELEMENTS : usize) -> [usize; 2]
{
	if pivot[0] == ELEMENTS
	{
		return  [1, 2];
	}
	else if pivot[1] == ELEMENTS
	{
		return [ELEMENTS, 1];
	}

	return [pivot[0] + 1, pivot[1] + 1];
}