fn main(){
println!("{}",count("peter",'e'));
}
fn count(word: &str, search: char) -> i32 {
	// transformiert string in character array
	let letters = word.chars();
	let mut result: i32 = 0;
	// zähle Vorkommnisse des gesuchten char
	for letter in letters {
        if letter == search {
        result += 1;
        }
    }    
    return result;
	}
