#[derive(Debug)]
pub struct HighScores {
    pub scores_list: &'static [u32],
}

impl HighScores {
    pub fn new(scores: &'static [u32]) -> Self {
        
        HighScores{
            scores_list: &scores
        }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores_list
    }

    pub fn latest(&self) -> Option<u32> {
        // self.scores_list
        unimplemented!("Return the highest score")
    }

    pub fn personal_best(&self) -> Option<u32> {
        unimplemented!("Return the highest score")
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        unimplemented!("Return 3 highest scores")
    }
}
