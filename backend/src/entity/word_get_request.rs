use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct WordGetRequest{
    pub word: String
}

impl WordGetRequest{
    pub fn new() -> WordGetRequestBuilder{
        WordGetRequestBuilder{
            word_get_request: WordGetRequest{
                word: String::from("")
            }
        }
    }
}

pub struct WordGetRequestBuilder{
    word_get_request: WordGetRequest
}

impl WordGetRequestBuilder{
    pub fn word(mut self, word: String) -> Self{
        self.word_get_request.word = word;
        self
    }

    pub fn build(self) -> WordGetRequest {self.word_get_request}
}