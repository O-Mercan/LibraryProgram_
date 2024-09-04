use std::borrow::Borrow;

use solana_program::{
    account_info::{self, next_account_info, Account, AccountInfo}, address_lookup_table::instruction, clock::Clock, entrypoint::ProgramResult, msg, program::invoke_signed, program_error::ProgramError, pubkey::Pubkey, rent::Rent, system_instruction, sysvar::Sysvar
};

use borsh::{BorshDeserialize, BorshSerialize};

use crate::{
    error::LibraryError::{
        ArithmeticError, 
        NotSignerPayer, 
        AuthorityError,
        InvalidOwner},

    instruction::LibraryInstruction,
    state::{
        Book, BookNumber, Configuration, Counter, User
    }
};

pub struct Processor;

impl Processor{
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8]
    ) -> ProgramResult{

        let instruction = LibraryInstruction::unpack(instruction_data)?;
        msg!("unpack called");

        match instruction {
            LibraryInstruction::CreateUser => {
                msg!("CreateUser Case");
                Self::process_create_user(program_id, accounts)
            },
            LibraryInstruction::CreateBook => {
                msg!("CreateBook Case");
                Self::process_create_book(program_id, accounts)
            },
            LibraryInstruction::AddBook{data} =>{ 
                msg!("AddBook Case");
                Self::process_add_book(program_id, accounts, data)
            },
            LibraryInstruction::RemoveBook => {
                msg!("RemoveBook Case");
                Self::process_remove_book(program_id, accounts)
            },
            LibraryInstruction::BorrowBook => {
                msg!("BorrowBook Case");
                Self::process_borrow_book(program_id, accounts)
            },
            LibraryInstruction::ReturnBook => {
                msg!("ReturnBook Case");
                Self::process_return_book(program_id, accounts)
            },
            LibraryInstruction::Config => {
                msg!("Config Case");
                Self::process_config(program_id, accounts)
            },
            LibraryInstruction::InitCounter => {
                msg!("Init Counter Case");
                Self::initialize_counter(program_id, accounts)
            },
            // LibraryInstruction::CheckAuthority { data } => {
            //     msg!("Check Authority Case");
            //     Self::check_authority(program_id, accounts, data)
            // },
        }
        
    }
    //process_create_user
    pub fn process_create_user(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
    )    -> ProgramResult {
        msg!("process_create_user");

        let account_info_iter = &mut accounts.iter();
        let user_account = next_account_info(account_info_iter)?; //user account infos
        let payer = next_account_info(account_info_iter)?;
        let user_counter = next_account_info(account_info_iter)?;

        
        let mut user_counter_info = Counter::try_from_slice(&user_counter.data.borrow())?;

        user_counter_info.counter = user_counter_info.counter.checked_add(1).ok_or(ArithmeticError)?;

        if !payer.is_signer {
            return Err(NotSignerPayer.into())
        }
     
        let user = User{
            user_no: user_counter_info.counter,
            user_address: payer.key.to_bytes(),
            book_no: 0,
            borrowed_at: 0,
            return_by: 0,
        };

        let(user_address, user_bump) = 
        Pubkey::find_program_address(&[b"UA", user_counter_info.counter.to_string().as_ref()], program_id);

        let rent = Rent::default();
        let user_address_rent = rent.minimum_balance(64);

        let ix = system_instruction::create_account(
            payer.key, 
            &user_address, 
            user_address_rent, 
            64, 
            program_id);

        invoke_signed(
            &ix,
            &[payer.clone(), user_account.clone()],
             &[&[b"UA", user_counter_info.counter.to_string().as_ref(), &[user_bump]]],
        )?;//invoke signed => pda signer

        user_counter_info.serialize(&mut &mut user_counter.data.borrow_mut()[..])?;
        user.serialize(&mut &mut user_account.data.borrow_mut()[..])?;
    
        Ok(())
    }

    //counter function 
    pub fn initialize_counter (
        program_id: &Pubkey,
        accounts: &[AccountInfo],
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let user_counter_account = next_account_info(account_info_iter)?;
        let book_counter_account = next_account_info(account_info_iter)?;
        let payer = next_account_info(account_info_iter)?;
        
        let rent = Rent::default();
        let user_counter_address_rent = rent.minimum_balance(8);
        let book_counter_address_rent = rent.minimum_balance(8);

        let(user_counter_address, user_counter_bump) = Pubkey::find_program_address(&[b"UC"], program_id);
 
        let ix = system_instruction::create_account(
            payer.key , 
            &user_counter_address, 
            user_counter_address_rent, 
            8, 
            program_id);

        invoke_signed(
            &ix, 
            &[payer.clone(), user_counter_account.clone()], 
            &[&[b"UC", &[user_counter_bump]]]
        )?;

        let(book_type_counter, book_type_counter_bump) = Pubkey::find_program_address(&[b"BC"], program_id); 

        let ix2 = system_instruction::create_account(
            payer.key, 
            &book_type_counter ,
            book_counter_address_rent, 
            8, 
            program_id
        );
        invoke_signed(  //send blockchain
            &ix2,
            &[payer.clone(), book_counter_account.clone()],
            &[&[b"BC", &[book_type_counter_bump]]]
        )?;

        Ok(())
    }


    //process_create_user
    pub fn process_create_book(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
    )    -> ProgramResult {
        msg!("process_create_book");

        let account_info_iter = &mut accounts.iter();
        let book_account = next_account_info(account_info_iter)?; //book account infos
        let payer = next_account_info(account_info_iter)?;
        let book_counter = next_account_info(account_info_iter)?;
        let config_account = next_account_info(account_info_iter)?;

        let  config =Configuration::try_from_slice(&config_account.data.borrow())?;

        let mut book_counter_info = Counter::try_from_slice(&book_counter.data.borrow())?;
        book_counter_info.counter = book_counter_info.counter.checked_sub(1).ok_or(ArithmeticError)?;

        if !payer.is_signer {
            return Err(NotSignerPayer.into())
        }

        if config_account.owner != program_id{
            msg!("config account owner is not equal program id")
        } //config account  gercekten bizim config account mi kontrolu

        Self::check_authority(
            payer.key, 
            program_id, 
            config)?;

        let book = Book{
            book_no: book_counter_info.counter,   
            number_of_books: 0,
            in_circulation: 0,
            total_number_of_books: 0
        };

        let(book_address, book_bump) = Pubkey::find_program_address(&[b"BC", book_counter_info.counter.to_string().as_ref()], program_id);

        let rent = Rent::default();
        let book_address_rent = rent.minimum_balance(32);

        let ix = system_instruction::create_account(
            payer.key, 
            &book_address,
            book_address_rent, 
            32, 
            program_id);

        invoke_signed(
            &ix, 
            &[payer.clone(), book_account.clone()], 
            &[&[b"BC", book_counter_info.counter.to_string().as_ref(), &[book_bump]]]
        )?;

        book_counter_info.serialize(&mut &mut book_counter.data.borrow_mut()[..])?;
        book.serialize(&mut &mut book_account.data.borrow_mut()[..])?;
    
        Ok(())
    }

    pub fn process_add_book(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        data: BookNumber,
    ) -> ProgramResult {
        msg!("process_add_book");

        let account_info_iter = &mut accounts.iter();
        let book_account = next_account_info(account_info_iter)?; //book account infos
        let payer = next_account_info(account_info_iter)?;
        let config_account = next_account_info(account_info_iter)?;

        let  config =Configuration::try_from_slice(&config_account.data.borrow())?;

        if !payer.is_signer {
            return Err(NotSignerPayer.into())
        }

        if book_account.owner != program_id{
            return Err(InvalidOwner.into())
        }

        if config_account.owner != program_id{
            msg!("config account owner is not equal program id")
        } //config account  gercekten bizim config account mi kontrolu

        Self::check_authority(
            payer.key, 
            program_id, 
            config)?;
     
        let mut book_data = Book::try_from_slice(&book_account.data.borrow())?;

        book_data.total_number_of_books = book_data.total_number_of_books.checked_add(data.book_number).ok_or(ArithmeticError)?;
        book_data.number_of_books = book_data.number_of_books.checked_add(data.book_number).ok_or(ArithmeticError)?;


        book_data.serialize(&mut &mut book_account.data.borrow_mut()[..])?;
    
        Ok(())
    }

    pub fn process_remove_book(
        program_id: &Pubkey,
        accounts: &[AccountInfo]
    ) -> ProgramResult {
        msg!("process_remove_book");

        let account_info_iter = &mut accounts.iter();
        let book_account = next_account_info(account_info_iter)?; //book account infos
        let payer = next_account_info(account_info_iter)?;
        //config ekle add bokk taki gibi
        if !payer.is_signer {
            return Err(NotSignerPayer.into())
        }

        if book_account.owner != program_id{panic!()}
     
        let mut book_data = Book::try_from_slice(&book_account.data.borrow())?;

        book_data.total_number_of_books = book_data.total_number_of_books.checked_add(1).ok_or(ArithmeticError)?;
        book_data.number_of_books = book_data.number_of_books.checked_sub(1).ok_or(ArithmeticError)?;

        
        book_data.serialize(&mut &mut book_account.data.borrow_mut()[..])?;
    
        Ok(())
    }

    pub fn process_borrow_book(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let book_account = next_account_info(account_info_iter)?;
        let user_account = next_account_info(account_info_iter)?;
        let payer = next_account_info(account_info_iter)?;

        
        let mut user_data = User::try_from_slice(&user_account.data.borrow())?;
        let mut book_data = Book::try_from_slice(&book_account.data.borrow())?;

       if !payer.is_signer { 
        return Err(NotSignerPayer.into())
       }

       if book_account.owner != program_id {
            return Err(InvalidOwner.into())
       }

       if user_account.owner != program_id {
            msg!("owner is not eqaul with  program id")
       }

       if user_data.user_address != payer.key.to_bytes(){
            msg!{"hata"}
       }


       let clock = Clock::get()?;
       let current_time = clock.unix_timestamp as u64;

       if book_data.number_of_books == 0 {
        msg!("Book is not available")
       }

       book_data.in_circulation = book_data.in_circulation.checked_add(1).ok_or(ArithmeticError)?;
       book_data.number_of_books =book_data.number_of_books.checked_sub(1).ok_or(ArithmeticError)?;

       book_data.serialize(&mut &mut book_account.data.borrow_mut()[..])?;

       

       user_data.book_no = book_data.book_no;
       user_data.borrowed_at = current_time;
       user_data.return_by = current_time.checked_add(1209600).ok_or(ArithmeticError)?;

       user_data.serialize(&mut &mut user_account.data.borrow_mut()[..])?;
  
        Ok(())
    }

    pub fn process_return_book(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let book_account = next_account_info(account_info_iter)?;
        let user_account = next_account_info(account_info_iter)?;
        let payer = next_account_info(account_info_iter)?;

        if !payer.is_signer {
            return Err(NotSignerPayer.into())
        }

        if book_account.owner != program_id {
            return Err(InvalidOwner.into())
        }

       let clock = Clock::get()?;
       let current_time = clock.unix_timestamp as u64;


        let mut return_book_data = Book::try_from_slice(&book_account.data.borrow())?;
        let mut return_user_data = User::try_from_slice(&user_account.data.borrow())?;

        return_book_data.in_circulation = return_book_data.in_circulation.checked_sub(1).ok_or(ArithmeticError)?;


        return_user_data.book_no = 0;   //Bunu yerine nasil bir kod yazilabilir
        return_user_data.borrowed_at = current_time.checked_sub(1209600).ok_or(ArithmeticError)?;
        return_user_data.return_by = current_time;
        

        return_book_data.serialize(&mut &mut book_account.data.borrow_mut()[..])?;
        return_user_data.serialize(&mut &mut user_account.data.borrow_mut()[..])?;

        Ok(())
    }

    pub fn process_config(
        program_id: &Pubkey,
        accounts: &[AccountInfo]
    ) -> ProgramResult{
        let account_info_iter = &mut accounts.iter();
        let config = next_account_info(account_info_iter)?;
        let payer = next_account_info(account_info_iter)?;

        let(config_address, authority_bump) = Pubkey::find_program_address(&[b"authority"], program_id);

        let rent = Rent::default();
        let authority_address_rent = rent.minimum_balance(96);

        invoke_signed (
            &system_instruction::create_account(
                payer.key, 
                &config_address, 
                authority_address_rent, 
                96, 
                program_id),
            &[payer.clone(),config.clone()],
            &[&[b"authority", &[authority_bump]]]
        )?;

        let authority_data = Configuration{
            authority_account1:payer.key.to_bytes(),
            authority_account2:payer.key.to_bytes(),
            authority_account3:payer.key.to_bytes()
        };

        authority_data.serialize(&mut &mut config.data.borrow_mut()[..])?;
        Ok(())
    }


    pub fn check_authority(
        authority: &Pubkey,
        program_id: &Pubkey,
        config: Configuration,
    ) -> ProgramResult {
      
        if config.authority_account1 != authority.to_bytes()
        && config.authority_account2 != authority.to_bytes()
        && config.authority_account3 != authority.to_bytes() {
         
          msg!("the user is the not authority");
          return Err(AuthorityError.into());
        }
        Ok(())
    }

    //updateconfig

}