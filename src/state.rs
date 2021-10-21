//state module

use borsh::{BorshDeserialize, BorshSerialize};

/*
State, part 1
- BorshDeserialize adds an associated method named try_from_slice() that we can use to construct a Mail object from a reference to a slice of u8.
- BorshSerialize provides a basic implementation of a method name serialize(). This allows us to serialize the content of the Mail object into a slice of u8
- Debug trait allows us to print the content of a mail object to the console using the `"{:?}" debug` formatter.
*/
#[derive(BorshDeserialize, BorshSerialize, Debug)]

//declared a struct that will represent our mail object
/*
State, part 1
- The from_address and to_address types hold a base58 representation of the sender's and receiver's public key.
- The subject field represents the subject of the mail
- The body field is the content of the mail
- The sent_date is the date the mail was sent. We are using the String data-type because the date we will be saving
  is going to be the string value of the specified date
*/

pub struct Mail {
    pub id: String,
    pub from_address: String,
    pub to_address: String,
    pub subject: String,
    pub body: String,
    pub sent_date: String
}

//declared a struct that will serve as the state of our user's account data
/*
- The inbox and sent fields will be vectors of the Mail object representing the lists of mails the user has in their account.
*/
#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct MailAccount {
    pub inbox: Vec<Mail>,
    pub sent: Vec<Mail>
}
