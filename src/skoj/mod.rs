pub struct Skoj<Word> {

    words: Vec<WordContainer<Word>>,
    last_word: usize,

    learn_rate: f32,

}

impl<Word: PartialEq + Clone> Skoj<Word> {

    const DEFAULT_WORD_RELATION_WEIGHT: f32 = 1.0;
    const DEFAULT_WORD_RELATION: f32 = 1.0;

    const DEFAULT_WORD_FEELING: f32 = 0.0;
    const DEFAULT_WORD_FEELING_ADD: f32 = 1.0;

    pub fn new() -> Self {

        Self {
            words: vec![],
            last_word: 0,
            learn_rate: 1.0,
        }
    }

    pub fn give_word(&mut self, word: Word) {
        let word_index = self.find_word(word);
        self.words[word_index].feeling += Self::DEFAULT_WORD_FEELING_ADD;
        self.learn(self.last_word);

        self.think(word_index);
        self.forget_feelings();
        self.last_word = word_index;
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
                word.likeness.push((Self::DEFAULT_WORD_RELATION, Self::DEFAULT_WORD_RELATION_WEIGHT));
            }

            word_index = self.words.len();
            self.words.push(WordContainer {
                word,
                feeling: Self::DEFAULT_WORD_FEELING,
                likeness: vec![(Self::DEFAULT_WORD_RELATION, Self::DEFAULT_WORD_RELATION_WEIGHT); self.words.len() + 1],
            });
        }

        word_index
    }

    fn learn(&mut self, target_index: usize) {
        //let target_word = &mut self.words[target_index];

        for relation_index in 0..self.words.len() {
            self.words[target_index].likeness[relation_index].0 += self.words[relation_index].feeling / self.words[target_index].likeness[relation_index].1;
            self.words[target_index].likeness[relation_index].1 += self.learn_rate;
        }

    }

    fn forget_feelings(&mut self) {
        let multiplier = 1.0 - 1.0 / self.words.len() as f32;

        for word in &mut self.words {
            word.feeling *= multiplier;
        }
    }



    pub fn get_word(&mut self) -> &Word {
        let word_index = self.find_best_word_index();

        self.think(word_index);
        self.forget_feelings();

        self.last_word = word_index;
        &self.words[word_index].word
    }

    fn find_best_word_index(&self) -> usize {
        let mut best_word_index = 0;
        let mut highest_bar = 0.0;

        for (word_index, word) in self.words.iter().enumerate() {
            if word.feeling * word.likeness[self.last_word].0 > highest_bar && self.last_word != word_index {
                best_word_index = word_index;
                highest_bar = word.feeling;
            }
        }

        best_word_index
    }



    fn think(&mut self, last_word_index: usize) {

        for word_index in 0..self.words.len() {
            self.words[word_index].feeling += self.words[last_word_index].likeness[word_index].0;
        }

    }

}



struct WordContainer<Word> {

    word: Word,
    feeling: f32,
    likeness: Vec<(f32, f32)>,

}
