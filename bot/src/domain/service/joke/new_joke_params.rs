use regex::Regex;
use accord::{Accord, Result as AccordResult, MultipleError, MultipleInvalid};
use accord::validators::length;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NewJokeParams {
    pub subject: String,
    pub adjective: String,
    pub text: String
}

impl NewJokeParams {
    pub fn parse(possible_joke: &str) -> Option<NewJokeParams> {  
        lazy_static! {
            static ref YOU_FEMALE: Regex = Regex::new("ты такая (.+), что (.+)").unwrap();
            static ref YOU_MALE: Regex = Regex::new("ты такой (.+), что (.+)").unwrap();
            static ref GENERIC_MALE: Regex = Regex::new("твой (.+) такой (.+), что (.+)").unwrap();
            static ref GENERIC_FEMALE: Regex = Regex::new("твоя (.+) такая (.+), что (.+)").unwrap();
        }
        let lowercase_possible_joke = possible_joke.to_owned().to_lowercase();

        if let Some(captures) = YOU_FEMALE.captures(&lowercase_possible_joke)
                    .or_else(|| YOU_MALE.captures(&lowercase_possible_joke)) {
            let adjective = captures.get(1).unwrap().as_str().to_owned();
            return Some(NewJokeParams::you(adjective, possible_joke.to_owned()));
        }
        if let Some(captures) = GENERIC_MALE.captures(&lowercase_possible_joke)
                    .or_else(|| GENERIC_FEMALE.captures(&lowercase_possible_joke)) {
            let subject = captures.get(1).unwrap().as_str().to_owned();
            let adjective = captures.get(2).unwrap().as_str().to_owned();
            return Some(NewJokeParams::anyone(subject, adjective, possible_joke.to_owned()));
        }
        None
    }

    fn you(adjective: String, text: String) -> NewJokeParams {
        NewJokeParams {
            subject: "ты".to_owned(),
            adjective: adjective,
            text: text
        }
    }

    fn anyone(subject: String, adjective: String, text: String) -> NewJokeParams {
        NewJokeParams {
            subject: subject,
            adjective: adjective,
            text: text
        }
    }
}

impl Accord for NewJokeParams {
    fn validate(&self) -> AccordResult {
        rules!{
            "Объект насмешки" => self.subject => [length(2, 50)],
            "Свойство насмешки" => self.adjective => [length(2, 50)],
            "Текст шутка" => self.text => [length(10, 1000)]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correctly_creates_you_female_new_joke_params() {
        //given
        let joke = "ты такая тупая, что смеялась над мухой над коровьей лепешкой, ведь она не могла сама навалить такую кучу.";
        //when
        let maybe_new_joke = NewJokeParams::parse(joke);
        //then
        assert_eq!(
            maybe_new_joke, 
            Some(NewJokeParams::you(
                "тупая".to_owned(), 
                joke.to_owned()
            ))
        );
    }

    #[test]
    fn correctly_creates_you_male_new_joke_params() {
        //given
        let joke = "ты такой старый, что когда ты родился, мертвое море только-только заболело.";
        //when
        let maybe_new_joke = NewJokeParams::parse(joke);
        //then
        assert_eq!(
            maybe_new_joke, 
            Some(NewJokeParams::you(
                "старый".to_owned(), 
                joke.to_owned()
            ))
        );
    }

    #[test]
    fn correctly_creates_generic_male_new_joke_params() {
        //given
        let joke = "твой кошелек такой пустой, что депутаты рады хотели использовать его в качестве бюджета на следующий год.";
        //when
        let maybe_new_joke = NewJokeParams::parse(joke);
        //then
        assert_eq!(
            maybe_new_joke, 
            Some(NewJokeParams::anyone(
                "кошелек".to_owned(),
                "пустой".to_owned(), 
                joke.to_owned()
            ))
        );
    }

    #[test]
    fn correctly_creates_generic_female_new_joke_params() {
        //given
        let joke = "твоя игра такая инди, что даже студентки художницы отказываются рисовать для нее";
        //when
        let maybe_new_joke = NewJokeParams::parse(joke);
        //then
        assert_eq!(
            maybe_new_joke, 
            Some(NewJokeParams::anyone(
                "игра".to_owned(),
                "инди".to_owned(), 
                joke.to_owned()
            ))
        );
    }
}
