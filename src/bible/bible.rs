pub struct Book {
    pub name: String;
    pub abbreviation: String;
    pub chapters: Vec<chapters>;
}

pub struct Chapter {
    pub number: u16;
    pub verses: Vec<verses>;
}

pub struct Verse {
    pub number: u16;
    pub text: String;
}