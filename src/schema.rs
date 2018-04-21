table! {
    Admin (id) {
        id -> Nullable<Integer>,
        userId -> Text,
        password -> Text,
    }
}

table! {
    Candidate (id) {
        id -> Nullable<Integer>,
        name -> Text,
        electionSessionId -> Integer,
        constituencyId -> Nullable<Integer>,
    }
}

table! {
    Constituency (id) {
        id -> Nullable<Integer>,
        name -> Text,
        population -> Integer,
    }
}

table! {
    ElectionSession (id) {
        id -> Nullable<Integer>,
        name -> Text,
        isPresidential -> Bool,
    }
}

table! {
    Vote (id) {
        id -> Nullable<Integer>,
        candidateId -> Integer,
    }
}

allow_tables_to_appear_in_same_query!(
    Admin,
    Candidate,
    Constituency,
    ElectionSession,
    Vote,
);
