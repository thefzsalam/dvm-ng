pub struct Admin {
    pub id: u64,
    pub user_id: String,
    pub password: String
}

pub struct Candidate {
    pub id: u64,
    pub name: String,
    pub election_session_id: u64,
    pub constituency_id: u64 //ignored for presidential candidate
}

pub struct Constituency {
    pub id: u64,
    pub name: String,
    pub population: u64
}

pub struct ElectionSession {
    pub id: u64,
    pub name: String,
    pub is_presidential: bool
}

pub struct Vote {
    pub id: u64,
    pub candidate_id: u64
}
