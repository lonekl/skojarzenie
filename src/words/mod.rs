pub struct WordList<Word> {

    words: Vec<WordContainer<Word>>,

}

impl<Word: PartialEq> WordList<Word> {

    const DEFAULT_WORD_RELATION: f32 = 0.0;

    pub fn new() -> Self {

        Self {
            words: vec![],
        }
    }

    pub fn give_word(&mut self, word: Word) {
        let word_index = self.find_word(word);

        // TODO Relations and stuff.

        self.forget_feelings();
    }

    fn find_word(&mut self, word: Word) -> usize {
        let mut word_index = usize::MAX;
        for (listed_word_index, listed_word) in self.words.iter().enumerate() {
            if listed_word.word == word {
                word_index = listed_word_index;
            }
        }

        if word_index == usize::MAX {
            for word in &mut self.words {
                word.relations.push(Self::DEFAULT_WORD_RELATION);
            }

            word_index = self.words.len();
            self.words.push(WordContainer {
                word,
                feeling: 0.0,
                relations: vec![Self::DEFAULT_WORD_RELATION; self.words.len() + 1],
            });
        }

        word_index
    }

    fn forget_feelings(&mut self) {
        let subtractor = 1.0 / self.words.len() as f32;

        for word in &mut self.words {
            word.feeling -= subtractor;
        }
    }

}



struct WordContainer<Word> {

    word: Word,
    feeling: f32,
    relations: Vec<f32>,

}
