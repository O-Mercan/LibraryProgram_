use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub struct Configuration{
    pub authority_account1:[u8; 32],  //authority1
    pub authority_account2:[u8; 32],  //authority2
    pub authority_account3:[u8; 32],  //authority3
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub struct User{
    pub user_no: u64,               
    pub user_address: [u8; 32], // user public key
    pub book_no: u64,            //every user can take 1 book
    pub borrowed_at: u64,       //borrow date
    pub return_by: u64         //return date
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]

pub struct Counter{
    pub counter: u64
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub struct BookNumber{
    pub book_number: u64
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub struct Book {
    pub book_no: u64,            // book name
    pub number_of_books: u64,    //number of same book
    pub in_circulation: u64,     //
    pub total_number_of_books: u64
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub struct BorrowRecord{
    book_no:u64,
    borrow_date: u64,
    pub return_date: u64
}
//her kitap olusturdugunda bir PDA olusturulacak